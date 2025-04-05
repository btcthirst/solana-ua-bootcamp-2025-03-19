import dotenv from "dotenv";
import { getExplorerLink } from "@solana-developers/helpers";
import {
  Connection,
  Keypair,
  PublicKey,
  clusterApiUrl,
} from "@solana/web3.js";
import { getOrCreateAssociatedTokenAccount } from "@solana/spl-token";

dotenv.config({ path: "../.env" });
let privateKey = process.env["SECRET_KEY"];
if (privateKey === undefined) {
  console.log("Add SECRET_KEY to .env!");
  process.exit(1);
}
const asArray = Uint8Array.from(JSON.parse(privateKey));
const sender = Keypair.fromSecretKey(asArray);

const connection = new Connection(clusterApiUrl("devnet"));

console.log(
  `🔑 Our pubic key is: ${sender.publicKey.toBase58()}`
);
const tokenMintAccount = new PublicKey(
    "HuvAhpotDVv1kSZ1ajuAdUAF7s1NhhBYNqrjh7sHFwSb"//"Address that create-token-mint.tx created for you"
);

let recipient_addr = process.env["RECIPIENT_ADDRESS"];
if (recipient_addr === undefined) {
    console.log("Add SECRET_KEY to .env file");
    process.exit(1);
}
const recipient = new PublicKey(recipient_addr);
  
const tokenAccount = await getOrCreateAssociatedTokenAccount(
    connection,
    sender,
    tokenMintAccount,
    recipient
);
  
console.log(`Token Account: ${tokenAccount.address.toBase58()}`);
  
const link = getExplorerLink(
    "address",
    tokenAccount.address.toBase58(),
    "devnet"
);
  
console.log(`✅ Created token account: ${link}`);