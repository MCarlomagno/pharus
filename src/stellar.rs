use crate::contract::ContractLoader;
use async_trait::async_trait;
use std::error::Error;
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


pub struct StellarLoader {
    network_passphrase: Option<String>,
}

impl StellarLoader {
    pub fn new(network_passphrase: Option<String>) -> Self {
        Self { network_passphrase }
    }
}

#[async_trait]
impl ContractLoader for StellarLoader {
    async fn load_local(&self, path: &str) -> Result<String, Box<dyn Error>> {
      let wasm_bytes = fs::read(&path)?;
      Ok(hash_wasm(&wasm_bytes))
    }

    async fn load_remote(&self, contract_id: &str, rpc_url: &str) -> Result<String, Box<dyn Error>> {
      let print = Print::new(true);
      let contract_id = ContractAddress::from_str(&contract_id).ok();

      let network_args = network::Args {
        rpc_url: Some(String::from(rpc_url)),
        network_passphrase: self.network_passphrase.clone(),
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
        None => panic!("Could not load remote contract, remote must be a valid WASM contract"),
      };
    
      Ok(res)
    }
}

#[cfg(test)]
mod tests {
  use super::*;
  use tokio;

  const VALID_MAINNET_CONTRACT: &str = "CB5HA53QWBLOCD7LQOFZ4FIOSQS2ZUA7KIBZYOV6D4CPJWXIYGX2OBAC";
  const VALID_TESTNET_CONTRACT: &str = "CCHXQJ5YDCIRGCBUTLC5BF2V2DKHULVPTQJGD4BAHW46JQWVRQNGA2LU";
  const MAINNET_PASSPHRASE: &str = "Public Global Stellar Network ; September 2015";
  const TESTNET_PASSPHRASE: &str = "Test SDF Network ; September 2015";
  const MAINNET_RPC: &str = "https://mainnet.sorobanrpc.com";
  const TESTNET_RPC: &str = "https://soroban-testnet.stellar.org";

  #[tokio::test]
  async fn test_load_remote_mainnet() {
    let loader = StellarLoader::new(Some(MAINNET_PASSPHRASE.to_string()));
    
    let result = loader
      .load_remote(VALID_MAINNET_CONTRACT, MAINNET_RPC)
      .await;

    assert!(result.is_ok(), "Failed to load mainnet contract: {:?}", result.err());
  }

  #[tokio::test]
  async fn test_load_remote_testnet() {
    let loader = StellarLoader::new(Some(TESTNET_PASSPHRASE.to_string()));
    
    let result = loader
      .load_remote(VALID_TESTNET_CONTRACT, TESTNET_RPC)
      .await;

    assert!(result.is_ok(), "Failed to load testnet contract: {:?}", result.err());
  }

  #[tokio::test]
  async fn test_invalid_passphrase() {
    let loader = StellarLoader::new(Some("invalid passphrase".to_string()));
    
    let result = loader
      .load_remote(VALID_MAINNET_CONTRACT, MAINNET_RPC)
      .await;

    assert!(result.is_err(), "Expected error with invalid passphrase");
  }

  #[tokio::test]
  async fn test_invalid_rpc() {
    let loader = StellarLoader::new(Some(MAINNET_PASSPHRASE.to_string()));
    
    let result = loader
      .load_remote(
        VALID_MAINNET_CONTRACT,
        "https://invalid-rpc.example.com",
      )
      .await;

    assert!(result.is_err(), "Expected error with invalid RPC URL");
  }

  #[tokio::test]
  async fn test_load_local() {
    let loader = StellarLoader::new(None);
    
    let result = loader
      .load_local("./fixture/artifact.wasm")
      .await;
    
    assert!(result.is_ok(), "Failed to load local contract: {:?}", result.err());
  }

  #[tokio::test]
  async fn test_invalid_local_path() {
    let loader = StellarLoader::new(None);
    
    let result = loader
      .load_local("./fixture/non_existent.wasm")
      .await;
    
    assert!(result.is_err(), "Expected error with invalid local path");
  }

  #[tokio::test]
  async fn test_invalid_contract_id() {
    let loader = StellarLoader::new(Some(MAINNET_PASSPHRASE.to_string()));
    
    let result = loader
      .load_remote("invalid_contract_id", MAINNET_RPC)
      .await;

    assert!(result.is_err(), "Expected error with invalid contract ID");
  }

  #[tokio::test]
  async fn test_matching_contracts() {
    let loader = StellarLoader::new(Some(TESTNET_PASSPHRASE.to_string()));
    
    let local_result = loader
      .load_local("./fixture/artifact-testnet.wasm")
      .await;
    let remote_result = loader
      .load_remote(VALID_TESTNET_CONTRACT, TESTNET_RPC)
      .await;

    assert!(local_result.is_ok(), "Failed to load local contract");
    assert!(remote_result.is_ok(), "Failed to load remote contract");
    assert_eq!(local_result.unwrap(), remote_result.unwrap(), "Contract hashes don't match");
  }
}

