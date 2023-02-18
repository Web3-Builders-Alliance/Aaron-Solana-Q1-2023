anchor `v0.26` bootstrap by amilz.sol

tips: 
- one termianl run local validator: `solana-test-validator`
    - optinal tag to run w/ [local programs](https://solanacookbook.com/references/local-development.html#how-to-load-programs-from-mainnet) `--bpf-program metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s metaplex.so --reset` 
- `anchor build`
- `solana airdrop 100 WALLET_ADDRESS`
- `anchor deploy`
- `anchor test --skip-deploy --skip-local-validator  --skip-build`
- Add IDL To local explorer `anchor idl init --filepath target/idl/bootstrap.json PROGRAM_ID`
