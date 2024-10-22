import wallet from "../../../rust_assig/Turbin3-wallet.json"
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createGenericFile, createSignerFromKeypair, signerIdentity } from "@metaplex-foundation/umi"
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys"
import { readFile } from "fs/promises"

// Create a devnet connection
const umi = createUmi('https://api.devnet.solana.com');

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(irysUploader());
umi.use(signerIdentity(signer));

(async () => {
    try {
        //1. Load image
        let image=await readFile("/Users/anujsrivastava/Documents/turbin3/generug.png")

        const myuri= createGenericFile(image,"rug",{
            contentType:"image/png"
        })

        const uri= await umi.uploader.upload([myuri])

        console.log("your image uri",uri);
        //2. Convert image to generic file.
        //3. Upload image

        // const image = ???

        // const [myUri] = ??? 
        // console.log("Your image URI: ", myUri);
    }
    catch(error) {
        console.log("Oops.. Something went wrong", error);
    }
})();
