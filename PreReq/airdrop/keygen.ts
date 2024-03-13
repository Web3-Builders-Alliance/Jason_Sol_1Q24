import { Keypair } from "@solana/web3.js";
import fs from "fs";
const prompt = require('prompt-sync')()

// Funzione per salvare il Keypair in un file JSON
function saveKeypairToJson(keypair: Keypair, filename: string) {
    const json = JSON.stringify({
        publicKey: keypair.publicKey.toBase58(),
        secretKey: Array.from(keypair.secretKey),
    });

    fs.writeFileSync(filename, json);
    console.log(`Wallet saved to ${filename}`);
    console.log("Public address:", keypair.publicKey.toBase58());
}

// Genera un nuovo Keypair
let kp = Keypair.generate();

// Salva il Keypair in un file JSON
saveKeypairToJson(kp, "CreatedWallet.json");



