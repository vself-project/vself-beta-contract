#!/bin/bash
set -e

# ID=beta_v8.ilerik.testnet
# MasterAccount=ilerik.testnet
ID=beta_v10.sergantche.testnet
MasterAccount=sergantche.testnet

# recreate account
near delete $ID $MasterAccount
near create-account $ID --masterAccount=$MasterAccount --initial-balance 100

# deploy contract
near deploy --wasmFile out/main.wasm --accountId $ID
near call $ID new --accountId $ID
near view $ID is_active

# copy credentials for later deploy
cp ~/.near-credentials/testnet/$ID.json ./creds