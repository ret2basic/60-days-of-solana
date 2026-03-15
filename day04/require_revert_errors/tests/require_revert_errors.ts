import * as anchor from "@coral-xyz/anchor";
import { AnchorError, Program } from "@coral-xyz/anchor";
import { RequireRevertErrors } from "../target/types/require_revert_errors";
import { assert } from "chai";

describe("require_revert_errors", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.requireRevertErrors as Program<RequireRevertErrors>;

  it("accepts values inside the allowed range", async () => {
    const tx = await program.methods.limitRange(new anchor.BN(50)).rpc();
    console.log("Your transaction signature", tx);
  });

  it("rejects values outside the allowed range", async () => {
    try {
      await program.methods.limitRange(new anchor.BN(9)).rpc();
      assert.fail("expected lower bound error");
    } catch (_err) {
      assert.isTrue(_err instanceof AnchorError);
      const err = _err as AnchorError;
      assert.strictEqual(err.error.errorMessage, "a is too small");
      console.log("Error number:", err.error.errorCode.number);
    }

    try {
      await program.methods.limitRange(new anchor.BN(101)).rpc();
      assert.fail("expected upper bound error");
    } catch (_err) {
      assert.isTrue(_err instanceof AnchorError);
      const err = _err as AnchorError;
      assert.strictEqual(err.error.errorMessage, "a is too big");
      console.log("Error number:", err.error.errorCode.number);
    }
  });

  it("returns the custom AlwaysErrors error", async () => {
    try {
      await program.methods.func().rpc();
      assert.fail("expected func to error");
    } catch (_err) {
      assert.isTrue(_err instanceof AnchorError);
      const err = _err as AnchorError;
      assert.strictEqual(err.error.errorMessage, "Always errors");
      console.log("Error number:", err.error.errorCode.number);
    }
  });

  it("can log and still succeed when returning Ok", async () => {
    const tx = await program.methods.funcOk().rpc();
    console.log("Your transaction signature", tx);
  });
});
