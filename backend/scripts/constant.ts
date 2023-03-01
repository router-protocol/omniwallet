import { SupportedChainId } from "../hardhat.config";

  export const FORWARDER_ADDRESS = "router1e3wxju04emsjqdyca0rw6v7g8vwvpk54x8658aftgmyg3zutgmss6f0npv"

  export const GATEWAY_ADDRESS: { [key: string]: string } = {
    [SupportedChainId.BSC_TESTNET]: "0xb91DEECd2B217eCF49771F42Ea14884a0BC23125",
    [SupportedChainId.GOERLI]: "0x12C9A8B2e3Db12ddC411a64D6a75f47E6642f026",
    [SupportedChainId.FUJI]: "0x4886fB1D678c7598C43E05bb5F24773fA8F0Ff3e",
    [SupportedChainId.POLYGON_MUMBAI]: "0xB139915AE11f6f0ACd05C8dB85E8ED1bE1c7c17d",
  };