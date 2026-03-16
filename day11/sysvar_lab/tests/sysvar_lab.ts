import * as anchor from "@coral-xyz/anchor";
import { expect } from "chai";
import { Program } from "@coral-xyz/anchor";
import { SysvarLab } from "../target/types/sysvar_lab";

describe("sysvar_lab", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const provider = anchor.getProvider() as anchor.AnchorProvider;
  const program = anchor.workspace.sysvarLab as Program<SysvarLab>;
  const state = anchor.web3.Keypair.generate();
  const weekdays = new Set([
    "Mon",
    "Tue",
    "Wed",
    "Thu",
    "Fri",
    "Sat",
    "Sun",
  ]);

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

  const fetchState = async () => program.account.sysvarState.fetch(state.publicKey);

  it("records clock sysvar fields", async () => {
    const account = await fetchState();

    expect(account.authority.toBase58()).to.equal(
      provider.wallet.publicKey.toBase58()
    );
    expect(account.unixTimestamp.toNumber()).to.be.greaterThan(0);
    expect(account.slot.toNumber()).to.be.greaterThan(0);
    expect(account.epoch.toNumber()).to.be.at.least(0);
  });

  it("derives the current weekday from unix timestamp", async () => {
    await program.methods
      .getDayOfTheWeek()
      .accounts({
        state: state.publicKey,
        authority: provider.wallet.publicKey,
      })
      .rpc();

    const account = await fetchState();
    expect(weekdays.has(account.dayOfWeek)).to.equal(true);
  });

  it("reads the deprecated recent blockhash sysvar", async () => {
    await program.methods
      .readRecentBlockhash()
      .accounts({
        state: state.publicKey,
        authority: provider.wallet.publicKey,
        recentBlockhashes: anchor.web3.SYSVAR_RECENT_BLOCKHASHES_PUBKEY,
      })
      .rpc();

    const account = await fetchState();
    expect(account.recentBlockhash.length).to.be.greaterThan(0);
  });
});
