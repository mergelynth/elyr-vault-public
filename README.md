# Elyr Vault

**Privacy-first encrypted vault protocol** for locking crypto assets and secret data on-chain using Fully Homomorphic Encryption (FHE).

Users create vaults that hold ETH, ERC-20 tokens, or arbitrary secret data — encrypted per-field at the client level before any on-chain transaction. Decryption is only possible by authorized participants via attested co-validator signatures.

> **Status:** Work in Progress / Alpha Testnet  
> **Network:** Base Sepolia (EVM) · Solana Devnet  
> **Contract version:** v3.15.0 (UUPS upgradeable proxy)

---

## What It Does

Elyr Vault is a non-custodial protocol for creating time-locked, condition-gated encrypted vaults on public blockchains.

- **Lock** — deposit ETH, ERC-20, or secret text into a vault with configurable unlock conditions
- **Encrypt** — every sensitive field (recipient, amount, fallback, name, conditions, secret content) can be independently FHE-encrypted before reaching the chain
- **Unlock** — vaults release automatically when on-chain conditions are met (time, inactivity, balance threshold, incoming transaction)
- **Claim** — authorized recipients decrypt and claim assets via wallet signature; no third party ever sees plaintext

All encryption and decryption happens client-side. The smart contract stores only FHE ciphertext (`euint256` handles). The blockchain never sees raw values for encrypted fields.

---

## Vault Types

| Type | Description |
|------|-------------|
| **Asset** | Lock ETH or ERC-20 tokens with optional encrypted amounts and recipients |
| **Secret** | Store encrypted text/data on-chain without any token transfer |
| **Hybrid** | Lock assets AND attach encrypted secret data in a single vault |

---

## Unlock Conditions

Multiple conditions can be combined per vault:

| Condition | Trigger |
|-----------|---------|
| **Release at Date** | Unlocks at a specific UTC timestamp |
| **Inactivity Timer** | Unlocks after creator inactivity for N days (dead man's switch) |
| **Balance Below** | Unlocks when a monitored address's token balance drops below a threshold |
| **Incoming Transaction** | Unlocks when a monitored address receives specific tokens |

Conditions support a configurable claim period and fallback behavior (return to creator, send to backup address, or keep locked).

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

Privacy flags are packed into a `uint8` bitmask — the contract never sees plaintext for encrypted fields.

---

## Participant Roles

| Role | Capabilities |
|------|-------------|
| **Owner** | Creates the vault, can refund after deadline, monitors lifecycle |
| **Recipient** | Claims assets/secrets when conditions are met (single or multi-recipient) |
| **Observer** | Read-only access with configurable per-field permission bitmap |
| **Fallback** | Receives refund if the vault expires unclaimed |

Multi-recipient vaults (v3.14+) support `first-come` or `equal-split` claim distribution modes.

---

## Encryption Specification

- **Client-side encryption** — FHE SDK encrypts data in the browser before any blockchain call
- **On-chain storage** — contract converts encrypted bytes → `euint256` FHE handles
- **Attested decryption** — `attestedDecrypt(walletClient, handles)` with co-validator signatures
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

### Deployment (Base Sepolia)

| Field | Value |
|-------|-------|
| Proxy | `0x8fac69E9D45b1e312ED58218a3dE3a317840AdaF` |
| Implementation | `0xb904137B334440ab66B3126FC13e53481621F3d9` |
| Chain ID | `84532` |
| Version | `3.15.0` |

### Solana — inco-vault

Anchor program with FHE Lightning integration. Encrypted vault accounts, SOL/SPL token deposits, claim/refund lifecycle.

| Field | Value |
|-------|-------|
| Program ID | `8kKZoqm42xJtu1JWvH1ZeoLsucVyUKpGfmhrY2eBHjBK` |
| Network | Solana Devnet |

---

## Tech Stack

| Layer | Technology |
|-------|------------|
| Smart Contracts (EVM) | Solidity 0.8.30, Hardhat, `@inco/lightning` |
| Smart Contracts (Solana) | Rust, Anchor, `@inco/solana-sdk` |
| Frontend | Next.js 14, React 18, TypeScript |
| Styling | Tailwind CSS, Framer Motion |
| Wallet (EVM) | RainbowKit, wagmi, viem |
| Wallet (Solana) | `@solana/wallet-adapter` |
| FHE SDK | `@inco/js` |
| Backend | NestJS, Prisma, PostgreSQL |
| Monorepo | npm workspaces, Turborepo |

---

## Networks

| Network | Type | Status | Currency | FHE Fee/field |
|---------|------|--------|----------|---------------|
| Base Sepolia | EVM | Active (default) | ETH | 0.001 ETH |
| Solana Devnet | Solana | Active | SOL | 0.005 SOL |

---

## Connect

To interact with the protocol:

1. Connect an EVM wallet (MetaMask, Rainbow, etc.) or Solana wallet (Phantom, Solflare)
2. Switch to Base Sepolia or Solana Devnet
3. Get testnet tokens from the built-in faucet (Settings page)
4. Create a vault via the Deploy wizard or call the contract directly

---

## Disclaimer

This software is in **alpha stage** and deployed on **testnets only**. Smart contracts have not been formally audited. Use at your own risk — do not deposit real assets. The protocol, API, and contract interfaces may change without notice before mainnet release.

---

## License

Proprietary software. All rights reserved. No use without permission.

---

Built with [Inco Network](https://inco.org) FHE
