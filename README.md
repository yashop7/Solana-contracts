# 🚀 Solana Smart Contracts Collection

> A comprehensive collection of production-ready Solana smart contracts built with Rust & Anchor framework — featuring AMMs, escrow systems, staking, NFT protocols, and advanced DeFi primitives.

![Solana](https://img.shields.io/badge/Solana-000000?style=for-the-badge&logo=solana&logoColor=white)
![Rust](https://img.shields.io/badge/Rust-DEA584?style=for-the-badge&logo=rust&logoColor=black)
![Anchor](https://img.shields.io/badge/Anchor-000000?style=for-the-badge&logo=anchor&logoColor=14f195)
![License](https://img.shields.io/badge/License-MIT-yellow.svg)

---

## 📚 Table of Contents

- [Overview](#-overview)
- [Projects](#-projects)
- [Tech Stack](#-tech-stack)
- [Getting Started](#-getting-started)
- [Contract Details](#-contract-details)
- [Security](#-security)
- [Contributing](#-contributing)
- [License](#-license)

---

## 🌟 Overview

This repository contains a curated collection of **Solana smart contracts** developed during the **Turbin3 Q1 2025 cohort**. Each contract demonstrates production-ready patterns for building secure, efficient, and composable decentralized applications on Solana.

### Key Highlights

- ✅ **12+ Smart Contracts** — From foundational primitives to advanced DeFi protocols
- 🔐 **Security-First** — Built with Anchor's security features and best practices
- ⚡ **Gas Optimized** — Minimal compute units with efficient state management
- 🧪 **Fully Tested** — Comprehensive test suites for each contract
- 🔗 **Composable** — Cross-program invocations (CPIs) and PDA patterns

---

## 📦 Projects

| Contract | Description | Status |
|----------|-------------|--------|
| [anchor-vault](./anchor-vault/) | Secure SOL vault with PDA-based account management | ✅ |
| [anchor-escrow](./anchor-escrow/) | Trustless token escrow for peer-to-peer swaps | ✅ |
| [anchor-amm](./anchor-amm/) | Constant product AMM with liquidity pools | ✅ |
| [anchor-dice-game](./anchor-dice-game/) | On-chain casino with verifiable randomness | ✅ |
| [anchor-nft-staking](./anchor-nft-staking/) | NFT staking with time-weighted rewards | ✅ |
| [anchor-staking-contract](./anchor-staking-contract/) | General SPL token staking mechanism | ✅ |
| [anchor-mplxcore](./anchor-mplxcore/) | Metaplex Core NFT integration | ✅ |
| [anchor-quadratic-funding](./anchor-quadratic-funding/) | Quadratic funding for public goods | ✅ |
| [blueshift-anchor-vault](./blueshift_anchor_vault/) | Advanced vault implementation | ✅ |
| [blueshift-anchor-escrow](./blueshift_anchor_escrow/) | Enhanced escrow system | ✅ |

---

## 🛠 Tech Stack

<div align="center">

| Technology | Purpose |
|------------|---------|
| **Rust** | Smart contract language |
| **Anchor** | Solana development framework |
| **Solana** | Blockchain runtime |
| **TypeScript** | Test suites & clients |
| **Solana CLI** | Local development |
| **Surfpool** | Local validator |

</div>

---

## 🚦 Getting Started

### Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"

# Install Anchor
cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
avm install latest
avm use latest

# Install Node.js & Yarn
brew install node yarn
```

### Running Tests

```bash
# Navigate to any contract
cd anchor-vault

# Build the program
anchor build

# Run tests
anchor test

# Or with Surfpool (faster)
surfpool start &
anchor test --skip-local-validator
```

---

## 💎 Contract Details

### 🔐 Anchor Vault
Secure vault program demonstrating:
- Program Derived Addresses (PDAs) for deterministic addresses
- SOL deposit & withdrawal mechanisms
- Account initialization & closure patterns
- Secure program-derived authority

```rust
// Key concepts: PDAs, SOL transfers, Account validation
```

### 🤝 Anchor Escrow
Trustless escrow system featuring:
- Maker/Taker swap model
- PDA-based token vaults
- Refund mechanism for cancelled trades
- Atomic token swaps

```rust
// Key concepts: Token vaults, CPI, Account constraints
```

### 📈 Anchor AMM
Decentralized exchange implementing:
- Constant product formula (x * y = k)
- Liquidity pool creation & management
- LP token minting & burning
- Swap fee distribution

```rust
// Key concepts: Mathematical formulas, Token mints, Pool accounts
```

### 🎲 Anchor Dice Game
On-chain casino showcasing:
- Verifiable randomness (no chainlink needed)
- State management for game sessions
- Probability-based outcomes
- Bet & payout handling

```rust
// Key concepts: Randomness, State accounts, Error handling
```

### 🎨 NFT Staking
NFT staking platform with:
- Stake any Metaplex-compatible NFT
- Time-based reward calculation
- Early withdrawal penalties
- Reward distribution

```rust
// Key concepts: NFT metadata, Token accounts, Clock sysvar
```

### 🏆 Quadratic Funding
Public goods funding mechanism:
- Quadratic voting algorithm
- Matching pool calculations
- Multi-contributor tracking
- Democratic allocation

```rust
// Key concepts: Math operations, Account arrays, Overflow checks
```

---

## 🔒 Security

Each contract implements:

- ✅ **Signer Verification** — All critical instructions require valid signers
- ✅ **Account Validation** — Comprehensive account checks before mutations
- ✅ **Overflow Protection** — Safe math operations for token calculations
- ✅ **Reinitialization Guards** — Prevention of account reinitialization attacks
- ✅ **PDA Verification** — Proper PDA validation for program-derived authorities
- ✅ **Token Checks** — SPL token balance and mint verification

---

## 🤝 Contributing

1. **Fork** the repository
2. **Create** a feature branch (`git checkout -b feature/amazing`)
3. **Commit** your changes (`git commit -m 'Add amazing feature'`)
4. **Push** to the branch (`git push origin feature/amazing`)
5. **Open** a Pull Request

---

## 📄 License

MIT License — see [LICENSE](LICENSE) for details.

---

<div align="center">

**Built with ❤️ during Turbin3 Q1 2025**

*[🌐 turbinsolana.io](https://turbin3.com)* • *[📚 Anchor Docs](https://www.anchor-lang.com/)* • *[📖 Solana Cookbook](https://solanacookbook.com/)*

</div>
# Solana-contracts
