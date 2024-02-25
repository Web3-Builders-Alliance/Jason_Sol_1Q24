import { Connection, Keypair, LAMPORTS_PER_SOL } from "@solana/web3.js"
import fs from "fs";

const connection = new Connection("https://api.devnet.solana.com");
const walletData = JSON.parse(fs.readFileSync("./dev-wallet.json", "utf-8"));
const keypair = Keypair.fromSecretKey(new Uint8Array(walletData.secretKey));


(async () => {
    try {
    // We're going to claim 2 devnet SOL tokens
    const txhash = await connection.requestAirdrop(keypair.publicKey, 2 * LAMPORTS_PER_SOL);
    console.log(`Success! Check out your TX here: https://explorer.solana.com/tx/${txhash}?cluster=devnet`);
    } catch(e) { console.error(`Oops, something went wrong: ${e}`) }
    })();
