import { Keypair } from "@solana/web3.js";

const checkText = `kumar`;
let date = new Date();
const now = date.getTime();

for (;;) {
    const keypair = Keypair.generate();
    if (keypair.publicKey.toString().toLowerCase().includes(checkText)) {
    console.log(`The public key is: `, keypair.publicKey.toBase58());
    console.log(`The secret key is: `, keypair.secretKey);
    console.log(`✅ Finished search!`,checkText);
    console.log(`The time elapsed is: ${((new Date()).getTime() - now) / 1000} seconds`);
    break;
    }
    
}
