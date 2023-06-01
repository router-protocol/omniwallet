echo "Starting OmniChain Wasm Deployment"
echo "current directory => $PWD"

echo "Changing current directory to crosschain-deployer"
cd ../../crosschain-deployer/rust/
sh scripts/build.sh
cp artifacts/router_crosschain_deployer-aarch64.wasm  ../../omniwallet/middleware/artifacts

cd ../../omniwallet/middleware
echo "current directory => $PWD"

sh scripts/build.sh

cd ../deployments/
echo "current directory => $PWD"

npx ts-node scripts/init.ts 
