import { SupportedChainId } from "../hardhat.config";

  export const FORWARDER_ADDRESS = "router1qxs8gwpv6kks2ypxxd4s0730qh6y0h006xe3ldtprjxny5hlr4pqcdwz0j"
  export const FEE_PAYER = "router1z6ralzg5tsznq9s6xmutyeen7evylcj7harhmq"
  export const GATEWAY_ADDRESS: { [key: string]: string } = {
    [SupportedChainId.FUJI]: "0xcD6a879234Fc8C94ca077Baf514a5cBE84E8b3A6",
    [SupportedChainId.POLYGON_MUMBAI]: "0xb178f5CD8c3A5D65d26D9e1eE5E3694b4903D91a",
  };