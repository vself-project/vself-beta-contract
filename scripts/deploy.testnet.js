const sh = require('shelljs');
const { CONTRACT_NAME, MASTER_ACCOUNT} = process.env;
const DELETE_BEFORE_DEPLOY = process.env.DELETE_BEFORE_DEPLOY === "true";

// Recreate account
if (DELETE_BEFORE_DEPLOY) {
  sh.exec(`near delete ${CONTRACT_NAME} ${MASTER_ACCOUNT}`);
}
sh.exec(`near create-account ${CONTRACT_NAME} --masterAccount=${MASTER_ACCOUNT} --initial-balance 80`);

// Deploy contract
sh.exec(`near deploy --wasmFile out/main.wasm --accountId ${CONTRACT_NAME}`);
sh.exec(`near call ${CONTRACT_NAME} new --accountId ${CONTRACT_NAME}`);
sh.exec(`near view ${CONTRACT_NAME} is_active`);

// Copy credentials for later deploy
sh.exec(`cp ~/.near-credentials/testnet/${CONTRACT_NAME}.json ./creds`);