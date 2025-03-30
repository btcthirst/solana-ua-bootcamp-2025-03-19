import dotenv from 'dotenv';
import {
    Connection,
    PublicKey,
    clusterApiUrl,
    LAMPORTS_PER_SOL,
    Keypair,
} from '@solana/web3.js';

dotenv.config();
const secretKey = process.env['SECRET_KEY'];
if (!secretKey) {
    console.log('Please set your SECRET_KEY in the .env file');
    process.exit(1);
}
const asArray = Uint8Array.from(JSON.parse(secretKey));
const keypair = Keypair.fromSecretKey(asArray);

const connection = new Connection(clusterApiUrl('devnet'), 'confirmed');
console.log('Connected to Solana Devnet');

const publicKey = keypair.publicKey;

const balanceInLamports = await connection.getBalance(publicKey);
const balanceInSOL = balanceInLamports / LAMPORTS_PER_SOL;
console.log(
    `💰 The balance for the wallet at address ${publicKey} is: ${balanceInSOL}`
  );
  