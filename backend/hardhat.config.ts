
import "@nomiclabs/hardhat-etherscan";
import "@typechain/hardhat";
import '@nomiclabs/hardhat-ethers'
import { NetworkUserConfig } from "hardhat/types";
import { resolve } from "path";
import { config as dotenvConfig } from "dotenv";

dotenvConfig({ path: resolve(__dirname, "./.env") });


export enum SupportedChainId {
  GOERLI = 5,
  POLYGON_MUMBAI = 80001,
  BSC_TESTNET = 97,
  FUJI = 43113,
}


const privateKey: string | undefined = process.env.PRIVATE_KEY;
if (!privateKey) {
  throw new Error("Please set your PRIVATE_KEY in a .env file");
}

function getChainConfig(network: SupportedChainId): NetworkUserConfig {
  let url=''
  if(network==SupportedChainId.GOERLI){
    url= "https://rpc.ankr.com/eth_goerli";
  }
  if(network==SupportedChainId.POLYGON_MUMBAI){
    url= "https://rpc.ankr.com/polygon_mumbai";
  };
  if(network==SupportedChainId.BSC_TESTNET){
    url= "https://rpc.ankr.com/bsc_testnet_chapel";
  }
  if(network==SupportedChainId.FUJI){
    url= "https://rpc.ankr.com/avalanche_fuji";
  }

 
  return {
    accounts: [`${privateKey}`],  
    chainId: network,
    url,
    // gasPrice:310_000_000_000
  };
}

const config = {
  etherscan: {
    apiKey: {
      polygonMumbai: process.env.POLYGON_ETHERSCAN_KEY,
      avalancheFujiTestnet:process.env.FUJI_ETHERSCAN_KEY,
      bscTestnet:process.env.BSC_ETHERSCAN_KEY
    }
  },
  networks: {
    goerli: getChainConfig(SupportedChainId.GOERLI),
    polygonMumbai:getChainConfig(SupportedChainId.POLYGON_MUMBAI),
    bscTestnet:getChainConfig(SupportedChainId.BSC_TESTNET),
    fuji:getChainConfig(SupportedChainId.FUJI),
  },
  paths: {
    artifacts: "./artifacts",
    cache: "./cache",
    sources: "./contracts",
    tests: "./test",
  },
  solidity: {
    version: "0.8.17",
    settings: {
      optimizer: {
        enabled: true,
        runs: 10000
      }
    }
  },
};

export default config;
