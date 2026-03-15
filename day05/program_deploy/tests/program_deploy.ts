import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { strict as assert } from "assert";
import fs from "fs";
import path from "path";
import { ProgramDeploy } from "../target/types/program_deploy";

describe("program_deploy", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const idlPath = path.join(__dirname, "..", "target", "idl", "program_deploy.json");
  const keypairPath = path.join(
    __dirname,
    "..",
    "target",
    "deploy",
    "program_deploy-keypair.json",
  );
  const idl = JSON.parse(fs.readFileSync(idlPath, "utf8")) as ProgramDeploy;
  const keypair = anchor.web3.Keypair.fromSecretKey(
    Uint8Array.from(JSON.parse(fs.readFileSync(keypairPath, "utf8"))),
  );
  const program = new Program<ProgramDeploy>(idl, provider);

  it("runs initialize without redeploying", async () => {
    assert.equal(program.programId.toBase58(), keypair.publicKey.toBase58());

    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
