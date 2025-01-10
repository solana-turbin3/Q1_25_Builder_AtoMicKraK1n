import bs58 from 'bs58';
import prompt from 'prompt-sync';

const input = prompt();

// Function to decode base58 to wallet (binary data)
function base58ToWallet(base58: string): Uint8Array {
    return bs58.decode(base58);
}

// Function to encode wallet (binary data) to base58
function walletToBase58(wallet: Uint8Array): string {
    return bs58.encode(wallet);
}

// Main function to handle user input and conversion
function main() {
    const mode = input('Choose mode (1: base58 to wallet, 2: wallet to base58): ');

    if (mode === '1') {
        const base58 = input('Enter your base58 encoded private key: ');
        const wallet = base58ToWallet(base58);
        console.log('Decoded wallet:', wallet);
    } else if (mode === '2') {
        const walletInput = input('Enter your wallet as a comma-separated list of bytes (e.g., 34,46,55,...): ');
        const wallet = Uint8Array.from(walletInput.split(',').map(byte => parseInt(byte.trim(), 10)));
        const base58 = walletToBase58(wallet);
        console.log('Encoded base58:', base58);
    } else {
        console.log('Invalid mode selected. Please choose 1 or 2.');
    }
}

main();
