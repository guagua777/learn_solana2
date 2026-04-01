import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SurfpoolDemo } from "../target/types/surfpool_demo";

// describe("surfpool-demo", () => {
//   // Configure the client to use the local cluster.
//   anchor.setProvider(anchor.AnchorProvider.env());

//   const program = anchor.workspace.surfpoolDemo as Program<SurfpoolDemo>;

//   it("Is initialized!", async () => {
//     // Add your test here.
//     const tx = await program.methods.initialize().rpc();
//     console.log("Your transaction signature", tx);
//   });
// });



describe("surfpool-demo", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.SurfpoolDemo as Program<SurfpoolDemo>;
  const counter = anchor.web3.Keypair.generate();

  it("初始化计数器", async () => {
    await program.methods
      .initialize()
      .accounts({
        counter: counter.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([counter])
      .rpc();
  });

  it("点击 +1", async () => {
    await program.methods
      .increment()
      .accounts({ counter: counter.publicKey })
      .rpc();

    const account = await program.account.counter.fetch(counter.publicKey);
    console.log("✅ 最终计数：", account.count.toString());
    anchor.BN.assert(new anchor.BN(1), "eq", account.count);
  });
});
