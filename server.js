const express = require("express");
const http = require("http");
const nacl = require("tweetnacl");
nacl.util = require("tweetnacl-util");
const cors = require("cors");
const fs = require("fs");
const nearAPI = require("near-api-js");
const getConfig = require("./config/near");

// Instantiate server with WS support
const app = express();
const port = process.env.PORT || 8080;
const server = http.createServer(app);
const io = require("socket.io")(server, {
  cors: true,
  origins: ["*"],
});

const nearConfig = getConfig(process.env.APP_ENV || 'development')
const { nodeUrl, networkId, contractName } = nearConfig;
//const contractName = process.env.CONTRACT_NAME;
//const contractName = "beta_v4.ilerik.testnet"
const contractMethods = 
{
  changeMethods: [ 'checkin' ],
  viewMethods: ['version', 'get_user_balance_extra', 'get_event_data'],
}

const {
  keyStores: { InMemoryKeyStore },
  Near,
  Account,
  Contract,
  KeyPair,
  utils: {
    format: { parseNearAmount },
  },
} = nearAPI;

// Load credentials
console.log(
  "Loading Credentials:\n",
  `./creds/${contractName}.json`
  //`${process.env.HOME}/.near-credentials/${networkId}/${contractName}.json`
);
const credentials = JSON.parse(
  fs.readFileSync(
    `./creds/${contractName}.json`
    //`${process.env.HOME}/.near-credentials/${networkId}/${contractName}.json`
  )
);

const keyStore = new InMemoryKeyStore();
keyStore.setKey(
  networkId,
  contractName,
  KeyPair.fromString(credentials.private_key)
);
const near = new Near({
  networkId,
  nodeUrl,
  deps: { keyStore },
});
const { connection } = near;
const contractAccount = new Account(connection, contractName);
contractAccount.addAccessKey = (publicKey) =>
  contractAccount.addKey(
    publicKey,
    contractName,
    contractMethods.changeMethods,
    parseNearAmount("0.1")
  );

const contract = new Contract(contractAccount, contractName, contractMethods);

// Logic API
app.get("/version", async (req, res) => {
  let result = await contract.version().catch( (err) => {  
    res.status(500).send();
  })
  res.json(result);
});

// Get status of current event
app.get("/status", async (req, res) => {
  let result;
  // Number of rewards (0 - for no event)
  result = await contract.get_event_data().catch( (err) => {  
    res.json(0);
  });
  res.json(result.quests.length);
});

// Balance of a single player or list of NFT rewards
app.get("/rewards", async (req, res) => {    
  let result = [];
  let nearid = req.query.nearid;  
  
  await contract.get_event_data().catch( (err) => {  
    console.log(err);
    res.status(200).send();
  }).then( event_data => {
    console.log("Event Data: ", event_data);
    result = event_data.quests.map(quest => quest.reward_url);  
  })    

  res.json(result);
});

// Balance of a single player or list of NFT rewards
app.get("/balance", async (req, res) => {    
  let result = 'None';
  let nearid = req.query.nearid;  

  if (nearid) { // If username is provided we need to return user balance
    let account_id = nearid.slice(1, -1); // Extract account id
    console.log("Account ID: ", account_id);
    await contract.get_user_balance_extra({ account_id }).catch( (err) => {  
      console.log(err);
      res.status(200).send();
    }).then( balance_data => {
      console.log("Balance: ", balance_data);
      result = balance_data;
    })
  } else { // If it's a request and we need to return list of NFTs
    await contract.get_event_data().catch( (err) => {  
      console.log(err);
      res.status(200).send();
    }).then( event_data => {
      console.log("Event Data: ", event_data);
      result = event_data.quests.map(quest => quest.reward_url);
      console.log(result);
    })    
  }

  res.json(result);
});


// Checkin
app.get("/checkin", async (req, res) => {
  let result = 'None';
  const username = req.query.nearid.slice(1, -1);
  const request = req.query.qr.slice(1, -1);
  const gas_cost = 300000000000000;
  const minting_cost = "100000000000000000000000";  
  console.log("Incoming action: {} {}", username, request);
  
  result = await contract.checkin({args: { username, request }, gas: gas_cost, amount: minting_cost })
  .catch( (err) => {  
    console.log(err);
    res.status(200).send();
  })
  console.log(result);
  res.json(result);
});

// Spin up server
app.use(cors());
app.use(express.json());
app.options("*", cors());

server.listen(port, () => {
  console.log(`Example app listening at http://localhost:${port}`);
});
