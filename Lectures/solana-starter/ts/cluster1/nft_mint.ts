import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createSignerFromKeypair, signerIdentity, generateSigner, percentAmount } from "@metaplex-foundation/umi"
import { createNft, mplTokenMetadata, CreateV1InstructionDataArgs, isNonFungible, CreateV1InstructionAccounts } from "@metaplex-foundation/mpl-token-metadata";

import wallet from "../cluster1/wallet/wba-wallet.json"
import base58 from "bs58";

const RPC_ENDPOINT = "https://api.devnet.solana.com";
const umi = createUmi(RPC_ENDPOINT);

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const myKeypairSigner = createSignerFromKeypair(umi, keypair);
umi.use(signerIdentity(myKeypairSigner));
umi.use(mplTokenMetadata())

const mint = generateSigner(umi);

(async () => {
    // let prop: CreateV1InstructionDataArgs = {
    //     name: "MyFirstRug",
    //     symbol: "MFR",
    //     uri: "https://arweave.net/pXn0ramJeX20dhpvyhf7Ko9Gv5sFtMjuttVt4PnI-Pk",
    //     sellerFeeBasisPoints: percentAmount(4),
    //     creators: null,
    //     tokenStandard: 0,
    //     };
    // let tx = await createNft(umi, );

    let tx = await createNft(umi, {
        mint,
        name: "MyFirstRug",
        uri: "https://arweave.net/pXn0ramJeX20dhpvyhf7Ko9Gv5sFtMjuttVt4PnI-Pk",
        sellerFeeBasisPoints: percentAmount(4),
      });

    let result = await tx.sendAndConfirm(umi);
    const signature = base58.encode(result.signature);
    
    console.log(`Succesfully Minted! Check out your TX here:\nhttps://explorer.solana.com/tx/${signature}?cluster=devnet`)

    console.log("Mint Address: ", mint.publicKey);
})();