# Blueshift Anchor Vault

Completed the Blueshift Anchor Vault challenge. Simple escrow program on Solana using Anchor.

## What it does

- Deposit SOL into a PDA vault
- Withdraw all SOL from the vault
- Each user gets their own vault derived from their pubkey

## Structure

- `deposit`: Transfer SOL from user to vault PDA
- `withdraw`: Transfer all SOL back to user and close vault
- Vault seeds: `["vault", user_pubkey]`

## Requirements

- Amount must be greater than rent-exempt minimum
- Can only deposit to empty vault
- Can only withdraw from existing vault

## Build

```bash
anchor build
```

## Test

```bash
anchor test
```

## Deploy

```bash
anchor deploy
```
