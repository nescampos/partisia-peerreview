# Partisia Peer Review

Peer Review is a new project, to enable peer reviews for the scientific community.

This project is built using a fork from [Partisia Blockchain dApp Playground](https://github.com/partisiablockchain/dapp-playground). Follow the instructions there to enable the Codespace.

Smart contract (test): https://browser.testnet.partisiablockchain.com/contracts/0227a8c08ac1855e8d8701bc367f9db86c8122140f

## Status

- Smart contract implemented, in _peerreview_ folder.
- UI for PeerReview (Work in progress).

## It is an Automated setup

To interact with the Partisia Blockchain you need an account with gas.
The codespace automatically provides you with three new accounts,
`Account-A.pk`, `Account-B.pk` and `Account-C.pk`.

The created accounts have test_coins pre-minted which gives you 100.000.000 gas on the Testnet to
interact, deploy and
play around with as part of the codespace. You can continue using these outside of the codespace,
just remember the private keys, because they are not saved when you delete the codespace.

Read how addresses works
for [accounts and smart contracts](https://partisiablockchain.gitlab.io/documentation/pbc-fundamentals/dictionary.html#address).

## How you can test

1. Build the project:
```sh
cargo partisia-contract build --release
```

2. Implement the smart contract (example with parameters):
```sh
cargo partisia-contract cli tx deploy --gas 2500000 --privatekey Account-A.pk  /workspaces/partisia-peerreview/target/wasm32-unknown-unknown/release/peerreview.wasm /workspaces/partisia-peerreview/target/wasm32-unknown-unknown/release/peerreview.abi 'My first Paper' 'This is a paper for my research for 5 years' 'https://demo.org'
```
3. Check the smart contract in the testnet.


## Team

This project is building by [Nestor Campos](https://www.linkedin.com/in/nescampos/)
and a web frontend written in TypeScript.

If you want to know more about the blockchain, ZK Rust or just contracts in general,
we urge you to visit our [documentation](https://partisiablockchain.gitlab.io/documentation/) and
participate
in [our active community on Discord](https://partisiablockchain.gitlab.io/documentation/get-support-from-pbc-community.html).
