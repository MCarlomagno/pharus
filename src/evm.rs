use alloy_provider::{Provider, ProviderBuilder};
use alloy_primitives::Address;
use reqwest::Url;
use std::str::FromStr;
use crate::networks::{get_network_defaults, Network};

pub struct EthereumClient {
  pub network: String,
}

impl EthereumClient {
    pub fn new(network: Network) -> Self {
        Self { network: String::from(network.as_str()) }
    }

    pub fn load_local(&self, path: String, contract_path: String, contract_name: String) -> Result<String, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let artifact: serde_json::Value = serde_json::from_str(&content)?;
        let bytecode = artifact["output"]["contracts"][contract_path][contract_name]["evm"]["deployedBytecode"]["object"].as_str()
            .ok_or("Bytecode not found in artifact")?;
        Ok(bytecode.to_string())
    }

    pub async fn load_remote(&self, address: String, rpc_url: Option<String>) -> Result<String, Box<dyn std::error::Error>> {
      let (default_rpc, _) = get_network_defaults(&self.network);
      let url_str = rpc_url
        .unwrap_or_else(|| default_rpc.unwrap().to_string())
        .parse::<Url>()?;

      let provider = ProviderBuilder::new().on_http(url_str);
      let address = Address::from_str(&address)?;
      let bytecode = provider.get_code_at(address).await?;
      Ok(hex::encode(bytecode))
    }
}

#[cfg(test)]
mod tests {
  use super::*;
  use tokio;

  #[tokio::test]
  async fn test_loads_remote_bytecode() {
    let client = EthereumClient::new(Network::Sepolia);
    let remote: String = String::from("0x1B9ec5Cc45977927fe6707f2A02F51e1415f2052");
    let (rpc_url, _) = get_network_defaults(&client.network);
  
    let result = client.load_remote(remote, rpc_url).await;

    assert!(result.is_ok(), "Failed to load remote client");
  }
  
  #[tokio::test]
  async fn test_invalid_rpc() {
    let client = EthereumClient::new(Network::Sepolia);
    let remote: String = String::from("0x1B9ec5Cc45977927fe6707f2A02F51e1415f2052");
    let rpc_url = Some(String::from("https://invalid_rpc.com"));
  
    let result = client.load_remote(remote, rpc_url).await;
    assert!(result.is_err(), "Expected error with invalid rpc url");
  }
}

