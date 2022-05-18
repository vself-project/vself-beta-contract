const { hash } = require('./utils');
const { extractResultBoolean } = require('./utils');
const {
  CONTRACT_NAME,
  MASTER_ACCOUNT,
  ADMIN_ACCOUNT,
} = process.env;

// First, import some helper libraries. `shelljs` is included in the
// devDependencies of the root project, which is why it's available here. It
// makes it easy to use *NIX-style scripting (which works on Linux distros,
// macOS, and Unix systems) on Windows as well.
const sh = require('shelljs');
const fs = require('fs');
const contractName = CONTRACT_NAME || fs.readFileSync('./neardev/dev-account').toString();
const masterAccount = MASTER_ACCOUNT || fs.readFileSync('./neardev/dev-account').toString();
const adminAccount = ADMIN_ACCOUNT || fs.readFileSync('./neardev/dev-account').toString();

// Test admin management
//console.log("..................................");
//console.log("Administrators set management...");
//sh.exec(`near view ${contractName} is_admin '{"admin_id": "${contractName}"}'`); // should equal true
//sh.exec(`near view ${contractName} is_admin '{"admin_id": "${adminAccount}"}'`); // should equal false
//sh.exec(`near call ${contractName} approve_admin '{"admin_id": "${adminAccount}"}' --accountId ${contractName} --gas 30000000000000`);
//sh.exec(`near view ${contractName} is_admin '{"admin_id": "${adminAccount}"}'`); // should equal true
//sh.exec(`near call ${contractName} revoke_admin '{"admin_id": "${adminAccount}"}' --accountId ${contractName} --gas 300000000000000`);
//sh.exec(`near view ${contractName} is_admin '{"admin_id": "${adminAccount}"}'`); // should equal false
//console.log("..................................");

// Check if the event has already been started and call start_event if it hasn't
let res = sh.exec(`near view ${contractName} is_active`);
if (!extractResultBoolean(res)) {
  console.log("..................................");
  console.log("Starting event...");
  sh.exec(`near call ${contractName} start_event '{"event": {
    "event_description":
    "vSelf launches a series of quests which will keep you motivated while you learn about our project and its place inside NEAR ecosystem",
    "event_name": "vSelf Onboarding Metabuild Quest",
    "finish_time": ${new Date().getTime() * 1000000 + 30 * 24 * 60 * 60 * 1000000},
    "quests": [{
      "qr_prefix_enc": "${hash('https://vself-dev.web.app/vself.apk')}",
      "qr_prefix_len": ${"https://vself-dev.web.app/vself.apk".length},
      "reward_description": "Welcome to the vSelf demo!",
      "reward_title": "vSelf: Welcome Badge",
      "reward_uri": "/nft1.png"
    },
    {
      "qr_prefix_enc": "${hash('You have registered in the NEAR community')}",
      "qr_prefix_len": ${"You have registered in the NEAR community".length},
      "reward_description": "You have registered in the NEAR community",
      "reward_title": "vSelf: NEAR User Badge",
      "reward_uri": "/nft2.png"
    },
    {
      "qr_prefix_enc": "${hash('Congrats! Now you know more about Web3')}",
      "qr_prefix_len": ${"Congrats! Now you know more about Web3".length},
      "reward_description": "Congrats! Now you know more about Web3",
      "reward_title": "vSelf: Early Adopter Badge",
      "reward_uri": "/nft3.png"
    },
    {
      "qr_prefix_enc": "${hash('Thank you <3 and see you soon!')}",
      "qr_prefix_len": ${"Thank you <3 and see you soon!".length},
      "reward_description": "Thank you <3 and see you soon!",
      "reward_title": "vSelf: Metabuidl Badge",
      "reward_uri": "/nft4.png"
    }],
  "start_time": ${new Date().getTime() * 1000000}}}' --accountId ${adminAccount}`);
}
console.log("..................................");
console.log("Get event data and stats...");
sh.exec(`near view ${contractName} get_event_data`);
sh.exec(`near view ${contractName} get_event_stats`);

// Emulate several checkins
console.log("..................................");
console.log("Simulating event...");
sh.exec(`near call ${contractName} checkin '{"username": "sergantche.testnet", "request": "Ground control to major Tom" }' --accountId ${masterAccount} --depositYocto 8470000000000000000000 --gas 300000000000000`);
sh.exec(`near call ${contractName} checkin '{"username": "ilerik.testnet", "request": "Congrats! Now you know more about Web3" }' --accountId ${masterAccount} --depositYocto 8470000000000000000000 --gas 300000000000000`);
sh.exec(`near call ${contractName} checkin '{"username": "sergantche.testnet", "request": "You have registered in the NEAR community" }' --accountId ${masterAccount} --depositYocto 8470000000000000000000 --gas 300000000000000`);
sh.exec(`near view ${contractName} get_event_stats`);
sh.exec(`near view ${contractName} get_user_balance '{"account_id": "ilerik.testnet"}'`);
sh.exec(`near view ${contractName} get_user_balance '{"account_id": "sergantche.testnet"}'`);
sh.exec(`near view ${contractName} get_actions '{"from_index": 0, "limit": 100}'`);

// Finish the event and get past events data
console.log("..................................");
console.log("Finishing event...");
sh.exec(`near call ${contractName} stop_event --accountId ${adminAccount}`);
sh.exec(`near view ${contractName} get_past_events '{"from_index": 0, "limit": 100}'`);
sh.exec(`near view ${contractName} get_actions '{"from_index": 0, "limit": 100}'`);
sh.exec(`near view ${contractName} get_past_event_actions '{"event_id": 0, "from_index": 0, "limit": 100}'`);

// exit script with the same code as the build command
process.exit()