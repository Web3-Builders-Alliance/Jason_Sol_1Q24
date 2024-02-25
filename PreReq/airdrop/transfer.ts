import { Transaction, SystemProgram, Connection, Keypair, LAMPORTS_PER_SOL, sendAndConfirmTransaction, PublicKey } from "@solana/web3.js"
import fs from "fs";
import wallet from "./dev-wallet.json"

const walletData = JSON.parse(fs.readFileSync("./dev-wallet.json", "utf-8"));
const from = Keypair.fromSecretKey(new Uint8Array(walletData.secretKey));

const to = new PublicKey("Bpve8rFMKGADd1bWSewvsLiX99gpRKCBuSR8u7EFMgFx");
const connection = new Connection("https://api.devnet.solana.com");

(async () => {
    try {
        const transaction1 = new Transaction().add(
            SystemProgram.transfer({
                fromPubkey: from.publicKey,
                toPubkey: to,
                lamports: LAMPORTS_PER_SOL / 100,
            })
        );

        transaction1.recentBlockhash = (
            await connection.getLatestBlockhash('confirmed')
        ).blockhash;
        transaction1.feePayer = from.publicKey;

        // Sign transaction, broadcast, and confirm
        const signature1 = await sendAndConfirmTransaction(
            connection,
            transaction1,
            [from]
        );

        console.log(`Transaction 1 successful!`);

        // Get balance of dev wallet
        const balance = await connection.getBalance(from.publicKey);

        // Create a test transaction to calculate fees
        const transaction2 = new Transaction().add(
            SystemProgram.transfer({
                fromPubkey: from.publicKey,
                toPubkey: to,
                lamports: balance,
            })
        );

        transaction2.recentBlockhash = (
            await connection.getLatestBlockhash('confirmed')
        ).blockhash;
        transaction2.feePayer = from.publicKey;

        // Calculate exact fee rate to transfer entire SOL amount out of account minus fees
        const fee = (
            await connection.getFeeForMessage(
                transaction2.compileMessage(),
                'confirmed'
            )
        ).value || 0;

        // Remove our transfer instruction to replace it
        transaction2.instructions.pop();

        // Now add the instruction back with correct amount of lamports
        transaction2.add(
            SystemProgram.transfer({
                fromPubkey: from.publicKey,
                toPubkey: to,
                lamports: balance - fee,
            })
        );

        // Sign transaction, broadcast, and confirm
        const signature2 = await sendAndConfirmTransaction(
            connection,
            transaction2,
            [from]
        );

        console.log(
            `Transaction 2 successful! Check out your TX here: https://explorer.solana.com/tx/${signature2}?cluster=devnet`
        );
    } catch (e) {
        console.error(`Oops, something went wrong: ${JSON.stringify(e)}`);
    }
})();
