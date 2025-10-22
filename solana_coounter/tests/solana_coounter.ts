import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolanaCoounter } from "../target/types/solana_coounter";

describe("solana_coounter", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.solanaCoounter as Program<SolanaCoounter>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initCounter().rpc();
    console.log("Your transaction signature", tx);
  });
});
