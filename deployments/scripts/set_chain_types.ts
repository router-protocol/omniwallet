import fs from "fs";
import dotenv from "dotenv";
import { exec_msg } from "./execute_msg";
import { Network, getChainInfoForNetwork } from "@routerprotocol/router-chain-sdk-ts";

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

    const voyagerSetupFilePath = "config/voyager.json";
    const voyagerSetup = JSON.parse(
        fs.readFileSync(voyagerSetupFilePath, "utf-8")
    );

    const voyagerAddr = voyagerSetup[network]["voyager"]["addr"];
    if (!voyagerAddr) {
        throw new Error("Not able to find 'voyagerAddr' in voyager Setup file");
    }

    const chainId = getChainInfoForNetwork(network).chainId;
    let set_chain_types = {
        "chain_type_info": [
            {
                "chain_id": "80001",
                "chain_type": 1
            },
            {
                "chain_id": "43113",
                "chain_type": 1
            },
            {
                "chain_id": "5",
                "chain_type": 1
            },
            {
                "chain_id": chainId,
                "chain_type": 1
            }
        ]
    };

    await exec_msg(voyagerAddr, "set_chain_types", set_chain_types);
    console.log("Setting Resources Complete");

}

main();
