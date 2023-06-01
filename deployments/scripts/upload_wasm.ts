import {
    getEndpointsForNetwork,
    PrivateKey,
    privateKeyToPublicKeyBase64,
    ChainRestAuthApi,
    createTransaction,
    BigNumberInBase,
    TxRestClient,
    Network,
    MsgStoreCode,
    TxGrpcClient,
    TxClientSimulateResponse,
    getChainInfoForNetwork,
} from "@routerprotocol/router-chain-sdk-ts";
import fs from "fs";
import dotenv from "dotenv";
import { parseRawLog } from "@cosmjs/stargate/build/logs";
import { logs } from "@cosmjs/stargate";
dotenv.config();


export const upload_wasm_code = async function (network: Network, privateKeyHash: string, wasmFilePath: string): Promise<string> {
    const endpoint = getEndpointsForNetwork(network);
    
    const privateKey = PrivateKey.fromPrivateKey(privateKeyHash);
    const chainId = getChainInfoForNetwork(network).chainId;

    const alice = privateKey.toBech32();
    console.log(alice);
    const publicKey = privateKeyToPublicKeyBase64(
        Buffer.from(privateKeyHash, "hex")
    );

    const restClient = new TxRestClient(endpoint.lcdEndpoint);
    const grpcClient = new TxGrpcClient(endpoint.grpcEndpoint);
    /** Get Faucet Accounts details */
    const aliceAccount = await new ChainRestAuthApi(
        endpoint.lcdEndpoint
    ).fetchAccount(alice);
    const wasmFile = fs.readFileSync(wasmFilePath);


    const storeCodeMsg = MsgStoreCode.fromJSON({
        sender: alice,
        wasm: wasmFile,
    });
    let simulationResponse: TxClientSimulateResponse;
    {
        let { txRaw } = createTransaction({
            message: storeCodeMsg.toDirectSign(),
            memo: "",
            pubKey: publicKey,
            sequence: parseInt(aliceAccount.account.base_account.sequence, 10),
            accountNumber: parseInt(
                aliceAccount.account.base_account.account_number,
                10
            ),
            chainId: chainId,
        });

        txRaw.setSignaturesList([""]);
        simulationResponse = await grpcClient.simulate(txRaw);
    }
    let gas = parseInt((simulationResponse.gasInfo.gasUsed * 1.3).toString()).toString();
    let amount = new BigNumberInBase(500000001).times(gas).toString();
    console.log(amount, gas);

    const { signBytes, txRaw } = createTransaction({
        message: storeCodeMsg.toDirectSign(),
        memo: "",
        fee: {
            amount: [
                {
                    amount: amount,
                    denom: "route",
                },
            ],
            gas: gas,
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
    console.log(txxResponse);
    let txResponse = await restClient.waitTxBroadcast(txxResponse.txhash);
    console.log(`txResponse =>`, txResponse);
    const parsedLogs = parseRawLog(txResponse.raw_log)

    const codeIdAttr =
        typeof parsedLogs === "string"
            ? { value: "null" }
            : logs.findAttribute(parsedLogs, "store_code", "code_id");

    console.log("store code info", codeIdAttr);
    return codeIdAttr.value;
}
