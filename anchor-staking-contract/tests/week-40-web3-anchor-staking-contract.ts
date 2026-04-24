import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Week40Web3AnchorStakingContract } from "../target/types/anchor-staking-contract";

describe("week-40-web3-anchor-staking-contract", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.week40Web3AnchorStakingContract as Program<Week40Web3AnchorStakingContract>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
