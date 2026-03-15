import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { ArithmeticBasics } from "../target/types/arithmetic_basics";
import { expect } from "chai";

describe("arithmetic_basics", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.arithmeticBasics as Program<ArithmeticBasics>;

  it("logs integer and string arguments", async () => {
    const tx = await program.methods
      .initialize(new anchor.BN(777), new anchor.BN(888), "hello")
      .rpc();
    console.log("Your transaction signature", tx);
  });

  it("accepts an array of integers", async () => {
    const tx = await program.methods
      .array([new anchor.BN(777), new anchor.BN(888)])
      .rpc();
    console.log("Your transaction signature", tx);
  });

  it("runs calculator-style integer operations", async () => {
    const addTx = await program.methods
      .add(new anchor.BN(4), new anchor.BN(5))
      .rpc();
    const subtractTx = await program.methods
      .subtract(new anchor.BN(9), new anchor.BN(4))
      .rpc();
    const multiplyTx = await program.methods
      .multiply(new anchor.BN(6), new anchor.BN(7))
      .rpc();
    const divideTx = await program.methods
      .divide(new anchor.BN(81), new anchor.BN(9))
      .rpc();
    const powerTx = await program.methods
      .power(new anchor.BN(2), 10)
      .rpc();

    console.log({ addTx, subtractTx, multiplyTx, divideTx, powerTx });
  });

  it("runs float math operations", async () => {
    const cubeRootTx = await program.methods.cubeRoot(50).rpc();
    const sqrtTx = await program.methods.sqrt(144).rpc();
    const log10Tx = await program.methods.log10(1000).rpc();

    console.log({ cubeRootTx, sqrtTx, log10Tx });
  });

  it("rejects underflow with checked subtraction", async () => {
    try {
      await program.methods.subtract(new anchor.BN(0), new anchor.BN(1)).rpc();
      expect.fail("expected subtract to fail");
    } catch (error) {
      expect(`${error}`).to.include("Arithmetic underflow");
    }
  });
});
