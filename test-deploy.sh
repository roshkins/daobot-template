#!/bin/bash

# rm -r ./target
cargo build --target wasm32-unknown-unknown --release
near dev-deploy ./target/wasm32-unknown-unknown/release/status_message.wasm
export $(cat ./neardev/dev-account.env  | xargs)
near call ${CONTRACT_NAME}  approve_members '{"dao_id":"autodao.sputnikv2.testnet"}' --gas 300000000000000 --accountId moopaloo.testnet 
#near call ${CONTRACT_NAME}  something '{"arg1":" blah"}' --accountId moopaloo.testnet
