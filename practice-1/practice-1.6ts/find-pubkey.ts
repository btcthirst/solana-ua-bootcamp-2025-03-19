import { Keypair } from "@solana/web3.js";

const checkText = `Kume`;

for (;true;) {
    const keypair = Keypair.generate();
    if (keypair.publicKey.toString().includes(checkText)) {
    console.log(`The public key is: `, keypair.publicKey.toBase58());
    console.log(`The secret key is: `, keypair.secretKey);
    console.log(`✅ Finished!`);
    break;
    }
}
