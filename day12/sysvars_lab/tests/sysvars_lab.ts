import * as anchor from "@coral-xyz/anchor";
import { expect } from "chai";
import { Program } from "@coral-xyz/anchor";
import { SysvarsLab } from "../target/types/sysvars_lab";

describe("sysvars_lab", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const provider = anchor.getProvider() as anchor.AnchorProvider;
  const program = anchor.workspace.sysvarsLab as Program<SysvarsLab>;
  const state = anchor.web3.Keypair.generate();
  const number = 3;
  const lastRestartSlotPubkey = new anchor.web3.PublicKey(
    "SysvarLastRestartS1ot1111111111111111111111"
  );

  before(async () => {
    await program.methods
      .initialize(number)
      .accounts({
        state: state.publicKey,
        authority: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
        stakeHistory: anchor.web3.SYSVAR_STAKE_HISTORY_PUBKEY,
        instructionSysvar: anchor.web3.SYSVAR_INSTRUCTIONS_PUBKEY,
        lastRestartSlot: lastRestartSlotPubkey,
      })
      .signers([state])
      .rpc();
  });

  const fetchState = async () =>
    program.account.sysvarSnapshot.fetch(state.publicKey);

  it("records get()-based sysvars", async () => {
    const account = await fetchState();
    const minimumBalance = await provider.connection.getMinimumBalanceForRentExemption(
      0
    );

    expect(account.authority.toBase58()).to.equal(
      provider.wallet.publicKey.toBase58()
    );
    expect(account.unixTimestamp.toNumber()).to.be.greaterThan(0);
    expect(account.slot.toNumber()).to.be.greaterThan(0);
    expect(account.epoch.toNumber()).to.be.at.least(0);
    expect(account.slotsPerEpoch.toNumber()).to.be.greaterThan(0);
    expect(account.leaderScheduleSlotOffset.toNumber()).to.be.greaterThan(0);
    expect(account.warmup).to.be.a("boolean");
    expect(account.lamportsPerByteYear.toNumber()).to.equal(3480);
    expect(account.rentExemptionThresholdMilli.toNumber()).to.equal(2000);
    expect(account.burnPercent.toNumber()).to.equal(50);
    expect(account.minimumBalanceForZeroBytes.toNumber()).to.equal(
      minimumBalance
    );
  });

  it("records account-based sysvars", async () => {
    const account = await fetchState();

    expect(account.stakeHistoryEntries).to.equal(0);
    expect(account.lastRestartSlot.toNumber()).to.be.at.least(0);
  });

  it("reads the instructions sysvar and decodes the argument", async () => {
    const account = await fetchState();

    expect(account.instructionProgramId.toBase58()).to.equal(
      program.programId.toBase58()
    );
    expect(account.instructionAccountCount).to.be.greaterThan(0);
    expect(account.instructionDataLen).to.be.greaterThan(8);
    expect(account.currentInstructionIndex.toNumber()).to.equal(0);
    expect(account.providedNumber).to.equal(number);
    expect(account.observedNumber).to.equal(number);
  });
});
