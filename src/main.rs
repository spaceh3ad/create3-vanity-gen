use ethers::utils::hex;
use rand::Rng;
use sha3::{Digest, Keccak256};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;
use std::env;

const PROXY_BYTECODE_HASH: &str = "0x21c35dbe1b344a2488cf3321d6ce542f8e9f305544ff09e4993a62319a497c1f";
const FACTORY_ADDRESS: &str = "0x93FEC2C00BfE902F733B57c5a6CeeD7CD1384AE1";

fn get_deployed(salt: &[u8], deployer_address: &str) -> String {
    let prefix = "ff";
    let proxy_bytecode_hash = hex::decode(PROXY_BYTECODE_HASH.trim_start_matches("0x")).unwrap();
    let creator_address = deployer_address.trim_start_matches("0x").to_lowercase();

    // Step 1: Encode the salt with the deployer address
    let hashed_salt = {
        let mut hasher = Keccak256::new();
        hasher.update(hex::decode(creator_address.clone()).unwrap());
        hasher.update(salt);
        hasher.finalize()
    };

    // Step 2: Calculate the proxy address
    let encoded_data = {
        let mut hasher = Keccak256::new();
        hasher.update(hex::decode(prefix).unwrap());
        hasher.update(hex::decode(FACTORY_ADDRESS.trim_start_matches("0x")).unwrap());
        hasher.update(hashed_salt);
        hasher.update(proxy_bytecode_hash);
        hasher.finalize()
    };

    let proxy = hex::encode(&encoded_data[12..32]);

    // Step 3: Calculate the deployed contract address
    let rlp_encoded = {
        let mut hasher = Keccak256::new();
        hasher.update(&[0xd6, 0x94]);
        hasher.update(hex::decode(&proxy).unwrap());
        hasher.update(&[0x01]);
        hasher.finalize()
    };

    let deployed_address = hex::encode(&rlp_encoded[12..32]);

    format!("0x{}", deployed_address)
}

fn remove_0x_prefix(s: &str) -> &str {
    if s.starts_with("0x") || s.starts_with("0X") {
        &s[2..]
    } else {
        s
    }
}

fn is_valid_hex(s: &str) -> bool {
    let s = remove_0x_prefix(s);
    s.chars().all(|c| c.is_digit(16))
}

fn find_salt_with_prefix(deployer_address: &str, prefix: &str, num_threads: usize) -> (String, String) {
    let result = Arc::new(Mutex::new(None));
    let deployer_address = Arc::new(deployer_address.to_string());
    let prefix = Arc::new(prefix.to_string());

    let mut handles = vec![];

    for _ in 0..num_threads {
        let result = Arc::clone(&result);
        let deployer_address = Arc::clone(&deployer_address);
        let prefix = Arc::clone(&prefix);

        let handle = thread::spawn(move || {
            let mut rng = rand::thread_rng();
            while result.lock().unwrap().is_none() {
                // Generate a random salt
                let random_salt: [u8; 32] = rng.gen();
                let deployed_address = get_deployed(&random_salt, &deployer_address);

                if deployed_address.starts_with(&prefix[..]) {
                    let mut result = result.lock().unwrap();
                    *result = Some((hex::encode(random_salt), deployed_address));
                }
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let x = result.lock().unwrap().clone().unwrap(); x}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() -1 != 3 {
        println!("Usage: {} <num_threads> <deployer_address> <prefix> [FACTORY | PROXY_BYTECODE]", args[0]);
        return;
    }

    let start = Instant::now();
    
    let num_threads: usize =  args[1].parse().unwrap();
    let deployer_address =  &args[2];
    let prefix: &String =  &args[3];

    if !is_valid_hex(prefix) {
        eprintln!("Error: The prefix must be a valid hex string.");
        std::process::exit(1);
    }

    let (salt, deployed_address) = find_salt_with_prefix(deployer_address, prefix, num_threads);

    println!("Found salt: 0x{}", salt);
    println!("Deployed address: {}", deployed_address);
    println!("Time taken: {:?}", start.elapsed());
}