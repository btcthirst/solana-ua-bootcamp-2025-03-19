import dotenv from "dotenv";
import { Connection, Keypair, PublicKey, clusterApiUrl } from "@solana/web3.js";
import { mintTo } from "@solana/spl-token";
import { getExplorerLink } from "@solana-developers/helpers";

dotenv.config({ path: "../.env" });
let privateKey = process.env["SECRET_KEY"];
if (privateKey === undefined) {
  console.log("Add SECRET_KEY to .env!");
  process.exit(1);
}
const asArray = Uint8Array.from(JSON.parse(privateKey));
const sender = Keypair.fromSecretKey(asArray);

const connection = new Connection(clusterApiUrl("devnet"));

// Our token has two decimal places
const MINOR_UNITS_PER_MAJOR_UNITS = Math.pow(10, 2);

const tokenMintAccount = new PublicKey(
  "HuvAhpotDVv1kSZ1ajuAdUAF7s1NhhBYNqrjh7sHFwSb"//"Address that create-token-mint.ts created for you"
);
const recipientAssociatedTokenAccount = new PublicKey(
    "2QcK5WeWEbRWACEGVgUwXEhdSB7FTpsfC1eiqQwm3gCG"//"Address that create-token-account.ts created"
  );
  
  const transactionSignature = await mintTo(
    connection,
    sender,
    tokenMintAccount,
    recipientAssociatedTokenAccount,
    sender,
    10 * MINOR_UNITS_PER_MAJOR_UNITS
  );
  
  const link = getExplorerLink("transaction", transactionSignature, "devnet");
  
  console.log("✅ Success!");
  console.log(`Mint Token Transaction: ${link}`);