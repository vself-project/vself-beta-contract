// First, import some helper libraries. `shelljs` is included in the
// devDependencies of the root project, which is why it's available here. It
// makes it easy to use *NIX-style scripting (which works on Linux distros,
// macOS, and Unix systems) on Windows as well.
const sh = require('shelljs')
const contractName = process.env.CONTRACT_NAME || fs.readFileSync('./neardev/dev-account').toString();

// Testing views
sh.exec(`near view ${contractName} is_active`);
sh.exec(`near view ${contractName} get_event_data`);
sh.exec(`near view ${contractName} get_event_stats`);

console.log("..................................");
console.log("Starting event...");
sh.exec(`near call ${contractName} start_event '{"event": {
    "event_description":
    "vSelf launches a series of quests which will keep you motivated while you learn about our project and its place inside NEAR ecosystem",
  "event_name": "vSelf Onboarding Metabuild Quest",
  "finish_time": ${new Date().getTime() * 1000000 + 30 * 24 * 60 * 60 * 1000000},
  "quests": [{
      "qr_prefix_enc": "https://vself-dev.web.app/vself.apk",
      "qr_prefix_len": ${"https://vself-dev.web.app/vself.apk".length},
      "reward_description": "Welcome to the vSelf demo!",
      "reward_title": "vSelf: Welcome Badge",
      "reward_uri": "/nft1.png"
    },
    {
      "qr_prefix_enc": "You have registered in the NEAR community",
      "qr_prefix_len": ${"You have registered in the NEAR community".length},
      "reward_description": "You have registered in the NEAR community",
      "reward_title": "vSelf: NEAR User Badge",
      "reward_uri": "/nft2.png"
    },
    {
      "qr_prefix_enc": "Congrats! Now you know more about Web3",
      "qr_prefix_len": ${"Congrats! Now you know more about Web3".length},
      "reward_description": "Congrats! Now you know more about Web3",
      "reward_title": "vSelf: Early Adopter Badge",
      "reward_uri": "/nft3.png"
    },
    {
      "qr_prefix_enc": "Thank you <3 and see you soon!",
      "qr_prefix_len": ${"Thank you <3 and see you soon!".length},
      "reward_description": "Thank you <3 and see you soon!",
      "reward_title": "vSelf: Metabuidl Badge",
      "reward_uri": "/nft4.png"
    }],
  "start_time": ${new Date().getTime() * 1000000}}}' --accountId ${contractName}`);
sh.exec(`near view ${contractName} get_event_data --accountId ${contractName}`);
sh.exec(`near view ${contractName} get_event_stats --accountId ${contractName}`);

// Emulate several checkins
console.log("..................................");
console.log("Simulating event...");
sh.exec(`near call ${contractName} checkin '{"username": "sergantche.testnet", "request": "https://1" }' --accountId ${contractName} --amount 1 --gas 300000000000000`);
sh.exec(`near view ${contractName} get_event_stats`);
sh.exec(`near call ${contractName} checkin '{"username": "ilerik.testnet", "request": "Congrats! Now you know more about Web3" }' --accountId ${contractName} --amount 1 --gas 300000000000000`);
sh.exec(`near view ${contractName} get_event_stats`);
sh.exec(`near view ${contractName} get_user_balance '{"account_id": "ilerik.testnet"}'`);
sh.exec(`near view ${contractName} get_user_balance '{"account_id": "sergantche.testnet"}'`);
sh.exec(`near view ${contractName} get_actions '{"from_index": 0, "limit": 100}'`);

console.log("..................................");
console.log("Finishing event...");
sh.exec(`near call ${contractName} stop_event --accountId ${contractName}`);
// sh.exec(`near view ${contractName} get_past_events '{"from_index": 0, "limit": 100}'`);
// sh.exec(`near view ${contractName} get_actions '{"from_index": 0, "limit": 100}'`);
// sh.exec(`near view ${contractName} get_past_event_actions '{"event_id": 0, "from_index": 0, "limit": 100}'`);

// exit script with the same code as the build command
process.exit()