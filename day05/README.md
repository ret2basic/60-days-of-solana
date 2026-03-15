# Day 5

This day is complete in `day05/program_deploy`.

## What was done

- Created an Anchor project named `program_deploy`
- Kept the program minimal with a single `initialize` instruction
- Changed the test to load the generated IDL directly and construct `Program` manually instead of using `anchor.workspace`
- Verified `anchor test --skip-local-validator --skip-deploy` executes the already deployed program without redeploying it
- Verified the same program id was reused across deploys while the logged message changed after an upgrade

## Commands that worked here

```bash
cd day05/program_deploy
anchor build
solana-test-validator --reset
solana config set --url localhost
solana airdrop 100 $(solana address)
anchor deploy
anchor test --skip-local-validator --skip-deploy
```

## Notes

- The deployed program id stayed `6mV3DoqKv4Bo37wtXaXa5wEDfpSGUZNCgDaZVz9cKNtH`
- The manual test reads `target/idl/program_deploy.json` and uses that generated IDL to create the client
- The first verification logged `program_deploy version 1`
- After rebuilding and redeploying, the second verification logged `program_deploy version 2`
- In this setup, changing the Rust source and running `anchor deploy` alone was not enough to update the on-chain behavior; rebuilding first with `anchor build` ensured the upgraded `.so` matched the new source

## Exercise Answers

- Exercise 1: Yes, the message string changed while the program id stayed the same. That demonstrates Solana upgraded the program at the existing address instead of deploying a brand new one.
- Exercise 2: The two observed log messages were `program_deploy version 1` and then `program_deploy version 2`, and both came from the same program id `6mV3DoqKv4Bo37wtXaXa5wEDfpSGUZNCgDaZVz9cKNtH`.
- Exercise 3: Running `anchor test --skip-local-validator --skip-deploy` still executed `initialize`, but it did not redeploy the program first. The log file confirmed only the instruction execution, not a fresh deploy or upgrade during the test step.
- Exercise 4: Solana programs do not have constructors. Deployment and later upgrades happen as separate loader operations, and Anchor normally performs those operations around your workflow unless you explicitly skip them.
- Exercise 5: One practical detail from this repo is that source changes should be rebuilt before redeploying. Otherwise the deployed bytecode can still reflect the previous compiled artifact.