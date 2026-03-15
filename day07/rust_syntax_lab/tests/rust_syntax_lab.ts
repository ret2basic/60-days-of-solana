import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { RustSyntaxLab } from "../target/types/rust_syntax_lab";

describe("rust_syntax_lab", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.rustSyntaxLab as Program<RustSyntaxLab>;

  it("demonstrates ownership, borrowing, and cloning", async () => {
    const tx = await program.methods.ownershipDemo().rpc();
    console.log("Your transaction signature", tx);
  });

  it("demonstrates copy types", async () => {
    const tx = await program.methods.copyTypeDemo().rpc();
    console.log("Your transaction signature", tx);
  });

  it("demonstrates mut variables", async () => {
    const tx = await program.methods.mutDemo().rpc();
    console.log("Your transaction signature", tx);
  });

  it("demonstrates generics", async () => {
    const tx = await program.methods.genericsDemo().rpc();
    console.log("Your transaction signature", tx);
  });

  it("demonstrates Option, unwrap, and deref", async () => {
    const tx = await program.methods.optionAndDerefDemo().rpc();
    console.log("Your transaction signature", tx);
  });

  it("demonstrates Result and the question-mark operator", async () => {
    const tx = await program.methods.encodeAndDecode().rpc();
    console.log("Your transaction signature", tx);
  });
});
