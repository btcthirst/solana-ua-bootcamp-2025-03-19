import dotenv from 'dotenv';
import { Keypair } from '@solana/web3.js';

dotenv.config();
let privateKey = process.env['SECRET_KEY'];
if (privateKey === undefined) {
  console.log('Add SECRET_KEY to .env file');
  process.exit(1);
}
const asArray = Uint8Array.from(JSON.parse(privateKey));
const keypair = Keypair.fromSecretKey(asArray);

console.log('Public Key:', keypair.publicKey.toBase58());