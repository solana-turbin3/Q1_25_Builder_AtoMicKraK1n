import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createSignerFromKeypair, signerIdentity, generateSigner, percentAmount } from "@metaplex-foundation/umi"
import { createNft, mplTokenMetadata } from "@metaplex-foundation/mpl-token-metadata";

import wallet from "../../turbin3-wallet.json"
import base58 from "bs58";

const RPC_ENDPOINT = "https://api.devnet.solana.com";
const umi = createUmi(RPC_ENDPOINT);

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const myKeypairSigner = createSignerFromKeypair(umi, keypair);
umi.use(signerIdentity(myKeypairSigner));
umi.use(mplTokenMetadata())

const mint = generateSigner(umi);

(async () => {
    let tx = await createNft(umi, {
        mint,
        name: "BrownRug",
        uri: "https://gateway.irys.xyz/A3q3hkvh8WkMjcDDepeExo6AJVUDzXonvjvjhh26kosx", //metadata uri we need here
        sellerFeeBasisPoints: percentAmount(5),

    })
    let result = await tx.sendAndConfirm(umi);
    const signature = base58.encode(result.signature);
    
    console.log(`Succesfully Minted! Check out your TX here:\nhttps://explorer.solana.com/tx/${signature}?cluster=devnet`)

    console.log("Mint Address: ", mint.publicKey); // mint address: AbmkjBNZF3nxJt1Pdm7XdZ6gfKjy8riS7MUTSSsYSTBX

    // Transaction: https://explorer.solana.com/tx/2xhWwzXWCCjuhYgLhbUWKCNEd7thSaqUavjBRqP1jLW6nxowt69TqmuD3tm93kaVaZboJQbzff1S74ZKqTBuzQJ9?cluster=devnet
    // Mint Address: djsEf8JRE4WpL9XfGN6hepejvgqBqDi3U7UPo8x7JCk
})();