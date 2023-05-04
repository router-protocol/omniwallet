import { ethers } from "hardhat";
import { FORWARDER_ADDRESS, GATEWAY_ADDRESS } from "./constant";
import hre from "hardhat";


async function main() {

   
  const chainId = hre.network.config.chainId
  if (!chainId) {
    return
  }
  const AbstractAccount = await ethers.getContractFactory("AbstractAccount");
  const contractByteCode = AbstractAccount.bytecode
  let ConstuctorParams = ethers.utils.defaultAbiCoder.encode(["address","string"],[GATEWAY_ADDRESS[chainId],FORWARDER_ADDRESS])
  let DeployedBytecode = `${contractByteCode}${ConstuctorParams}`
  

  console.log(`Deployed AbstractAccount ConstuctorParams chainId ${chainId}: ${ConstuctorParams}`);
  console.log(`Deployed AbstractAccount contractByteCode chainId ${chainId}: ${contractByteCode}`);
  console.log(`Deployed AbstractAccount bytecode chainId ${chainId}: ${DeployedBytecode}`);

}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
