TagMe — Solana-Based Product Authenticity & Ownership Registry

TagMe is a decentralized product authenticity system built on Solana, designed to connect physical products to cryptographic identities stored on-chain.

Each product contains an NFC tag capable of generating an ECC keypair.
The public key is registered on Solana, while the private key stays inside the NFC chip forever.
Users can verify authenticity by simply tapping the tag with their mobile device.

TagMe enables:

Product authenticity verification

Ownership tracking

Anti-counterfeit protection

Tamper-proof provenance

Trustless transfer of ownership

Optional ownership history

## Features
🔐 Cryptographic Product Identity

Every product contains a secure NFC chip with its own ECC keypair.

📦 On-Chain Product Registry

Each product is linked to:

A permanent author

A current owner

Its metadata hash

Its lifecycle state

🔄 Ownership Transfers

Owners can transfer products securely on-chain.

🏛 Immutable Authorship

The author of a product can never be changed, preventing re-registration by counterfeiters.

🔍 Client-Side Verification

The app verifies authenticity by:

Sending a challenge to the NFC chip

Checking the cryptographic signature

Fetching the product record from the blockchain

📚 Optional Ownership History

Products can store a full list of previous owners.

## Architecture Overview

TagMe is built using Solana Anchor and consists of these core components:

UserAccount — represents authors and owners

ProductRegistryAccount — main product record

OwnershipHistoryAccount — optional owner trace

ProgramState — global admin/config state

Instructions — the actions allowed by the program

Below is the final UML class diagram the system is based on:

UserAccount "1" --> "*" ProductRegistryAccount : author of >
UserAccount "1" --> "*" ProductRegistryAccount : owns >
ProductRegistryAccount "1" --> "0..1" OwnershipHistoryAccount : history >
ProgramState --> Instructions : controls >

## On-Chain Accounts
### UserAccount

Represents any user in the system (authors and owners share the same type).

user_pubkey : Pubkey
name        : String
url         : String
bump        : u8

### ProductRegistryAccount

The main product record on-chain.

product_pubkey : Pubkey     (immutable)
author_pubkey  : Pubkey     (immutable)
owner_pubkey   : Pubkey     (mutable)
metadata_hash  : [u8; 32]
created_at     : u64
status         : ProductStatus
bump           : u8

### OwnershipHistoryAccount (optional)
product_pubkey  : Pubkey
previous_owners : Vec<Pubkey>
last_transfer_at : u64
bump            : u8

### ProgramState
admin_pubkey : Pubkey
version      : u8

## Enum: ProductStatus
ACTIVE
REVOKED


States are terminal:
Once revoked, a product cannot return to ACTIVE.

## Instructions (Anchor Methods)
register_user()

Creates a new UserAccount.

register_product()

Registers a new product with immutable author and mutable owner.

transfer_ownership()

Updates owner_pubkey and appends to OwnershipHistoryAccount.

update_product_metadata()

Allows authors to update product metadata.

revoke_product()

Permanently disables a product.

## System Flows
### 🔧 Provisioning / Manufacturing Workflow

NFC chip generates ECC keypair

Author registers user account (UserAccount)

Author registers product (ProductRegistryAccount)

Product becomes ACTIVE on-chain

### 📱 Verification Workflow (Phone Tap)

App sends a challenge to the product's NFC chip

Chip signs the challenge with its private key

App fetches product data from Solana

App checks:

Signature matches product_pubkey

status == ACTIVE

Expected author

Expected owner (optional)

### 🔄 Ownership Transfer Workflow

Current owner initiates transfer

Program validates:

signer is the current owner

product is ACTIVE

Updates owner_pubkey

Updates OwnershipHistoryAccount

## Product Lifecycle (State Machine)
[*] --> Unregistered
Unregistered --> Registered : register_product()
Registered --> Active
Active --> Active : transfer_ownership()
Active --> Active : update_product_metadata()
Active --> Revoked : revoke_product()
Revoked --> [*]

## Why Solana?

Ultra-low fees

High throughput

Fast verification

Perfect for real-time interactions like NFC taps

Supports PDA-based identity models

## Future Enhancements

Device attestation & hardware security modules

Batch registration for manufacturers

Off-chain metadata with IPFS or Arweave

Open marketplace for authenticated products

Multi-signature (multisig) author governance

Support for multiple tag types (NTAG, ST25TA-E, etc.)

## License

MIT or Apache 2.0 (choose later)