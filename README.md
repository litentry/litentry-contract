# litentry contract
The repo gives an example contract how to use the data from Litentry network to compute credit score. The project based on the [redspot](https://github.com/patractlabs/redspot) from patractlab. The erc20 is example of Ink contract. The credit is the one to be deployed in Litentry blockchain and show how to leverage the data from Litentry.

## build and run europa
With [europa](https://github.com/patractlabs/europa), you can test the contract without deployment in the Litentry blockchain. Europa provide the service of sandbox environment of substrate runtime. For the pallets of Litentry specific, we need extend the europe smart contract pallet implementation. Details could be found in the litentry branch.

git clone submodule --recursive https://github.com/litentry/europa
git checkout litentry
cd europa
make release
release/target/europa --tmp

## build environment
1. install nodejs, yarn and redspot
cd credit
yarn add respot 

2. install ink
cargo install cargo-contract --force

3. build contract
cd credit
yarn build

## test contract
cd credit
yarn test
