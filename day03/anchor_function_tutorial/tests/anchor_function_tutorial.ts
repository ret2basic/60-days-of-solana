import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorFunctionTutorial } from "../target/types/anchor_function_tutorial";
import { expect } from "chai";

describe("anchor_function_tutorial", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.anchorFunctionTutorial as Program<AnchorFunctionTutorial>;

  it("calls boaty mc boatface", async () => {
    const tx = await program.methods.boatyMcBoatface(new anchor.BN(42)).rpc();
    console.log("Your transaction signature", tx);
  });

  it("performs add and sub", async () => {
    const addTx = await program.methods.add(new anchor.BN(1), new anchor.BN(2)).rpc();
    const subTx = await program.methods.sub(new anchor.BN(10), new anchor.BN(3)).rpc();
    console.log({ addTx, subTx });
  });

  it("performs mul, div, and modulo", async () => {
    const mulTx = await program.methods.mul(new anchor.BN(6), new anchor.BN(7)).rpc();
    const divTx = await program.methods.div(new anchor.BN(20), new anchor.BN(5)).rpc();
    const moduloTx = await program.methods.modulo(new anchor.BN(20), new anchor.BN(6)).rpc();
    console.log({ mulTx, divTx, moduloTx });
  });

  it("accepts a non-empty accounts struct", async () => {
    const anotherSigner = anchor.web3.Keypair.generate();
    const signature = await program.methods
      .nonEmptyAccountExample()
      .accountsPartial({
        signer: provider.wallet.publicKey,
        anotherSigner: anotherSigner.publicKey,
      })
      .signers([anotherSigner])
      .rpc();

    console.log("nonEmptyAccountExample", signature);
  });

  it("rejects division by zero", async () => {
    try {
      await program.methods.div(new anchor.BN(10), new anchor.BN(0)).rpc();
      expect.fail("expected div to fail");
    } catch (error) {
      expect(`${error}`).to.include("Division by zero");
    }
  });
});
