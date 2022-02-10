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
  viewMethods: ['version', 'get_user_balance', 'get_event_data'],
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

/// Logic

// Rewards array
const rewards = [
  { qr: "common", title: "Common", description: "Common Description", url: "https://vself-dev.web.app/0.jpg"},
  { qr: "uncommon", title: "Uncommon", description: "Uncommon Description", url:"https://vself-dev.web.app/1.jpg"},
];

app.get("/version", async (req, res) => {
  let result = 'None';
  
  result = await contract.version().catch( (err) => {  
    res.status(500).send();
  })

  res.json(result);
});

// Get status of current event
app.get("/status", async (req, res) => {
  let result;

  // result = 2; // Number of rewards
  result = await contract.get_event_data().catch( (err) => {  
    res.status(500).send();
  })

  res.json(result);
});

app.get("/rewards", async (req, res) => {    
  let result = 'None';
  const username = req.query.nearid.slice(1, -1);

  result = [{
    index: 0,
    got: true,
    url: rewards[0].url,
    title: rewards[0].title,
    description: rewards[0].description,
  }]

  res.json(result);
});

/// Checkin

app.get("/checkin", async (req, res) => {
  let result = 'None';
  console.log(req.query);
  const username = req.query.nearid.slice(1, -1);
  const request = req.query.qr.slice(1, -1);
  const minting_cost = "100000000000000000000000";
  
  console.log(request);
  
  result = await contract.checkin({ username, request }, 300000000000000, minting_cost ).catch( (err) => {  
    console.log(err);
    res.status(200).send();
  })
  console.log(result);

  result = result % 2;  
  let reward = rewards[result];

  res.json({
    index: result,
    got: false,
    title: reward.title,
    description: reward.description,    
  });
});

app.get("/balance", async (req, res) => {
  let result = 'None';
  let username = 'testuser';
  
  result = await contract.get_balance({ account_id: username }).catch( (err) => {  
    console.log(err);
    res.status(500).send();
  })

  res.json(result);
});

app.use(cors());
app.use(express.json());
app.options("*", cors());

server.listen(port, () => {
  console.log(`Example app listening at http://localhost:${port}`);
});
