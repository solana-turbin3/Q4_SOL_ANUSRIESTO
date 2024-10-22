mod programs;



pub fn add(left: usize, right: usize) -> usize {
    left + right
}






#[cfg(test)] mod tests {
    use solana_sdk::{message::Message,signature::{Keypair, Signer}, pubkey::Pubkey}; 
    use solana_client::rpc_client::RpcClient; use solana_sdk::{signature::{ read_keypair_file} };
    const RPC_URL: &str = "https://api.devnet.solana.com";
    use solana_program::{system_instruction::transfer,system_program};
    use crate::programs::Turbin3_prereq::{Turbin3PrereqProgram, CompleteArgs,UpdateArgs};
    use bs58;
    use std::io::{self, BufRead};
    use solana_sdk::transaction::Transaction;
    use std::str::FromStr;  
    #[test]
    fn keygen() {
        let kp = Keypair::new();
        println!("You've generated a new Solana wallet: {}", kp.pubkey().to_string()); println!("");
        println!("To save your wallet, copy and paste the following into a JSON file:");
        println!("{:?}", kp.to_bytes());
    } 
    #[test]
    fn airdop() {
        // Import our keypair
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file"); 
        let client = RpcClient::new(RPC_URL); 
        match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {
            Ok(s) => {
            println!("Success! Check out your TX here:");
            println!("https://explorer.solana.com/tx/{}?cluster=devnet", s.to_string());
            },
            Err(e) => println!("Oops, something went wrong: {}", e.to_string()) };
    } 
    #[test]
    fn transfer_sol() {
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
        let to_pubkey = Pubkey::from_str("BadBSFChYywEM2jRRVni7oXSrzEMMi5AtjymH6cyXceJ").unwrap();
        let rpc_client = RpcClient::new(RPC_URL);
        
        let balance = rpc_client
        .get_balance(&keypair.pubkey())
        .expect("Failed to get balance");
        
    
        let recent_blockhash = rpc_client .get_latest_blockhash()
        .expect("Failed to get recent blockhash"); 
        let message = Message::new_with_blockhash(
        &[transfer( &keypair.pubkey(), &to_pubkey, balance,
        )], Some(&keypair.pubkey()), &recent_blockhash
        );
        let fee = rpc_client
        .get_fee_for_message(&message)
        .expect("Failed to get fee calculator");
        let transaction = Transaction::new_signed_with_payer( &[transfer(
        &keypair.pubkey(), &to_pubkey,  balance - fee,
        )], Some(&keypair.pubkey()), &vec![&keypair], recent_blockhash
        );
        let signature = rpc_client
        .send_and_confirm_transaction(&transaction)
        .expect("Failed to send transaction");
    println!("Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
        signature);
    }
    #[test]
    fn enroll(){
        // Create a Solana devnet connection
        let rpc_client = RpcClient::new(RPC_URL);

        // Define our accounts
        let signer = read_keypair_file("Turbin3-wallet.json").expect("Couldn't find wallet file");

        // Create PDA for the prereq account
        let prereq = Turbin3PrereqProgram::derive_program_address(&[
            b"prereq",
            signer.pubkey().to_bytes().as_ref(),
        ]);

        // Define our instruction data
        let args = CompleteArgs {
            github: b"anusriesto".to_vec(), // Your GitHub username
        };

        // Get recent blockhash
        let blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");
        let transaction =Turbin3PrereqProgram::complete(
        &[&signer.pubkey(), &prereq, &system_program::id()], &args,
        Some(&signer.pubkey()),
        &[&signer],
        blockhash );

        // Send the transaction
        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");

        // Print the transaction output
        println!(
            "Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
            signature
        );
    }

    #[test]
    fn base58_to_wallet() {
    println!("Input your private key as base58:");
    let stdin = io::stdin();
    let base58 = stdin.lock().lines().next().unwrap().unwrap(); println!("Your wallet file is:");
    let wallet = bs58::decode(base58).into_vec().unwrap(); println!("{:?}", wallet);
    }
    #[test]
    fn wallet_to_base58() {
    println!("Input your private key as a wallet file byte array:"); 
    let stdin = io::stdin();
    let wallet =stdin.lock().lines().next().unwrap().unwrap().trim_start_matches('[').trim_end_matches(']').
    split(',') .map(|s| s.trim().parse::<u8>().unwrap()).collect::<Vec<u8>>();
    println!("Your private key is:");
    let base58 = bs58::encode(wallet).into_string(); println!("{:?}", base58);
    }
}
