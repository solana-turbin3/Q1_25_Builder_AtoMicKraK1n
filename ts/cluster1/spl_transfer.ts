import { Commitment, Connection, Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js"
import wallet from "../../turbin3-wallet.json"
import { getOrCreateAssociatedTokenAccount, transfer } from "@solana/spl-token";

// We're going to import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

// Mint address
const mint = new PublicKey("5oVMpgi4rBkWEcrHJB3ERqvrZMEMySWhRGjqvgPGmHzf");

// Recipient address
const to = new PublicKey("AwrrVVot6J2BHD8iQwSG8SLJA99gvA6u9u2zyqpHCKaA");

(async () => {
    try {
        // Get the token account of the fromWallet address, and if it does not exist, create it
        const fromAta = await getOrCreateAssociatedTokenAccount(connection, keypair, mint, keypair.publicKey);
        

        // Get the token account of the toWallet address, and if it does not exist, create it
        const toAta = await getOrCreateAssociatedTokenAccount(connection, keypair, mint, to);

        // Transfer the new token to the "toTokenAccount" we just created
        const tx = await transfer(connection, keypair,  fromAta.address, toAta.address, keypair, 5000000);

        console.log("Transaction Signature", tx);
    } catch(e) {
        console.error(`Oops, something went wrong: ${e}`)
    }
})();