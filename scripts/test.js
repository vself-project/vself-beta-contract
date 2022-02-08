// First, import some helper libraries. `shelljs` is included in the
// devDependencies of the root project, which is why it's available here. It
// makes it easy to use *NIX-style scripting (which works on Linux distros,
// macOS, and Unix systems) on Windows as well.
const sh = require('shelljs')
const contractName = process.env.CONTRACT_NAME || fs.readFileSync('./neardev/dev-account').toString();

// Testing views
// sh.exec(`near view ${contractName} is_active --accountId ${contractName}`);
// sh.exec(`near view ${contractName} get_event_data --accountId ${contractName}`);
// sh.exec(`near view ${contractName} get_event_stats --accountId ${contractName}`);

console.log("Starting event...");
sh.exec(`near call ${contractName} start_event --accountId ${contractName}`);

// sh.exec(`near view ${contractName} get_event_data --accountId ${contractName}`);
// sh.exec(`near view ${contractName} get_event_stats --accountId ${contractName}`);

// Emulate several checkins
console.log("Simulating event...");
sh.exec(`near call ${contractName} checkin '{"username": "ilerik.testnet", "request": "http://1" }' --accountId ${contractName} --amount 1 --gas 300000000000000`);
sh.exec(`near view ${contractName} get_event_stats --accountId ${contractName}`);
sh.exec(`near call ${contractName} checkin '{"username": "ilerik.testnet", "request": "randomtext" }' --accountId ${contractName} --amount 1 --gas 300000000000000`);
sh.exec(`near view ${contractName} get_event_stats`);
sh.exec(`near call ${contractName} checkin '{"username": "ilerik.testnet", "request": "https://1" }' --accountId ${contractName} --amount 1 --gas 300000000000000`);
sh.exec(`near view ${contractName} get_event_stats`);
sh.exec(`near call ${contractName} checkin '{"username": "sergantche.testnet", "request": "https://1" }' --accountId ${contractName} --amount 1 --gas 300000000000000`);
sh.exec(`near view ${contractName} get_event_stats`);
sh.exec(`near view ${contractName} get_user_balance '{"account_id": "ilerik.testnet"}'`);
sh.exec(`near view ${contractName} get_user_balance '{"account_id": "sergantche.testnet"}'`);

console.log("Finishing event...");
sh.exec(`near call ${contractName} stop_event --accountId ${contractName}`);

// sh.exec(`near view ${contractName} get_event_data --accountId ${contractName}`);
// sh.exec(`near view ${contractName} get_event_stats --accountId ${contractName}`);

// exit script with the same code as the build command
process.exit()