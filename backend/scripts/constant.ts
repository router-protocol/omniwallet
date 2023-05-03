import { SupportedChainId } from "../hardhat.config";

  export const FORWARDER_ADDRESS = "router1e3wxju04emsjqdyca0rw6v7g8vwvpk54x8658aftgmyg3zutgmss6f0npv"

  export const GATEWAY_ADDRESS: { [key: string]: string } = {
    [SupportedChainId.FUJI]: "0xcD6a879234Fc8C94ca077Baf514a5cBE84E8b3A6",
    [SupportedChainId.POLYGON_MUMBAI]: "0xb178f5CD8c3A5D65d26D9e1eE5E3694b4903D91a",
  };