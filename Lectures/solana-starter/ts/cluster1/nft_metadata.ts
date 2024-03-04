import wallet from "../cluster1/wallet/wba-wallet.json"
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createGenericFile, createSignerFromKeypair, signerIdentity } from "@metaplex-foundation/umi"
import { createBundlrUploader } from "@metaplex-foundation/umi-uploader-bundlr"
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys"

// Create a devnet connection
const umi = createUmi('https://api.devnet.solana.com');
const bundlrUploader = createBundlrUploader(umi);

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(irysUploader());
umi.use(signerIdentity(signer));

(async () => {
    try {
        // Follow this JSON structure
        // https://docs.metaplex.com/programs/token-metadata/changelog/v1.0#json-structure

        const image = 'https://arweave.net/KaTL-emOdakWKazzgHG5A8cRZZmAZUrVnh_WVFnhLdE'
        const metadata = {
            name: "MyFirstRug",
            symbol: "MFR",
            description: "This is a description",
            image: image,
            attributes: [
                {trait_type: 'Color', value: 'green'},
                {trait_type: 'Rarity', value: '1'},
                {trait_type: 'Coins', value: '1000000'},
                {trait_type: 'Claimable', value: 'Yes'}
            ],
            properties: {
                files: [
                    {
                        type: "image/png",
                        uri: "https://arweave.net/KaTL-emOdakWKazzgHG5A8cRZZmAZUrVnh_WVFnhLdE"
                    },
                ]
            },
            // creators: []
        };
        const myUri = await umi.uploader.uploadJson([metadata]); 
        console.log("Your image URI: ", myUri);
    }
    catch(error) {
        console.log("Oops.. Something went wrong", error);
    }
})();