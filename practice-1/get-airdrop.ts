import dotenv from 'dotenv';
import { clusterApiUrl, Connection, Keypair, LAMPORTS_PER_SOL } from '@solana/web3.js';
import { airdropIfRequired } from '@solana-developers/helpers'; 

dotenv.config();
let privateKey = process.env['SECRET_KEY'];
if (privateKey === undefined) {
  console.log('Add SECRET_KEY to .env file');
  process.exit(1);
}
const asArray = Uint8Array.from(JSON.parse(privateKey));
const keypair = Keypair.fromSecretKey(asArray);

const connection = new Connection(clusterApiUrl('devnet'), 'confirmed');
console.log('Connected to Solana Devnet');

await airdropIfRequired(
    connection,
    keypair.publicKey,
    5 * LAMPORTS_PER_SOL,
    0.5 * LAMPORTS_PER_SOL
);