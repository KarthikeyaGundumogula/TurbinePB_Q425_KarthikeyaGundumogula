import * as anchor from "@coral-xyz/anchor";
import type { Program } from "@coral-xyz/anchor";
import { Keypair, PublicKey } from "@solana/web3.js";
import { assert } from "chai";
import { CounterSolana } from "../target/types/counter_solana";

describe("counter_solana", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.counterSolana as Program<CounterSolana>;
  const user = anchor.web3.Keypair.generate();
  let counterPDA: anchor.web3.PublicKey;

  before(async () => {
    await getAirdrop(user.publicKey);
    [counterPDA] = getPDA(user.publicKey, program.programId);
  });
  let max_value = new anchor.BN(100);
  let min_value = new anchor.BN(10);

  it("Initialize Counter", async () => {
    await program.methods
      .initializeCounter(max_value, min_value)
      .accounts({
        payer: user.publicKey,
        counter: counterPDA,
        systemProgram: anchor.web3.SystemProgram.programId,
      } as any)
      .signers([user])
      .rpc();

    const counterAccount = await program.account.counter.fetch(counterPDA);
    assert.equal(counterAccount.count.toNumber(), 10);
  });

  it("Increment Counter", async () => {
    let oldCounter = await program.account.counter.fetch(counterPDA);
    await program.methods
      .increment()
      .accounts({ counter: counterPDA, signer: user.publicKey } as any)
      .signers([user])
      .rpc();
    let newCounter = await program.account.counter.fetch(counterPDA);
    assert.equal(newCounter.count.toNumber(), oldCounter.count.toNumber() + 1);
  });

  it("Decrement Counter", async () => {
    let oldCounter = await program.account.counter.fetch(counterPDA);
    await program.methods
      .decrement()
      .accounts({ counter: counterPDA, signer: user.publicKey } as any)
      .signers([user])
      .rpc();
    let newCounter = await program.account.counter.fetch(counterPDA);
    assert.equal(newCounter.count.toNumber(), oldCounter.count.toNumber() - 1);
  });

  it("Reset Counter", async () => {
    await program.methods
      .reset()
      .accounts({ counter: counterPDA, signer: user.publicKey } as any)
      .signers([user])
      .rpc();
    let newCounter = await program.account.counter.fetch(counterPDA);
    assert.equal(newCounter.count.toNumber(), min_value.toNumber());
  });

  const getAirdrop = async (user: anchor.web3.PublicKey) => {
    const tx = await program.provider.connection.requestAirdrop(
      user,
      2 * anchor.web3.LAMPORTS_PER_SOL
    );
    await program.provider.connection.confirmTransaction(tx);
  };

  const getPDA = (user: PublicKey, programId: PublicKey) => {
    return PublicKey.findProgramAddressSync(
      [Buffer.from("Counter"), user.toBuffer()],
      programId
    );
  };
});
