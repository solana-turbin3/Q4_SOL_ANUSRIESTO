import wallet from "../../../rust_assig/Turbin3-wallet.json"
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { 
    createMetadataAccountV3, 
    CreateMetadataAccountV3InstructionAccounts, 
    CreateMetadataAccountV3InstructionArgs,
    DataV2Args,
    findMetadataPda
} from "@metaplex-foundation/mpl-token-metadata";
import { createSignerFromKeypair, signerIdentity, publicKey } from "@metaplex-foundation/umi";
import bs58 from "bs58";
import { PublicKey } from "@solana/web3.js";


// Define our Mint address
const mint = publicKey("F4XwXdQqmc5xWRxFw1aNJwTAAAGNCNfHakXZk8oWKZPP")
const metadatapda=publicKey ("GithegU6vAvvm18jmBaevex8ZcNgBHU5U3ecJSbdJf8S")
// Create a UMI connection
const umi = createUmi('https://api.devnet.solana.com');
const keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);
umi.use(signerIdentity(createSignerFromKeypair(umi, keypair)));


(async () => {
    try {
        // Start here
        let accounts: CreateMetadataAccountV3InstructionAccounts = {
            mint:mint,
            mintAuthority:signer,
            updateAuthority:signer.publicKey,
        }

        let data: DataV2Args = {
            name:"mustard",
            symbol:"$stard",
            uri:" ",
            sellerFeeBasisPoints:0,
            creators:null,
            collection:null,
            uses:null
        }

        let args: CreateMetadataAccountV3InstructionArgs = {
            data:data,
            isMutable:true,
            collectionDetails:null
        }

        let tx = createMetadataAccountV3(
            umi,
            {
                ...accounts,
                ...args
            }
        )

        let result = await tx.sendAndConfirm(umi);
        console.log(bs58.encode(result.signature));
    } catch(e) {
        console.error(`Oops, something went wrong: ${e}`)
    }
})();