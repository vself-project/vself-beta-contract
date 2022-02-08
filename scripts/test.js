// First, import some helper libraries. `shelljs` is included in the
// devDependencies of the root project, which is why it's available here. It
// makes it easy to use *NIX-style scripting (which works on Linux distros,
// macOS, and Unix systems) on Windows as well.
const sh = require('shelljs')
const contractName = process.env.CONTRACT_NAME || fs.readFileSync('./neardev/dev-account').toString();

// Testing views
sh.exec(`near view ${contractName} is_active --accountId ${contractName}`);
sh.exec(`near view ${contractName} get_event_data --accountId ${contractName}`);
sh.exec(`near view ${contractName} get_event_stats --accountId ${contractName}`);

console.log("Starting event...");
sh.exec(`near call ${contractName} start_event --accountId ${contractName}`);

sh.exec(`near view ${contractName} get_event_data --accountId ${contractName}`);
sh.exec(`near view ${contractName} get_event_stats --accountId ${contractName}`);

console.log("Finishing event...");
sh.exec(`near call ${contractName} stop_event --accountId ${contractName}`);

sh.exec(`near view ${contractName} get_event_data --accountId ${contractName}`);
sh.exec(`near view ${contractName} get_event_stats --accountId ${contractName}`);

// exit script with the same code as the build command
process.exit()