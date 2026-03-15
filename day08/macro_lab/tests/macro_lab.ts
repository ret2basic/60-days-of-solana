import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { MacroLab } from "../target/types/macro_lab";

describe("macro_lab", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.macroLab as Program<MacroLab>;

  it("demonstrates function-like macros", async () => {
    const tx = await program.methods.functionLikeMacroDemo().rpc();
    console.log("Your transaction signature", tx);
  });

  it("demonstrates fixed-arity helper functions", async () => {
    const tx = await program.methods.fixedArityFunctionDemo().rpc();
    console.log("Your transaction signature", tx);
  });

  it("demonstrates Anchor macro categories", async () => {
    const tx = await program.methods.anchorMacroInventory().rpc();
    console.log("Your transaction signature", tx);
  });
});
