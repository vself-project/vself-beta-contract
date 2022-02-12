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

// Logic API
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

  // Number of rewards
  result = await contract.get_event_data().catch( (err) => {  
    res.status(500).send();
  })

  res.json(result);
});

// Balance of a single player or list of NFT rewards
app.get("/rewards", async (req, res) => {    
  let result = 'None';
  let nearid = req.query.nearid;  

  if (nearid) { // If username is provided we need to return user balance
    let account_id = nearid.slice(1, -1); // Extract account id
    console.log("Account ID: ", account_id);
    result = await contract.get_user_balance({ account_id }).catch( (err) => {  
      console.log(err);
      res.status(200).send();
    }).then( balance_data => {
      console.log("Balance: ", balance_data.quests_status);
      result = balance_data.quests_status;
    })
  } else { // If it's a request and we need to return list of NFTs
    result = await contract.get_event_data().catch( (err) => {  
      console.log(err);
      res.status(200).send();
    }).then( event_data => {
      console.log(event_data);
    })
    // result = [{
    //   index: 0,
    //   got: true,
    //   url: rewards[0].url,
    //   title: rewards[0].title,
    //   description: rewards[0].description,
    // }]
  }

  res.json(result);
});

// Checkin
app.get("/checkin", async (req, res) => {
  let result = 'None';
  console.log(req.query);
  const username = req.query.nearid.slice(1, -1);
  const request = req.query.qr.slice(1, -1);
  const minting_cost = "100000000000000000000000";
  
  console.log(request);
  
  result = await contract.checkin({args: { username, request }, gas: 300000000000000, amount: minting_cost }).catch( (err) => {  
    console.log(err);
    res.status(200).send();
  })
  console.log(result);
  res.json(result);
});

app.use(cors());
app.use(express.json());
app.options("*", cors());

server.listen(port, () => {
  console.log(`Example app listening at http://localhost:${port}`);
});
