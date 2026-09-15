require("dotenv").config();
const { ethers } = require("ethers");

async function main() {
  const provider = new ethers.JsonRpcProvider(process.env.SEPOLIA_RPC_URL);
  const wallet = new ethers.Wallet(process.env.PRIVATE_KEY, provider);

  const network = await provider.getNetwork();
  const balance = await provider.getBalance(wallet.address);

  console.log("Network:", network.name);
  console.log("Chain ID:", network.chainId.toString());
  console.log("Wallet:", wallet.address);
  console.log("ETH:", ethers.formatEther(balance));
}

main().catch(console.error);
