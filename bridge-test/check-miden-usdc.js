require("dotenv").config();
const { ethers } = require("ethers");

const TOKEN = "0x2BB4FfD7E2c6D432b697554Efd77fA13bdbefd69";

const ABI = [
  "function balanceOf(address owner) view returns (uint256)",
  "function decimals() view returns (uint8)",
  "function symbol() view returns (string)",
  "function name() view returns (string)"
];

async function main() {
  const provider = new ethers.JsonRpcProvider(process.env.SEPOLIA_RPC_URL);
  const wallet = new ethers.Wallet(process.env.PRIVATE_KEY, provider);

  const token = new ethers.Contract(TOKEN, ABI, provider);

  const network = await provider.getNetwork();

  const [balance, decimals, symbol, name] = await Promise.all([
    token.balanceOf(wallet.address),
    token.decimals(),
    token.symbol(),
    token.name()
  ]);

  console.log("Network:", network.name);
  console.log("Chain ID:", network.chainId.toString());
  console.log("Wallet:", wallet.address);
  console.log("");
  console.log("TOKEN:", TOKEN);
  console.log("Name:", name);
  console.log("Symbol:", symbol);
  console.log("Decimals:", decimals);
  console.log("Balance:", ethers.formatUnits(balance, decimals));
}

main().catch(console.error);
