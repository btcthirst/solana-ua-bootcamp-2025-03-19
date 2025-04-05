import dotenv from 'dotenv';
import {
    Keypair,
    Connection,
    Transaction,
    SystemProgram,
    sendAndConfirmTransaction,
    LAMPORTS_PER_SOL,
    clusterApiUrl,
    PublicKey,
    TransactionInstruction
} from '@solana/web3.js';
import { createMemoInstruction } from '@solana/spl-memo';

dotenv.config({path: '../.env'});
let privateKey = process.env["SECRET_KEY"];
if (privateKey === undefined) {
    console.log("Add SECRET_KEY to .env file");
    process.exit(1);
}

const asArray = Uint8Array.from(JSON.parse(privateKey));
const sender = Keypair.fromSecretKey(asArray);

const connection = new Connection(clusterApiUrl('devnet'));

console.log(`🔑 Our public key is: ${sender.publicKey.toBase58()}`);

let recipient_addr = process.env["RECIPIENT_ADDRESS"];
if (recipient_addr === undefined) {
    console.log("Add SECRET_KEY to .env file");
    process.exit(1);
}
const recipient = new PublicKey(recipient_addr);
console.log(`💸 Attempting to send 0.01 SOL to ${recipient.toBase58()}...`)

const transaction = new Transaction()

const sendSOLInstructions = SystemProgram.transfer({
    fromPubkey: sender.publicKey,
    toPubkey: recipient,
    lamports: 0.01 * LAMPORTS_PER_SOL,
});

transaction.add(sendSOLInstructions);

const signature = await sendAndConfirmTransaction(
    connection,
    transaction,
    [sender]);

console.log(`✅ Transaction successful with signature: ${signature}`);
  
  const memoText = `My new memo text!`;

  const addMemoInstruction = createMemoInstruction(
    memoText,
    [sender.publicKey],
  )
  
  transaction.add(addMemoInstruction);
  
  console.log(`📝 memo is: ${memoText}`);