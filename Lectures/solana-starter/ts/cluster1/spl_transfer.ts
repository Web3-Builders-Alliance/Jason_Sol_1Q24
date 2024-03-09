import { Commitment, Connection, Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js"
import wallet from "../cluster1/wallet/wba-wallet.json"
import { getOrCreateAssociatedTokenAccount, transfer } from "@solana/spl-token";
import bs58 from 'bs58';

// We're going to import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

// Mint address
const mint = new PublicKey("3hDSRkhukpXvhzUdHzprx1mVCVeqgUTyUXtF794pbesD");

// Recipient address
const to = new PublicKey("FAXDZafhXmNBykiqSQtDmT3ZELDiRifzPtq9Yv4zEuvM");

(async () => {
    try {
        // // Get or create the token account for the sender (fromWallet)
        const fromWalletTokenAccount = await getOrCreateAssociatedTokenAccount(
            connection, 
            keypair,
            mint,
            keypair.publicKey);

        // // Get or create the token account for the receiver (toWallet)
        const toWalletTokenAccount = await getOrCreateAssociatedTokenAccount(
            connection,
            keypair,
            mint,
            keypair.publicKey
        );

        // // Transfer tokens from the sender to the receiver
        const transferTx = await transfer(
            connection,
            keypair,
            fromWalletTokenAccount.address,
            toWalletTokenAccount.address,
            keypair.publicKey,
            1_000_000
        );

        console.log("Transfer transaction:", transferTx);
    } catch(e) {
        console.error(`Oops, something went wrong: ${e}`)
    }
})();