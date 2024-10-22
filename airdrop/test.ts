import * as bs58 from 'bs58';
import {promptSync }from 'prompt-sync';

// Function to decode base58 input to wallet bytes
function base58ToWallet() {
    const prompt = promptSync();
    const base58: string = prompt('2CZ4qKZ3UNT997MAMnkUZzuGHofHvWtPCfCKnneQQ4JYVk8rcmvnVK2b9W7gdNwnHr5Kxgh5biFsUMCbQKSrHAhk');

    try {
        const wallet = bs58.decode(base58);  // Decode base58 string
        console.log('Decoded Wallet:', wallet);
    } catch (err) {
        console.error('Invalid base58 string!', err);
    }
}

// Call the function

