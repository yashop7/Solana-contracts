# Solana Staking Contract

A staking program built with Anchor that lets you stake SOL and earn points over time. Pretty straightforward - the longer you stake, the more points you rack up.

## What it does

You can stake your SOL and earn 1 point per SOL per day. The contract tracks everything in a PDA (Program Derived Address) specific to your wallet, so your stake and points are tied to you.

Main features:
- Stake any amount of SOL
- Unstake whenever you want (partial or full)
- Points accumulate automatically based on time staked
- Claim your points when you're ready
- Check your current points and stake without making a transaction

## How the points work

The math is simple: 1 SOL staked for 1 day = 1 point. Stake 5 SOL for 2 days? That's 10 points. The contract uses micro-points internally for precision, so you don't lose anything to rounding.

Points keep accumulating even after you claim them - they're not reset when you unstake. Only claiming points resets your counter.

## Building and testing

Build the program:
```bash
anchor build
```

Start a local validator (in a separate terminal):
```bash
solana-test-validator
```

Run tests:
```bash
anchor test --skip-local-validator
```

## Using the contract

The program has 5 instructions you can call:

### 1. Create your staking account
First time setup - creates a PDA to track your stakes and points.
```typescript
await program.methods
  .createPdaAccount()
  .accounts({
    payer: wallet.publicKey,
    systemProgram: SystemProgram.programId,
  })
  .rpc();
```

### 2. Stake SOL
Lock up some SOL to start earning points.
```typescript
const amount = 1 * LAMPORTS_PER_SOL; // 1 SOL
await program.methods
  .stake(new BN(amount))
  .accounts({
    user: wallet.publicKey,
    systemProgram: SystemProgram.programId,
  })
  .rpc();
```

### 3. Unstake SOL
Take your SOL back whenever you want.
```typescript
const amount = 0.5 * LAMPORTS_PER_SOL; // 0.5 SOL
await program.methods
  .unstake(new BN(amount))
  .accounts({
    user: wallet.publicKey,
    systemProgram: SystemProgram.programId,
  })
  .rpc();
```

### 4. Check your points
View your current points without spending gas.
```typescript
await program.methods
  .getPoints()
  .accounts({
    user: wallet.publicKey,
  })
  .rpc();
```

### 5. Claim points
Reset your points counter (useful if you're implementing a reward system).
```typescript
await program.methods
  .claimPoints()
  .accounts({
    user: wallet.publicKey,
  })
  .rpc();
```

## Project structure

```
programs/week-40-web3-anchor-staking-contract/src/
  └── lib.rs          # Main contract logic
tests/
  └── week-40-web3-anchor-staking-contract.ts  # Test suite
Anchor.toml           # Anchor configuration
```

## Technical details

- Uses PDAs derived from seed "client1" + user's pubkey
- Points calculated using micro-points (1M micro-points = 1 point) to avoid precision loss
- Proper overflow/underflow checks on all arithmetic
- Time-based calculations use Unix timestamps
- Account size: 8 (discriminator) + 32 (owner) + 8 (staked) + 8 (points) + 8 (timestamp) + 1 (bump) = 65 bytes

## License

Do whatever you want with this code.
