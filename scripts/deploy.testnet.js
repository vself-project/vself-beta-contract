const sh = require("shelljs");
const {
  CONTRACT_NAME,
  MASTER_ACCOUNT,
  CONTRACT_DEPLOYMENT_COST,
  ADMIN_ACCOUNT,
  CONTRACT_CONSTRUCTOR_COST,
  CONTRACT_STORAGE_COST,
  CONTRACT_EXTRA
} = process.env;
const DELETE_BEFORE_DEPLOY = process.env.DELETE_BEFORE_DEPLOY === "true";
const CREATE_CONTRACT_ACCOUNT_BEFORE_DEPLOY = process.env.CREATE_CONTRACT_ACCOUNT_BEFORE_DEPLOY === "true";

// Contract initial balance
let initialBalance = Number(CONTRACT_DEPLOYMENT_COST) + Number(CONTRACT_STORAGE_COST) + Number(CONTRACT_CONSTRUCTOR_COST) + Number(CONTRACT_EXTRA);
console.log('Contract initial balance: ', initialBalance);

// Recreate account
if (DELETE_BEFORE_DEPLOY) {
  console.log('Recreate contract account: ', CONTRACT_NAME);
  sh.exec(`near delete ${CONTRACT_NAME} ${MASTER_ACCOUNT}`);
  sh.exec(
    `near create-account ${CONTRACT_NAME} --masterAccount=${MASTER_ACCOUNT} --initialBalance ${initialBalance}`
  );

  // Copy credentials for later deploy
  sh.exec(`cp ~/.near-credentials/testnet/${CONTRACT_NAME}.json ./creds`);
} else if (CREATE_CONTRACT_ACCOUNT_BEFORE_DEPLOY) {
  console.log('Create contract account before deploy: ', CONTRACT_NAME);
  sh.exec(
    `near create-account ${CONTRACT_NAME} --masterAccount=${MASTER_ACCOUNT} --initialBalance ${initialBalance}`
  );

  // Copy credentials for later deploy
  sh.exec(`cp ~/.near-credentials/testnet/${CONTRACT_NAME}.json ./creds`);
}

// Deploy contract
sh.exec(`near deploy --wasmFile out/main.wasm --accountId ${CONTRACT_NAME}`);
sh.exec(`near call ${CONTRACT_NAME} new --accountId ${CONTRACT_NAME}`);
sh.exec(`near view ${CONTRACT_NAME} is_active`);

// Set admin
// sh.exec(`near call ${CONTRACT_NAME} approve_admin '{"admin_id": "${ADMIN_ACCOUNT}"}' --accountId ${CONTRACT_NAME} --gas 30000000000000`);
