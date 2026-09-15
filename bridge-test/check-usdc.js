require("dotenv").config();
const { ethers } = require("ethers");

const USDC = "0x1c7D4B196Cb0C7B01d743Fbc6116a902379C7238";

const ABI = [
  "function balanceOf(address owner) view returns (uint256)",
  "function decimals() view returns (uint8)",
  "function symbol() view returns (string)"
];

async function main() {
  const provider = new ethers.JsonRpcProvider(process.env.SEPOLIA_RPC_URL);
  const wallet = new ethers.Wallet(process.env.PRIVATE_KEY, provider);

  const usdc = new ethers.Contract(USDC, ABI, provider);

  const network = await provider.getNetwork();
  const balance = await usdc.balanceOf(wallet.address);
  const decimals = await usdc.decimals();
  const symbol = await usdc.symbol();

  console.log("Network:", network.name);
  console.log("Chain ID:", network.chainId.toString());
  console.log("Wallet:", wallet.address);
  console.log("USDC contract:", USDC);
  console.log("Token:", symbol);
  console.log("USDC balance:", ethers.formatUnits(balance, decimals));
}

main().catch(console.error);
