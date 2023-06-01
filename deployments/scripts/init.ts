import fs from "fs";
import dotenv from "dotenv";
import { init_wasm_code } from "./instantiate_msg";
import { upload_wasm_code } from "./upload_wasm";
import { Network, PrivateKey } from "@routerprotocol/router-chain-sdk-ts";
dotenv.config();

async function main() {
  let network = Network.AlphaDevnet;
  if (process.env.ENV == "devnet") {
    network = Network.Devnet;
  } else if (process.env.ENV == "testnet") {
    network = Network.Testnet;
  } else if (process.env.ENV == "mainnet") {
    network = Network.Mainnet;
  } else if (process.env.ENV && process.env.ENV != "alpha-devnet") {
    throw new Error("Please set your NETWORK in the .env file");
  }

  const privateKeyHash = process.env.PRIVATE_KEY;

  if (!privateKeyHash) {
    throw new Error("Please set your PRIVATE_KEY in the .env file");
  }
  const privateKey = PrivateKey.fromPrivateKey(privateKeyHash);
  const owner = privateKey.toBech32();

  let wasmSuffix = ".wasm";
  if (process.env.IS_APPLE_CHIPSET == "YES" ) {
    wasmSuffix = "-aarch64.wasm"
  }
  const omniChainFilePath = "config/omniChain.json";
  const omniChainSetup = JSON.parse(
    fs.readFileSync(omniChainFilePath, "utf-8")
  );
  console.log("Present Deployment Details -> ", omniChainSetup[network]);

  const crossChainDeployerCodeId = await upload_wasm_code(
    network,
    privateKeyHash,
    "../middleware/artifacts/router_crosschain_deployer-aarch64.wasm"
  );

  const forwarderCodeId = await upload_wasm_code(
    network,
    privateKeyHash,
    "../middleware/artifacts/forwarder" + wasmSuffix
  );
  const forwarderDeployerCodeId = await upload_wasm_code(
    network,
    privateKeyHash,
    "../middleware/artifacts/forwarder_deployer" + wasmSuffix
  );
  
  const crossChainDeployerInitMsg = JSON.stringify({
    owner: owner,
  });
  console.log("deploying the Forwarder Deployer")
  const crossChainDeployerAddr = await init_wasm_code(
    crossChainDeployerCodeId,
    "Cross Chain Deployer",
    crossChainDeployerInitMsg
  );
  console.log("crossChainDeployerAddr", crossChainDeployerAddr);

  const forwarderDeployerInitMsg = JSON.stringify({
    deployer: crossChainDeployerAddr,
    code_id: parseInt(forwarderCodeId)
  });
  
  console.log("deploying the Forwarder Deployer")
  const forwarderDeployerAddr = await init_wasm_code(
    forwarderDeployerCodeId,
    "Forwarder Deployer",
    forwarderDeployerInitMsg
  );
  console.log("forwarderDeployerAddr", forwarderDeployerAddr);


  console.log("admin ->", owner);
  console.log(
    "CrossChain Deployer -> code_id-",
    crossChainDeployerCodeId,
    "addr-",
    crossChainDeployerAddr
  );
  console.log(
    "Forwarder -> code_id-",
    forwarderCodeId,
    "addr-",
    ""
  );
  console.log(
    "ForwarderDeployer -> code_id-",
    forwarderDeployerCodeId,
    "addr-",
    forwarderDeployerAddr
  );
  
  if (!omniChainSetup[network]) {
    omniChainSetup[network] = {};
  }
  omniChainSetup[network]["crossChainDeployer"] = {
    addr: crossChainDeployerAddr,
    code_id: crossChainDeployerCodeId,
  };
  omniChainSetup[network]["forwarder"] = {
    addr: "",
    code_id: forwarderCodeId,
  };
  omniChainSetup[network]["forwarder-deployer"] = {
    addr: forwarderDeployerAddr,
    code_id: forwarderDeployerCodeId,
  };
  fs.writeFileSync(omniChainFilePath, JSON.stringify(omniChainSetup));
}

main();
