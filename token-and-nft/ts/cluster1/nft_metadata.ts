import wallet from "./wallet/wba-wallet.json";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createGenericFile, createSignerFromKeypair, signerIdentity } from "@metaplex-foundation/umi"
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys"

// Create a devnet connection
const umi = createUmi('https://api.devnet.solana.com');

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(irysUploader());
umi.use(signerIdentity(signer));

(async () => {
    try {
        // Follow this JSON structure
        // https://docs.metaplex.com/programs/token-metadata/changelog/v1.0#json-structure

        const image = "https://devnet.irys.xyz/9QmAyMrzxJnosuij7vKv3aEmDo2jxTSmZE65eTfAwKji"
        const metadata = {
            name: "RRUG",
            symbol: "RRG",
            description: "MAGIC RUG",
            image,
            attributes: [
                {trait_type: 'Color', value: 'Multi'},
                {trait_type: 'Size', value: '90'},
                
            ],
            properties: {
                files: [
                    {
                        type: "image/jpg",
                        uri: image
                    },
                ]
            },
            creators: []
        };
        
        const myUri = await umi.uploader.uploadJson(metadata);
        console.log("Your image URI: ", myUri);
    }
    catch(error) {
        console.log("Oops.. Something went wrong", error);
    }
})();