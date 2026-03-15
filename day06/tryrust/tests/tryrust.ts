import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Tryrust } from "../target/types/tryrust";

describe("tryrust", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.tryrust as Program<Tryrust>;

  it("logs the constant example", async () => {
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });

  it("runs the if/else age checker", async () => {
    const tx = await program.methods.ageChecker(new anchor.BN(35)).rpc();
    console.log("Your transaction signature", tx);
  });

  it("runs the ternary-style checker", async () => {
    const tx = await program.methods.ternaryChecker(new anchor.BN(24)).rpc();
    console.log("Your transaction signature", tx);
  });

  it("runs the match example", async () => {
    const tx = await program.methods.matchChecker(new anchor.BN(5)).rpc();
    console.log("Your transaction signature", tx);
  });

  it("runs the stepped loop example", async () => {
    const tx = await program.methods.loopDemo().rpc();
    console.log("Your transaction signature", tx);
  });

  it("runs the array and vector example", async () => {
    const tx = await program.methods.vectorDemo().rpc();
    console.log("Your transaction signature", tx);
  });

  it("runs the HashMap example", async () => {
    const tx = await program.methods.hashmapDemo("name", "Bob").rpc();
    console.log("Your transaction signature", tx);
  });

  it("runs the struct example", async () => {
    const tx = await program.methods.structDemo("Alice", new anchor.BN(20)).rpc();
    console.log("Your transaction signature", tx);
  });

  it("runs the usize casting example", async () => {
    const tx = await program.methods.usizeDemo().rpc();
    console.log("Your transaction signature", tx);
  });

  it("runs the even-number exercise", async () => {
    const tx = await program.methods
      .filterEvenNumbers([
        new anchor.BN(1),
        new anchor.BN(2),
        new anchor.BN(3),
        new anchor.BN(4),
        new anchor.BN(5),
        new anchor.BN(6),
      ])
      .rpc();
    console.log("Your transaction signature", tx);
  });
});
