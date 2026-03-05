# Elyr Vault

**Privacy-first encrypted vault protocol** for locking crypto assets and secret data on-chain using Fully Homomorphic Encryption (FHE).

Users create vaults that hold ETH, ERC-20 tokens, SOL, SPL tokens, or arbitrary secret data — encrypted per-field at the client level before any on-chain transaction. Decryption is only possible by authorized participants via attested co-validator signatures. The protocol is multi-chain: EVM (Base Sepolia) and Solana (Devnet), both powered by the Inco Lightning coprocessor.

> **Status:** Work in Progress / Alpha Testnet  
> **Networks:** Base Sepolia (EVM) · Solana Devnet  
> **EVM Contract:** v3.15.0 (UUPS upgradeable proxy)  
> **Solana Program:** Anchor + Inco Lightning

---

## What It Does

Elyr Vault is a non-custodial protocol for creating time-locked, condition-gated encrypted vaults on public blockchains. It works on both EVM and Solana networks simultaneously.

- **Lock** — deposit ETH, ERC-20, SOL, SPL tokens, or secret text into a vault with configurable unlock conditions
- **Encrypt** — every sensitive field (recipient, amount, fallback, name, conditions, secret content) can be independently FHE-encrypted before reaching the chain
- **Unlock** — vaults release when on-chain conditions are met (time, inactivity via protocol calls, balance threshold, cumulative deposits via protocol)
- **Claim** — authorized recipients decrypt and claim assets via wallet signature; no third party ever sees plaintext
- **Multi-chain** — unified experience across EVM and Solana with automatic network-aware fee calculation, explorer links, and wallet management

All encryption and decryption happens client-side. The smart contracts store only FHE ciphertext (`euint256` handles). The blockchain never sees raw values for encrypted fields.

---

## Use Cases

- **Crypto inheritance** — set up a dead man's switch that releases assets to heirs after a period of inactivity
- **Time-locked savings** — lock assets until a future date with no way to withdraw early
- **Conditional payments** — release funds when on-chain conditions are met (balance drops, tokens received)
- **Private messaging** — store encrypted messages on-chain, readable only by the intended recipient
- **Escrow** — lock assets with multi-party access and configurable claim rules
- **Secret sharing** — distribute encrypted credentials, keys, or notes to specific wallet addresses
- **DAO treasury vesting** — time-lock team allocations with per-field privacy

---

## Vault Types

| Type | Description |
|------|-------------|
| **Asset** | Lock ETH/ERC-20 (EVM) or SOL/SPL (Solana) with optional encrypted amounts and recipients |
| **Secret** | Store encrypted text/data on-chain without any token transfer |
| **Hybrid** | Lock assets AND attach encrypted secret data in a single vault |

---

## Unlock Conditions

Multiple conditions can be combined per vault:

| Condition | Trigger |
|-----------|---------|
| **Release at Date** | Unlocks at a specific UTC timestamp |
| **Inactivity Timer** | Unlocks after creator inactivity for N days (dead man's switch). Only interactions with the Elyr Vault contract count as activity — regular network transactions do not reset the timer |
| **Balance Below** | Unlocks when a monitored address's token balance drops below a threshold (reads live on-chain balance at claim time — no protocol interaction required) |
| **Incoming Transaction** | Unlocks when cumulative deposits sent through the Elyr Vault contract reach the required threshold. Regular token transfers do not count — only deposits routed through the protocol |

Conditions support a configurable claim period and fallback behavior (return to creator, send to backup address, or keep locked). When conditions are encrypted, a commit-reveal pattern ensures values remain private until claim time.

---

## Per-Field Privacy

Each vault field can be independently toggled for FHE encryption, stored as `euint256` ciphertext on-chain:

| Field | What gets encrypted |
|-------|-------------------|
| Recipient | Destination address hidden from public view |
| Amount | Deposit value encrypted |
| Fallback | Refund/backup address encrypted |
| Name | Vault title visible only to participants |
| Token | Token contract address masked |
| Conditions | Unlock parameters encrypted (commit-reveal pattern) |

Privacy flags are packed into a `uint8` bitmask — the contract never sees plaintext for encrypted fields. Users choose their privacy level: from fully public vaults (no encryption, gas-only cost) to fully private (all fields encrypted).

---

## Participant Roles

| Role | Capabilities |
|------|-------------|
| **Owner** | Creates the vault, can refund after deadline, monitors lifecycle |
| **Recipient** | Claims assets/secrets when conditions are met (single or multi-recipient) |
| **Observer** | Read-only access with configurable per-field permission bitmap |
| **Fallback** | Receives refund if the vault expires unclaimed |

Multi-recipient vaults (v3.14+) support `first-come` or `equal-split` claim distribution modes with per-recipient claim tracking.

---

## Encryption Specification

- **Client-side encryption** — Inco Lightning SDK encrypts data in the browser before any blockchain call
- **On-chain storage** — contract converts encrypted bytes → `euint256` FHE handles
- **TEE-backed attested decryption** — `attestedDecrypt(walletClient, handles)` with TEE co-validator signatures
- **Secret chunking** — secrets > 32 bytes are split into `euint256` chunks (32 bytes each), independently encrypted
- **Batch decryption** — single wallet signature for all vault handles, with per-handle fallback
- **Commit-reveal** — recipient verification via `keccak256(address)` hash

### FHE Handle Types

| Type | Bits | Use Case |
|------|------|----------|
| `euint256` | 256 | Addresses, amounts, secrets, hashes |
| `euint64` | 64 | Token amounts (wei) |
| `ebool` | 1 | Boolean flags, recipient match results |
| `ebytes256` | 2048 | Large binary data |

---

## Smart Contracts

### EVM — PrivateVaultV3

UUPS upgradeable proxy on Base Sepolia. Modular architecture with 7 base modules:

| Module | Responsibility |
|--------|---------------|
| `VaultStorage` | Storage layout — FHE encrypted fields, mappings, events, errors |
| `VaultCreation` | `createAssetVaultETH`, `createAssetVaultERC20`, `createSecretVault` |
| `VaultActions` | `claim`, `refund`, `cancelVault` |
| `VaultConditions` | Unlock condition evaluation (4 types) |
| `VaultTransfers` | ETH/ERC-20 transfer helpers |
| `VaultRecipientVerify` | Commit-reveal recipient verification |
| `VaultViews` | Read-only getter functions |

**Libraries:** `ConditionsLib` (condition encoding/evaluation), `ActivitySigLib` (EIP-712 activity signatures)

**Key events:** `VaultCreated`, `VaultClaimed`, `VaultRefunded`, `ConditionDeposit`, `RecipientVerificationRequested`, `ObserverAdded`

#### Deployment (Base Sepolia)

| Field | Value |
|-------|-------|
| Proxy | `0x8fac69E9D45b1e312ED58218a3dE3a317840AdaF` |
| Implementation | `0xb904137B334440ab66B3126FC13e53481621F3d9` |
| Chain ID | `84532` |
| Version | `3.15.0` |

### Solana — Elyr Vault Program

Anchor program with Inco Lightning integration. Encrypted vault accounts, SOL/SPL token deposits, claim/refund lifecycle. Mirrors the EVM vault logic with Solana-native account model.

#### Deployment (Solana Devnet)

| Field | Value |
|-------|-------|
| Program ID | `8kKZoqm42xJtu1JWvH1ZeoLsucVyUKpGfmhrY2eBHjBK` |
| Network | Solana Devnet |

---

## Tech Stack

| Layer | Technology |
|-------|------------|
| Smart Contracts (EVM) | Solidity 0.8.30, Hardhat |
| Smart Contracts (Solana) | Rust, Anchor |
| Frontend | Next.js 14, React 18, TypeScript |
| Styling | Tailwind CSS, Framer Motion |
| Wallet (EVM) | RainbowKit, wagmi, viem |
| Wallet (Solana) | `@solana/wallet-adapter` |
| Encryption | Inco Lightning (FHE coprocessor, TEE-backed) |
| Backend | NestJS, Prisma, PostgreSQL |
| Monorepo | npm workspaces, Turborepo |

---

## Networks

| Network | Type | Status | Currency | FHE Fee/field |
|---------|------|--------|----------|---------------|
| Base Sepolia | EVM (L2 testnet) | Active (default) | ETH | 0.001 ETH |
| Solana Devnet | Solana | Active | SOL | 0.005 SOL |
| Base Mainnet | EVM | Coming soon | ETH | TBD |
| Solana Mainnet | Solana | Coming soon | SOL | TBD |

---

## Connect

To interact with the protocol:

1. Connect an EVM wallet (MetaMask, Rainbow, Coinbase Wallet, etc.) or Solana wallet (Phantom, Solflare)
2. Switch to **Base Sepolia** (chain ID `84532`) for EVM or **Solana Devnet** for Solana
3. Get testnet tokens from the built-in faucet (Settings page, 24h cooldown per wallet)
4. Create a vault via the Deploy wizard or call the contract directly

---

## Roadmap

### Responsive & Adaptive Design

- Full mobile-first responsive layout for all pages (dashboard, deploy wizard, vault detail, inbox)
- Touch-optimized interactions for mobile wallet browsers (MetaMask Mobile, Rainbow, Phantom)
- Adaptive navigation (bottom sheet nav on mobile, floating rail on desktop)

### Progressive Web App (PWA)

- Installable on mobile and desktop — add to home screen with native app experience
- Offline-capable shell with service worker caching (static assets, app shell, fonts)
- Push notifications for vault events (unlock, claim, refund, expiry warning)
- Background sync for pending transactions and draft vault persistence
- Web App Manifest with custom icons, splash screens, and standalone display mode

### Coming Soon Features

| Feature | Description | Status |
|---------|-------------|--------|
| **File Vault** | Encrypt and store files (up to 5 MB) in vaults — files are FHE-encrypted client-side before on-chain storage. Supports documents, images, credentials | Planned |
| **Shared Admin Control** | Multi-signature vault management — multiple admins can approve vault operations (claim, refund, cancel). Requires smart contract upgrade to support multi-sig authorization | Planned |
| **Admin Participant Role** | New participant role with full vault management permissions, beyond owner/recipient/observer | Planned |
| **Custom Claim Shares** | Manual allocation of claim percentages per recipient (beyond first-come and equal-split) | Planned |
| **Custom Logic Conditions** | Programmable unlock conditions via user-defined on-chain logic | Planned |
| **Ukrainian Language** | Full UI localization in Ukrainian (translations exist, activation pending) | Planned |
| **Base Mainnet** | Production deployment on Base L2 mainnet | Planned |
| **Solana Mainnet** | Production deployment on Solana mainnet | Planned |

### Condition Monitoring via External Protocols

Currently, the **Inactivity Timer** and **Incoming Transaction** conditions rely on direct interaction with the Elyr Vault smart contract. Only the **Balance Below** condition reads live on-chain state directly.

Planned integrations to enable passive, protocol-independent condition monitoring:

| Integration | Use Case |
|-------------|----------|
| **Chainlink Automation** (Keepers) | Automated condition checks and vault unlocks without manual claims — time-based triggers, balance monitoring via Chainlink Data Feeds |
| **The Graph** | Indexed subgraph for real-time vault event querying, activity tracking, and condition state without polling RPC nodes |
| **Chainlink Functions** | Off-chain computation for complex condition evaluation (e.g., cross-chain balance checks, API-driven triggers) |
| **Gelato Network** | Gasless automated execution of vault operations (auto-claim, auto-refund on deadline) |
| **OpenZeppelin Defender** | Automated monitoring and incident response for vault conditions, with relay-based meta-transactions |

These integrations will allow the **Inactivity** and **Incoming Transaction** conditions to be monitored passively — without requiring users to interact directly with the protocol contract — and will enable truly autonomous vault lifecycle management.

---

## Disclaimer

This software is in **alpha stage** and deployed on **testnets only**. Smart contracts have not been formally audited. Use at your own risk — do not deposit real assets. The protocol, API, and contract interfaces may change without notice before mainnet release.

---

## License

Proprietary software. All rights reserved. No use without permission.

---

Built with [Inco Lightning](https://inco.org) FHE
