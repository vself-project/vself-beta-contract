#!/bin/bash
set -e

ID=beta_v5.ilerik.testnet

# recreate account
# near delete $ID ilerik.testnet
near create-account $ID --masterAccount=ilerik.testnet --initial-balance 100

# deploy contract
near deploy --wasmFile out/main.wasm --accountId $ID
near call $ID new --accountId $ID
near view $ID is_active

# copy credentials for later deploy
cp ~/.near-credentials/testnet/$ID.json ./creds