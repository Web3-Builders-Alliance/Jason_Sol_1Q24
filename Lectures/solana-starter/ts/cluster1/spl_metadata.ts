import wallet from "../cluster1/wallet/wba-wallet.json"
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { 
    createMetadataAccountV3, 
    CreateMetadataAccountV3InstructionAccounts, 
    CreateMetadataAccountV3InstructionArgs,
    DataV2Args
} from "@metaplex-foundation/mpl-token-metadata";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { PublicKey as SolanaPublicKey} from "@solana/web3.js";
//import { Buffer } from "buffer";
import { createSignerFromKeypair, signerIdentity, publicKey } from "@metaplex-foundation/umi";
import { base58, publicKey as publicKeySerializer, string } from "@metaplex-foundation/umi/serializers";
//import { fromWeb3JsPublicKey, toWeb3JsPublicKey } from '@metaplex-foundation/umi-web3js-adapters'

// Define our Mint address
const mint = publicKey("3hDSRkhukpXvhzUdHzprx1mVCVeqgUTyUXtF794pbesD")
//const mint = new SolanaPublicKey("3hDSRkhukpXvhzUdHzprx1mVCVeqgUTyUXtF794pbesD");

// Add the Token Metadata Program
const tokenMetadataProgramId = publicKey(
    "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
  );

// Create a UMI connection
const umi = createUmi('https://api.devnet.solana.com');
let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);
umi.use(signerIdentity(createSignerFromKeypair(umi, keypair)));

const seeds = [
    string({ size: "variable" }).serialize("metadata"),
    publicKeySerializer().serialize(tokenMetadataProgramId),
    publicKeySerializer().serialize(mint),
  ];
  const metadata_pda = umi.eddsa.findPda(tokenMetadataProgramId, seeds);

(async () => {
    try {
        // Start here
        let accounts: CreateMetadataAccountV3InstructionAccounts = {
            metadata: publicKey(metadata_pda),
            mint: publicKey(mint.toString()),
            mintAuthority: signer,
            payer: undefined,
            updateAuthority: keypair.publicKey,
            systemProgram: undefined,
            rent: undefined,   
        }

        let data: DataV2Args = {
            name: "WBA_Lecture2",
            symbol: "WBAL2",
            uri: "test.com",
            sellerFeeBasisPoints: 0,
            collection: null,
            creators: null,
            uses: null
        }

        let args: CreateMetadataAccountV3InstructionArgs = {
            data: data,
            isMutable: true,
            collectionDetails: null,
        }

        let tx = createMetadataAccountV3(
            umi,
            {
                ...accounts,
                ...args
            }
        )

        let result = await tx.sendAndConfirm(umi).then(r => r.signature.toString());
        console.log(result);
    } 
    catch(e) {
        console.error(`Oops, something went wrong: ${e}`)
    }
})();