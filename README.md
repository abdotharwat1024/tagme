# **TagMe – Solana-Based Product Authenticity & Ownership Registry**

TagMe is a decentralized product authenticity platform built on **Solana**, enabling manufacturers and owners to bind physical products to cryptographic identities secured by NFC hardware chips.

Each product contains a secure NFC tag capable of generating an **ECC keypair**.
The **public key** is registered on-chain, while the **private key** never leaves the chip.

Users verify authenticity with a simple **phone tap** — no apps, no accounts, no trust required.

---

## **✨ Features**

### 🔐 Cryptographic Product Identity

Each product contains a secure NFC chip with its own ECC keypair, enabling tamper-proof authentication.

### 📦 On-Chain Product Registry

Every product is stored on Solana with:

* Immutable **author**
* Mutable **owner**
* Metadata hash
* Lifecycle state
* Creation timestamp

### 🔄 Ownership Transfers

Owners can securely transfer products on-chain without trusted intermediaries.

### 🏛 Immutable Authorship

Once registered, the **author_pubkey** cannot be changed — eliminating counterfeit re-registration attacks.

### 🔍 Client-Side Verification

Verification works by:

1. Sending a challenge to the NFC tag
2. Validating the signature with the product’s public key
3. Fetching product data from Solana
4. Ensuring the product is ACTIVE and authentic

### 📚 Optional Ownership History

Products can optionally store their entire ownership timeline on-chain.

---

## **📐 Architecture Overview**

TagMe is implemented as a Solana Anchor program and is composed of:

* **UserAccount** – represents both authors and owners
* **ProductRegistryAccount** – main product identity record
* **OwnershipHistoryAccount** – optional ownership history
* **Instructions** – the program’s allowed actions

---

## **🗂 On-Chain Accounts**

### **UserAccount**

Holds information about an author or product owner.

| Field         | Type     |
| ------------- | -------- |
| `user_pubkey` | `Pubkey` |
| `name`        | `String` |
| `url`         | `String` |
| `bump`        | `u8`     |

---

### **ProductRegistryAccount**

Main product identity stored on-chain.

| Field            | Type            | Notes             |
| ---------------- | --------------- | ----------------- |
| `product_pubkey` | `Pubkey`        | Immutable         |
| `author_pubkey`  | `Pubkey`        | Immutable         |
| `owner_pubkey`   | `Pubkey`        | Mutable           |
| `metadata_hash`  | `[u8; 32]`      | IPFS/Arweave hash |
| `created_at`     | `u64`           | Timestamp         |
| `status`         | `ProductStatus` | ACTIVE / REVOKED  |
| `bump`           | `u8`            | PDA bump          |

---

### **OwnershipHistoryAccount** (Optional)

| Field              | Type          |
| ------------------ | ------------- |
| `product_pubkey`   | `Pubkey`      |
| `previous_owners`  | `[Pubkey; 5]` |
| `last_transfer_at` | `u64`         |
| `bump`             | `u8`          |

---

## **🔧 Instructions (Anchor Methods)**

### `register_user()`

Creates a new `UserAccount`.

### `register_product()`

Registers a new product with immutable author and mutable owner.

### `transfer_ownership()`

Transfers ownership from current owner to new owner and updates history.

### `update_product_metadata()`

Allows authors to modify product metadata (e.g., new IPFS hash).

### `revoke_product()`

Permanently disables a product.
**Revocation is terminal** — a product cannot become ACTIVE again.

---

## **🔁 System Workflows**

### **1. Provisioning / Manufacturing**

1. NFC chip generates ECC keypair
2. Author registers a `UserAccount`
3. Author registers the product on-chain
4. Product enters ACTIVE state

---

### **2. Verification (Tap-to-Verify)**

1. User taps product with phone
2. App sends random challenge to NFC chip
3. Chip signs with private key
4. App verifies signature matches `product_pubkey`
5. App fetches on-chain product record
6. App checks:

   * signature validity
   * `status == ACTIVE`
   * expected author
   * expected owner (optional)

---

### **3. Ownership Transfer**

1. Current owner signs transfer instruction
2. Program ensures:

   * signer is current owner
   * product is ACTIVE
3. Updates `owner_pubkey`
4. Appends to ownership history (optional)

---

## **🧭 Product Lifecycle**

```
[*] --> Unregistered
Unregistered --> Registered : register_product()
Registered --> Active
Active --> Active : transfer_ownership()
Active --> Active : update_product_metadata()
Active --> Revoked : revoke_product()
Revoked --> [*]
```

Lifecycle is simple, secure, and final.

---

## **⚡ Why Solana?**

* Ultra-low fees for frequent interactions
* High throughput for real-time NFC taps
* PDA-based identity system perfectly fits product accounts
* Fast confirmation time for seamless user experience

---

## **🚀 Future Enhancements**

* Hardware-based device attestation
* Batch provisioning for manufacturers
* IPFS/Arweave metadata standardization
* Multi-signature author teams
* Support for multiple NFC chip families
* Manufacturer dashboards & analytics

---

## **🧪 Development**

### Build

```bash
anchor build
```

### Test

```bash
anchor test
```

### Deploy

```bash
solana program deploy target/deploy/tagme.so
```

---

## **📄 License**

Apache 2.0
