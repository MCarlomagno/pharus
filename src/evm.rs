use crate::contract::ContractLoader;
use async_trait::async_trait;
use std::error::Error;
use alloy_provider::{Provider, ProviderBuilder};
use alloy_primitives::Address;
use reqwest::Url;
use std::str::FromStr;

pub struct EvmLoader {
  contract_path: String,
  contract_name: String,
}

impl EvmLoader {
  pub fn new(contract_path: String, contract_name: String) -> Self {
    Self {
      contract_path,
      contract_name,
    }
  }
}

#[async_trait]
impl ContractLoader for EvmLoader {
  async fn load_local(&self, path: &str) -> Result<String, Box<dyn Error>> {
    let content = std::fs::read_to_string(path)?;
    let artifact: serde_json::Value = serde_json::from_str(&content)?;
    let bytecode = artifact["output"]["contracts"][&self.contract_path][&self.contract_name]["evm"]["deployedBytecode"]["object"].as_str()
      .ok_or("Bytecode not found in artifact")?;
    Ok(bytecode.to_string())
  }

  async fn load_remote(&self, address: &str, rpc_url: &str) -> Result<String, Box<dyn Error>> {
    let url = rpc_url.parse::<Url>()?;

    let provider = ProviderBuilder::new().on_http(url);
    let address = Address::from_str(&address)?;
    let bytecode = provider.get_code_at(address).await?;
    Ok(hex::encode(bytecode))
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use tokio;

  const VALID_CONTRACT_ADDRESS: &str = "0x1B9ec5Cc45977927fe6707f2A02F51e1415f2052";
  const SEPOLIA_RPC: &str = "https://ethereum-sepolia-rpc.publicnode.com";

  #[tokio::test]
  async fn test_load_remote() {
    let loader = EvmLoader::new(
      "contracts/Box.sol".to_string(),
      "Box".to_string(),
    );
    
    let result = loader
      .load_remote(VALID_CONTRACT_ADDRESS, SEPOLIA_RPC)
      .await;

    assert!(result.is_ok(), "Failed to load remote contract: {:?}", result.err());
  }

    #[tokio::test]
    async fn test_invalid_rpc() {
      let loader = EvmLoader::new(
        "contracts/Box.sol".to_string(),
        "Box".to_string(),
      );
      
      let result = loader
        .load_remote(
            VALID_CONTRACT_ADDRESS,
            "https://invalid-rpc.example.com",
        )
        .await;

      assert!(result.is_err(), "Expected error with invalid RPC URL");
    }

    #[tokio::test]
    async fn test_load_local() {
      let loader = EvmLoader::new(
        "contracts/Box.sol".to_string(),
        "Box".to_string(),
      );
      
      let result = loader.load_local("./fixture/artifact.json").await;
      
      assert!(result.is_ok(), "Failed to load local contract: {:?}", result.err());
    }

    #[tokio::test]
    async fn test_invalid_contract_path() {
      let loader = EvmLoader::new(
        "invalid/path.sol".to_string(),
        "InvalidContract".to_string(),
      );
      
      let result = loader.load_local("./fixtures/artifact.json").await;
      
      assert!(result.is_err(), "Expected error with invalid contract path");
    }
}
