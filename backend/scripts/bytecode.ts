import { ethers } from "hardhat";
import { FEE_PAYER, FORWARDER_ADDRESS, GATEWAY_ADDRESS } from "./constant";
import hre from "hardhat";


async function main() {


  const chainId = hre.network.config.chainId
  if (!chainId) {
    console.log("chain Id batao")
    return
  }
  const AbstractAccount = await ethers.getContractFactory("AbstractAccount");
  const contractByteCode = AbstractAccount.bytecode
  let ConstuctorParams = ethers.utils.defaultAbiCoder.encode(["address", "string", "string"], [GATEWAY_ADDRESS[chainId], FORWARDER_ADDRESS, FEE_PAYER])
  let DeployedBytecode = `${contractByteCode}${ConstuctorParams}`

  console.log("ConstuctorParams: ", GATEWAY_ADDRESS[chainId], FORWARDER_ADDRESS, FEE_PAYER)
  console.log(`Deployed AbstractAccount ConstuctorParams chainId ${chainId}: ${ConstuctorParams}`);
  console.log(`Deployed AbstractAccount contractByteCode chainId ${chainId}: ${contractByteCode}`);
  // console.log(`Deployed AbstractAccount bytecode chainId ${chainId}: ${DeployedBytecode}`);

}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
