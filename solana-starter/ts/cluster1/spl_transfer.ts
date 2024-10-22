import { Commitment, Connection, Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js"
import wallet from "../../../rust_assig/Turbin3-wallet.json"
import { createTransferCheckedWithFeeAndTransferHookInstruction, getOrCreateAssociatedTokenAccount, transfer } from "@solana/spl-token";

// We're going to import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);
const token_decimals = 1_000_000n;
// Mint address
const mint = new PublicKey("F4XwXdQqmc5xWRxFw1aNJwTAAAGNCNfHakXZk8oWKZPP");
const programID=new PublicKey("HC2oqz2p6DEWfrahenqdq2moUcga9c9biqRBcdK3XKU1")

// Recipient address
const to = new PublicKey("215kZRrNL9M4f4yvhi5zz4ftTsPDNjXrxJM1s5mMVa8F");

// (async () => {
//     try {
//         // Get the token account of the fromWallet address, and if it does not exist, create it
//         const fromwallet=await getOrCreateAssociatedTokenAccount(connection,keypair,mint,keypair.publicKey)

//         // Get the token account of the toWallet address, and if it does not exist, create it
//         const toWallet= await getOrCreateAssociatedTokenAccount(connection,keypair,mint,to)

//         // Transfer the new token to the "toTokenAccount" we just created
//         const transfer_tok= await transfer(connection,keypair,fromwallet.address,toWallet.address,keypair.publicKey,1_00n*token_decimals)

//         console.log(`Your mint txid: ${transfer_tok}`);
//     } catch(e) {
//         console.error(`Oops, something went wrong: ${e}`)
//     }
// })();

//lets send 10 tokens to everyone


async function createAssociatedTokenAccountForAddress(owner: PublicKey) {
    try{
        const toWallet= getOrCreateAssociatedTokenAccount(connection,keypair,mint,owner,true)
        console.log(`Created/found ATA for owner ${owner.toBase58()}: ${toWallet}`);
        return toWallet;
    }
    catch(e){
        console.error('error fetching ata of address')
    }
}

async function transferspltokens(destinationAta:PublicKey,amount:number){
    try {
        const fromwallet=await getOrCreateAssociatedTokenAccount(connection,keypair,mint,keypair.publicKey)
        // Create the transfer instruction
        const transferIx = transfer(
            connection,
            keypair,
            fromwallet.address,
            destinationAta,    
            keypair.publicKey,        
            amount,                           
        );

        

        console.log(`Your mint txid: ${transferIx}`);
    } catch (error) {
        console.error(`Failed to transfer tokens to ${destinationAta.toBase58()}:`, error);
    }
}
async function fetchProgramAccountsAndCreateATAs() {
    try {
        
        const accounts = await connection.getProgramAccounts(programID);

        console.log(`Found ${accounts.length} accounts that interacted with program ${programID.toString()}`);
        const transferAmount = 10 * 10**6;

        for (const account of accounts) {
            const owner = account.pubkey;
            const destinationAta = await createAssociatedTokenAccountForAddress(owner);
            if (destinationAta) {
                await transferspltokens(destinationAta.address, transferAmount);
            }

        }
    } catch (error) {
        console.error("Error fetching program accounts or creating ATAs:", error);
    }
}
fetchProgramAccountsAndCreateATAs();