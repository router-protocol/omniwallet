import { SupportedChainId } from "../hardhat.config";

  export const FORWARDER_ADDRESS = "router13sp9h2p6lg6vdkayg40wnyxfqkhwr39jvq9c893dxs49e9upya0q0h9xcq"
  export const FEE_PAYER = "router1z6ralzg5tsznq9s6xmutyeen7evylcj7harhmq"
  export const GATEWAY_ADDRESS: { [key: string]: string } = {
    [SupportedChainId.FUJI]: "0xcAa6223D0d41FB27d6FC81428779751317FC24cB",
    [SupportedChainId.POLYGON_MUMBAI]: "0xcAa6223D0d41FB27d6FC81428779751317FC24cB",
  };