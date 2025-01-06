use std::fs;  
use soroban_cli::print::Print;
use sha2::{Sha256, Digest};
use crate::networks::get_network_defaults;
use soroban_cli::config::{network, ContractAddress};
use soroban_cli::commands::contract::info::shared::{fetch_wasm, Args};
use std::str::FromStr;

pub fn hash_wasm(bytes: &[u8]) -> String {
  let mut hasher = Sha256::new();
  hasher.update(bytes);
  let result = hasher.finalize();
  format!("{:x}", result)
}

pub fn load_local_file(path: String) -> Result<String, Box<dyn std::error::Error>> {
  let wasm_bytes = fs::read(&path)?;
  Ok(hash_wasm(&wasm_bytes))
}

pub async fn load_remote_file(network: String, contract_id: String, rpc_url: Option<String>, network_passphrase: Option<String>) -> Result<String, Box<dyn std::error::Error>> {
  let print = Print::new(true);
  let contract_id = ContractAddress::from_str(&contract_id).ok();

  let (default_rpc, default_passphrase) = get_network_defaults(&network);

  let network_args = network::Args {
    rpc_url: rpc_url.or(default_rpc),
    network_passphrase: network_passphrase.or(default_passphrase),
    rpc_headers: vec![
        (String::from("Content-Type"), String::from("application/json")),
    ],
    ..Default::default()
  };

  let args = Args {
    contract_id,
    network: network_args,
    ..Default::default()
  };
  let (wasm_bytes, _, _) = fetch_wasm(&args, &print).await?;

  let res = match wasm_bytes {
    Some(bytes) => hash_wasm(&bytes),
    None => panic!("could not load remote contract"),
  };

  Ok(res)
}  