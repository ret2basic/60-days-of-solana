import * as anchor from "@coral-xyz/anchor";
import { expect } from "chai";
import { Program } from "@coral-xyz/anchor";
import { VisibilityLab } from "../target/types/visibility_lab";

describe("visibility_lab", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const provider = anchor.getProvider() as anchor.AnchorProvider;
  const program = anchor.workspace.visibilityLab as Program<VisibilityLab>;
  const state = anchor.web3.Keypair.generate();

  before(async () => {
    await program.methods
      .initialize()
      .accounts({
        state: state.publicKey,
        authority: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([state])
      .rpc();
  });

  const fetchState = async () =>
    program.account.visibilityState.fetch(state.publicKey);

  it("initializes the visibility state", async () => {
    const account = await fetchState();

    expect(account.authority.toBase58()).to.equal(
      provider.wallet.publicKey.toBase58()
    );
    expect(account.filePrivateValue.toNumber()).to.equal(0);
    expect(account.internalValue.toNumber()).to.equal(0);
    expect(account.restrictedValue.toNumber()).to.equal(0);
  });

  it("uses a file-private helper", async () => {
    await program.methods
      .filePrivateDemo()
      .accounts({
        state: state.publicKey,
        authority: provider.wallet.publicKey,
      })
      .rpc();

    const account = await fetchState();
    expect(account.filePrivateValue.toNumber()).to.equal(2);
  });

  it("uses an internal module from two call sites", async () => {
    await program.methods
      .internalVisibilityDemo()
      .accounts({
        state: state.publicKey,
        authority: provider.wallet.publicKey,
      })
      .rpc();

    const account = await fetchState();
    expect(account.internalValue.toNumber()).to.equal(7);
    expect(account.internalFromOtherModuleValue.toNumber()).to.equal(7);
  });

  it("uses a restricted function only from the parent module", async () => {
    await program.methods
      .restrictedVisibilityDemo()
      .accounts({
        state: state.publicKey,
        authority: provider.wallet.publicKey,
      })
      .rpc();

    const account = await fetchState();
    expect(account.restrictedValue.toNumber()).to.equal(13);
  });

  it("uses a helper from a separate file", async () => {
    await program.methods
      .separateFileModuleDemo(new anchor.BN(4), new anchor.BN(6))
      .accounts({
        state: state.publicKey,
        authority: provider.wallet.publicKey,
      })
      .rpc();

    const account = await fetchState();
    expect(account.separateFileSum.toNumber()).to.equal(10);
  });

  it("uses a helper from an inline module", async () => {
    await program.methods
      .inlineModuleDemo(new anchor.BN(10), new anchor.BN(15))
      .accounts({
        state: state.publicKey,
        authority: provider.wallet.publicKey,
      })
      .rpc();

    const account = await fetchState();
    expect(account.inlineSum.toNumber()).to.equal(25);
  });
});
