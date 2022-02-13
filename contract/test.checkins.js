// First, import some helper libraries. `shelljs` is included in the
// devDependencies of the root project, which is why it's available here. It
// makes it easy to use *NIX-style scripting (which works on Linux distros,
// macOS, and Unix systems) on Windows as well.
const sh = require('shelljs')
const contractName = process.env.CONTRACT_NAME || fs.readFileSync('./neardev/dev-account').toString();

// Emulate several checkins
console.log("Simulating event...");
sh.exec(`near call ${contractName} checkin '{"username": "meta_irony.testnet", "request": "http://1" }' --accountId ${contractName} --amount 1 --gas 300000000000000`);
sh.exec(`near call ${contractName} checkin '{"username": "ilerik.testnet", "request": "http://1" }' --accountId ${contractName} --amount 1 --gas 300000000000000`);
sh.exec(`near call ${contractName} checkin '{"username": "ilerik.testnet", "request": "randomtext" }' --accountId ${contractName} --amount 1 --gas 300000000000000`);
sh.exec(`near call ${contractName} checkin '{"username": "ilerik.testnet", "request": "https://1" }' --accountId ${contractName} --amount 1 --gas 300000000000000`);
sh.exec(`near call ${contractName} checkin '{"username": "sergantche.testnet", "request": "https://1" }' --accountId ${contractName} --amount 1 --gas 300000000000000`);

// exit script with the same code as the build command
process.exit()