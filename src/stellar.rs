use std::fs;  
use soroban_cli::print::Print;
use sha2::{Sha256, Digest};
use soroban_cli::config::{network, ContractAddress};
use soroban_cli::commands::contract::info::shared::{fetch_wasm, Args};
use std::str::FromStr;

pub fn hash_wasm(bytes: &[u8]) -> String {
  let mut hasher = Sha256::new();
  hasher.update(bytes);
  let result = hasher.finalize();
  format!("{:x}", result)
}

#[derive(Debug)]
pub struct StellarClient;  // or SorobanProtocol

impl StellarClient {
    pub fn new() -> Self {
        Self
    }

    pub fn load_local(&self, path: String) -> Result<String, Box<dyn std::error::Error>> {
      let wasm_bytes = fs::read(&path)?;
      Ok(hash_wasm(&wasm_bytes))
    }

    pub async fn load_remote(&self, contract_id: String, rpc_url: String, network_passphrase: Option<String>) -> Result<String, Box<dyn std::error::Error>> {
      let print = Print::new(true);
      let contract_id = ContractAddress::from_str(&contract_id).ok();

      let network_args = network::Args {
        rpc_url: Some(rpc_url),
        network_passphrase,
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
}

#[cfg(test)]
mod tests {
  use super::*;
  use tokio;
  use crate::networks::Network;

  #[tokio::test]
  async fn test_invalid_passphrase() {
    let client = StellarClient::new();
    let remote = String::from("CB5HA53QWBLOCD7LQOFZ4FIOSQS2ZUA7KIBZYOV6D4CPJWXIYGX2OBAC");
    let network_passphrase = Some(String::from("invalid passphrase"));
    let rpc_url = String::from(Network::Stellar.get_default_rpc().unwrap());

    let result = client.load_remote(remote, rpc_url, network_passphrase).await;
    assert!(result.is_err(), "Expected error with invalid passphrase");
  }
  
  #[tokio::test]
  async fn test_invalid_rpc() {
    let client = StellarClient::new();
    let remote: String = String::from("CB5HA53QWBLOCD7LQOFZ4FIOSQS2ZUA7KIBZYOV6D4CPJWXIYGX2OBAC");
    let rpc_url = String::from("https://invalid_rpc.com");
  
    let result = client.load_remote(remote, rpc_url, None).await;
    assert!(result.is_err(), "Expected error with invalid rpc url");
  }
}

