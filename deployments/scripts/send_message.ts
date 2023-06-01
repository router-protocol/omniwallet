import {
  NetworkEndpoints,
  getEndpointsForNetwork,
  Network,
  ChainRestAuthApi,
  MsgExecuteContract,
  createTransaction,
  BigNumberInBase,
  PrivateKey,
  TxRestClient,
  privateKeyToPublicKeyBase64,
  getChainInfoForNetwork,
} from "@routerprotocol/router-chain-sdk-ts";

async function sendMessage({
  contractAddr,
  action,
  data,
}: {
  contractAddr: string;
  action: string;
  data: any;
}) {
  let network: Network;

  if (process.env.ENV == "devnet") {
    network = Network.Devnet;
  } else if ((process.env.ENV = "alpha-devnet")) {
    network = Network.AlphaDevnet;
  } else if ((process.env.ENV = "testnet")) {
    network = Network.Testnet;
  } else if ((process.env.ENV = "mainnet")) {
    network = Network.Mainnet;
  } else {
    throw new Error(
      "your env does not match either devnet, alpha, testnet or mainnet"
    );
  }

  let endpoint: NetworkEndpoints = getEndpointsForNetwork(network);
  const chainId = getChainInfoForNetwork(network).chainId;

  const privateKeyHash = process.env.PRIVATE_KEY;

  if (!privateKeyHash) {
    throw new Error("Please set your PRIVATE_KEY in the .env file");
  }
  const privateKey = PrivateKey.fromPrivateKey(privateKeyHash);

  const alice = privateKey.toBech32();

  const publicKey = privateKeyToPublicKeyBase64(
    Buffer.from(privateKeyHash, "hex")
  );

  const restClient = new TxRestClient(endpoint.lcdEndpoint);

  /** Get Faucet Accounts details */
  const aliceAccount = await new ChainRestAuthApi(
    endpoint.lcdEndpoint
  ).fetchAccount(alice);

  const executeContractMsg = MsgExecuteContract.fromJSON({
    sender: alice,
    action: action,
    contractAddress: contractAddr,
    msg: data,
  });

  const { signBytes, txRaw } = createTransaction({
    message: executeContractMsg.toDirectSign(),
    memo: "",
    fee: {
      amount: [
        {
          amount: new BigNumberInBase(500000001).times(500000).toString(),
          denom: "route",
        },
      ],
      gas: (500000).toString(),
    },
    pubKey: publicKey,
    sequence: parseInt(aliceAccount.account.base_account.sequence, 10),
    accountNumber: parseInt(
      aliceAccount.account.base_account.account_number,
      10
    ),
    chainId: chainId,
  });

  /** Sign transaction */
  const signature = await privateKey.sign(signBytes);

  /** Append Signatures */
  txRaw.setSignaturesList([signature]);

  /** Broadcast transaction */
  let txxResponse = await restClient.broadcast(txRaw);
  let txResponse = await restClient.waitTxBroadcast(txxResponse.txhash);
  console.log(`txResponse =>`, txResponse);
}

export default sendMessage;
