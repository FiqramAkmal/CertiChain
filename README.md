# CertiChain: Immutable Academic Verifier

**CertiChain** - A blockchain-powered community platform built on the **Stellar Network** using **Soroban Smart Contracts**. This project aims to secure professional hiring by letting universities publish verifiable credential hashes on-chain.

---

## 📌 Problem Statement
Academic and professional credential fraud ruins hiring integrity and weakens organizational trust globally.

Currently, this issue faces several hurdles:
1. **Credential Fraud:** Job seekers easily forge diploma PDFs and certificate papers.
2. **Slow Background Checks:** Employers spend weeks calling colleges to verify credentials.
3. **High Admin Overhead:** Colleges waste resource cycles handling background verification requests.

## 🚀 Urgency
Employers spend weeks checking credentials manually. Uploading cryptographic document hashes lets companies perform free verification in seconds.
* **Instant Verification:** Employers check certificate validity in a single click.
* **Anti-Fraud Registry:** Cryptographic hashes protect document truth from edits.

## ✨ Key Features
* **Free Verification:** Verify credential legitimacy via cryptographic hashes on-ledger.
* **Sovereign Issuance:** Only verified academic institutions can issue certificates.
* **On-Chain Revocation:** Accommodates status changes (e.g. degree revocation or error correction).

---

## 🛠 Technical Stack
* **Blockchain:** Stellar Network
* **Smart Contract Engine:** Soroban
* **Language:** Rust
* **Development Environment:** Soroban CLI / Rust toolchain

## 📋 Smart Contract Overview
The contract handles academic credential issuances, verifications, and revocations:
1. `get_credentials()`: Fetch all issued academic credentials.
2. `issue_credential(institution, candidate, hash_proof)`: Verified university issues a credential with a document hash.
3. `verify_credential(hash_proof)`: Verify if a document hash represents a valid active credential.
4. `revoke_credential(institution, id)`: Revoke an issued credential in case of error or policy violation.

---

## 💡 Future Roadmap
- [ ] **Freighter QR verification:** Let students display credentials via offline Freighter QRs.
- [ ] **LinkedIn Direct Link:** Embed verified CertiChain links inside candidate profiles.
- [ ] **Course Badges:** Support granular micro-credential logs for single college courses.

---
## Screenshots
<img width="1920" height="1080" alt="image" src="screenshot.png" />

---
Stellar ID: GG7FTM62X5VFMIB5HKUC4LLSJGXNYUCAQSH2FOAG2LX2RIVXNC5TQBP6
*Developed for the Stellar Community and the advancement of Decentralized Social Economies.*
