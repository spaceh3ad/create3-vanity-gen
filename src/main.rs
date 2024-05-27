use ethers::utils::{keccak256, hex};
use ethers::types::Address;
use rand::Rng;
use sha3::{Digest, Keccak256};

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

fn find_salt_with_prefix(deployer_address: &str, prefix: &str) -> (String, String) {
    let mut rng = rand::thread_rng();

    loop {
        // Generate a random salt
        let random_salt: [u8; 32] = rng.gen();
        let deployed_address = get_deployed(&random_salt, deployer_address);

        if deployed_address.starts_with(prefix) {
            return (hex::encode(random_salt), deployed_address);
        }
    }
}

fn main() {
    let deployer_address = "0x0c35464E6bfa9cBBc29e0d0ae72B329B9773d3bC";
    let prefix = "0xc0041e";
    let (salt, deployed_address) = find_salt_with_prefix(deployer_address, prefix);

    println!("Found salt: 0x{}", salt);
    println!("Deployed address: {}", deployed_address);
}
