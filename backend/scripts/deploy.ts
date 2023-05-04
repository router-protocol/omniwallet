import { ethers } from "hardhat";
import { FORWARDER_ADDRESS, GATEWAY_ADDRESS,FEE_PAYER } from "./constant";
import hre from "hardhat";


async function main() {
  const chainId = hre.network.config.chainId
  if (!chainId) {
    return
  }
  const AbstractAccount = await ethers.getContractFactory("AbstractAccount");
  const contract = await AbstractAccount.deploy(GATEWAY_ADDRESS[chainId],FORWARDER_ADDRESS,FEE_PAYER);
  console.log(`Params: gateway: ${GATEWAY_ADDRESS[chainId]} and forwarder: ${FORWARDER_ADDRESS}`)
  await contract.deployed();
  console.log(`Deployed AbstractAccount Address deployed to ${contract.address}`);

  setTimeout(async function() {
    console.log("Verifying contract...")
    await hre.run("verify:verify", {
      address: contract.address,
      constructorArguments: [
        GATEWAY_ADDRESS[chainId],
        FORWARDER_ADDRESS,
        FEE_PAYER
      ],
    });
  }, 8000);

}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
