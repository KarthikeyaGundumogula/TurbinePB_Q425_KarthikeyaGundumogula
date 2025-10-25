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
  const user1 = anchor.web3.Keypair.generate();
  let counterPDA: anchor.web3.PublicKey;
  let counterPDA1: anchor.web3.PublicKey;

  before(async () => {
    await getAirdrop(user.publicKey);
    await getAirdrop(user1.publicKey);
    [counterPDA] = getPDA(user.publicKey, program.programId);
    [counterPDA1] = getPDA(user1.publicKey, program.programId);
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

  it("does not increment counter above max value", async () => {
    let max_value1 = new anchor.BN(10);
    let min_value1 = new anchor.BN(9);
    await program.methods
      .initializeCounter(max_value1, min_value1)
      .accounts({
        payer: user1.publicKey,
        counter: counterPDA1,
        systemProgram: anchor.web3.SystemProgram.programId,
      } as any)
      .signers([user1])
      .rpc();

    const counterAccount = await program.account.counter.fetch(counterPDA1);
    assert.equal(counterAccount.count.toNumber(), 9);

    // Now increment user1's counter using the correct PDA
    await program.methods
      .increment()
      .accounts({ counter: counterPDA1, signer: user1.publicKey } as any)
      .signers([user1])
      .rpc();
    let newCounter = await program.account.counter.fetch(counterPDA1);
    assert.equal(newCounter.count.toNumber(), min_value1.toNumber() + 1);
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
