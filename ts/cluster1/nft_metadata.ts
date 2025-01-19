import wallet from "../../turbin3-wallet.json"
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

        const image = "https://gateway.irys.xyz/7VCYMJk2z2a8v8i833yrKHD1oBWtc95hqxKq1WXwFjcS";
        const metadata = {
            name: "BrownRug",
            symbol: "BRUG",
            description: "A brown rug",
            image,
            attributes: [
                {trait_type: 'special', value: '5'}
            ],
            properties: {
                files: [
                    {
                        type: "image/png",
                        uri: image
                    },
                ]
            },
            creators: []
        };
        const myUri = await umi.uploader.uploadJson(metadata);
        console.log("Your metadata URI: ", myUri); //A3q3hkvh8WkMjcDDepeExo6AJVUDzXonvjvjhh26kosx
    }
    catch(error) {
        console.log("Oops.. Something went wrong", error);
    }
})();
