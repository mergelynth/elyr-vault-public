# Usage Guide

How to interact with Elyr Vault — creating vaults, claiming assets, and understanding the protocol.

---

## Overview

Elyr Vault is a privacy-first protocol for creating encrypted, condition-gated vaults on-chain. All sensitive data is encrypted in your browser using Fully Homomorphic Encryption (FHE) before being sent to the blockchain. The smart contract only stores ciphertext — decryption requires your wallet signature.

---

## Supported Networks

| Network | Type | Currency | Status |
|---------|------|----------|--------|
| **Base Sepolia** | EVM (Ethereum L2 testnet) | ETH | Active (default) |
| **Solana Devnet** | Solana | SOL | Active |

Both networks use the Inco FHE coprocessor for on-chain encryption.

---

## Getting Started

### 1. Connect Your Wallet

Connect an EVM wallet (MetaMask, Rainbow, Coinbase Wallet, etc.) or a Solana wallet (Phantom, Solflare).

- EVM: switch your wallet to **Base Sepolia** (chain ID `84532`)
- Solana: switch to **Devnet**

### 2. Get Testnet Tokens

Navigate to **Settings** and use the built-in faucet. Cooldown: 24 hours per wallet per network.

### 3. Create a Vault

Go to **Deploy** to open the 5-step creation wizard.

### 4. Monitor & Manage

Use **Dashboard** for an overview, **Vaults** for your created vaults, and **Inbox** for vaults where you're a recipient.

---

## Vault Types

### Asset Vault
Lock ETH or ERC-20 tokens with optional encryption of the amount, recipient, and other fields. Tokens are held by the smart contract until unlock conditions are met.

### Secret Vault
Store encrypted text/data on-chain without transferring any tokens. Useful for private messages, credentials, notes, or any data you want to make available under specific conditions.

### Hybrid Vault
Combines both — lock tokens AND attach encrypted secret data in a single vault.

---

## Creating a Vault (5-Step Wizard)

### Step 1 — Identity & Visibility
- Set a vault name
- Choose vault type (Asset, Secret, or Hybrid)
- Toggle per-field privacy options (which fields get FHE-encrypted)

### Step 2 — Data
- **Asset vaults:** select token (ETH or ERC-20), enter amount. Optionally use Confidential ERC-20 (cERC-20) for shielded token deposits.
- **Secret vaults:** enter secret text content. Large secrets are automatically chunked into 32-byte `euint256` segments.
- **Hybrid vaults:** both of the above.

### Step 3 — Participants
- Add one or more **recipients** (addresses that can claim)
- Set a **fallback** address for refunds if the vault expires unclaimed
- Optionally add **observers** with configurable per-field read permissions
- For multi-recipient vaults: choose claim distribution (`first-come` or `equal-split`) and data access mode

### Step 4 — Unlock Conditions
Add one or more conditions (combined per vault):

| Condition | Configuration |
|-----------|--------------|
| **Release at Date** | Pick a UTC date/time. Vault unlocks at that timestamp. |
| **Inactivity Timer** | Set a duration in days. Vault unlocks if the creator does not interact for that period (dead man's switch). |
| **Balance Below** | Set a token, monitoring address, and threshold. Vault unlocks when the monitored balance drops below the threshold. |
| **Incoming Transaction** | Set a token and monitoring address. Vault unlocks when the address receives that token. |

You also configure:
- **Deadline** — after this time, the vault can be refunded if unclaimed
- **Claim period fallback** — what happens after deadline (return to creator, send to backup, keep locked)

### Step 5 — Review & Deploy
Review all settings, check the gas estimate and FHE fee breakdown, then submit the transaction.

- The system panel shows real-time gas cost estimation
- FHE storage cost is calculated per encrypted field (Base Sepolia: 0.001 ETH/field, Solana Devnet: 0.005 SOL/field)
- Risk assessment shows a score based on privacy flags, condition complexity, and participant configuration

After deployment, you receive a vault ID and a shareable link.

---

## Claiming a Vault

When you are a recipient of a vault and the unlock conditions are met:

1. **Navigate** to the vault via your Inbox or a shared link
2. **Verify identity** — the contract checks that your connected wallet matches the vault's recipient (or commit-reveal hash)
3. **Decrypt** — click "Decrypt" to reveal encrypted fields. This triggers an `attestedDecrypt` call that requires your wallet signature. Co-validators verify authorization and return decrypted values.
4. **Claim** — submit a claim transaction to transfer assets to your wallet
5. **Receive** — ETH/tokens are transferred directly to your address

For multi-recipient vaults:
- **First-come mode:** first recipient to claim receives the full amount
- **Equal-split mode:** each recipient claims their proportional share

---

## Recipient Verification

Vaults use a commit-reveal pattern for recipient verification:

1. At creation, the creator commits a `keccak256(recipientAddress)` hash
2. At claim time, the recipient proves ownership by signing with their wallet
3. The contract verifies the hash matches and allows the claim

For encrypted recipients, FHE-based verification is used: the contract computes an `ebool` handle comparing the caller's address to the encrypted recipient.

---

## Privacy Options

Each field can be independently encrypted:

| Field | Effect when encrypted |
|-------|--------------------|
| **Recipient** | Destination address hidden from public blockchain explorers |
| **Amount** | Deposited value is not visible on-chain |
| **Fallback** | Refund address is hidden |
| **Name** | Vault title only visible to authorized participants |
| **Conditions** | Unlock parameters (timestamps, thresholds) are encrypted |
| **Secret** | Text content stored as encrypted `euint256` chunks |

Privacy flags are packed into a `uint8` bitmask in the smart contract.

---

## Fee Structure

Total cost = **gas fee** + **FHE encryption fee**

- **FHE fee** is charged per encrypted field:
  - Base Sepolia: **0.001 ETH** per field
  - Solana Devnet: **0.005 SOL** per field
- Secret content with N chunks counts as N encrypted fields
- Confidential ERC-20 (cERC-20) deposits may require additional FHE fees for the shield operation
- Minimum fee: 1 encrypted field equivalent (even for vaults with no encrypted fields)

The Deploy wizard shows a real-time fee breakdown before you submit.

---

## Risk Assessment

The system calculates a risk score (RSK 1–5) based on:

| Factor | What it measures |
|--------|-----------------|
| **Participants** | Number of recipients, observer access breadth |
| **Visibility** | How many fields are publicly visible vs encrypted |
| **Unlock complexity** | Number and type of combined conditions |
| **Asset value** | Deposit size relative to FHE fees |
| **Privacy coverage** | Percentage of fields that are encrypted |

Higher risk scores indicate vaults with more public exposure or complex configurations.

---

## Activity & Audit Log

Every vault action is recorded on-chain and indexed:

- **Created** — vault deployed with configuration hash
- **Claimed** — recipient claimed assets/secrets
- **Refunded** — creator or fallback received expired assets
- **Deposit** — additional tokens sent to vault for condition triggers

View the full timeline on the **Activity** page, filterable by address.

---

## Interacting via Contract

You can call the vault contract directly (without the UI):

### Contract Addresses

| Network | Proxy Address |
|---------|--------------|
| Base Sepolia | `0x8fac69E9D45b1e312ED58218a3dE3a317840AdaF` |

### Basic Contract Call (ethers.js example)

```js
import { ethers } from "ethers";
import abi from "./PrivateVault.abi.json";

const VAULT_ADDRESS = "0x8fac69E9D45b1e312ED58218a3dE3a317840AdaF";
const provider = new ethers.BrowserProvider(window.ethereum);
const signer = await provider.getSigner();
const vault = new ethers.Contract(VAULT_ADDRESS, abi, signer);

// Read vault details
const vaultData = await vault.getVault(vaultId);
console.log("Status:", vaultData.status);
console.log("Created at:", vaultData.createdAt.toString());

// Check version
const version = await vault.version();
console.log("Contract version:", version); // "3.15.0"
```

### Reading Vault Count

```js
const count = await vault.getVaultCount();
console.log("Total vaults:", count.toString());
```

### Checking Conditions

```js
const conditionsMet = await vault.areConditionsMet(vaultId);
console.log("Unlockable:", conditionsMet);
```

> **Note:** Creating vaults with encrypted fields requires the Inco FHE SDK (`@inco/js`) for client-side encryption. See the ABI for full function signatures.

---

## Disclaimer

This protocol is in **alpha** and running on **testnets only**. Contracts have not been formally audited. Do not use with real assets. Interfaces may change before mainnet release.
