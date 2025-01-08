mod networks;
mod stellar;
mod evm;
mod contract;

use clap::Parser;
use contract::{ContractComparator, ContractLoader};
use networks::{NetworkKind, Network};
use evm::EvmLoader;
use stellar::StellarLoader;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
  #[arg(long)]
  local: String,
  
  #[arg(long)]
  remote: String,
  
  #[arg(long)]
  network: String,
  
  #[arg(long)]
  rpc_url: Option<String>,
  
  #[arg(long)]
  network_passphrase: Option<String>,
  
  #[arg(long)]
  contract_path: Option<String>,
  
  #[arg(long)]
  contract_name: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let args = Args::parse();

  let network = match args.network.as_str() {
    "ethereum" => Network::ethereum(),
    "stellar" => Network::stellar(),
    name => Network::custom_evm(
      name.to_string(),
      args.rpc_url.clone(),
    ),
  };

  let rpc_url = args.rpc_url
    .or_else(|| network.default_rpc.clone())
    .ok_or("No RPC URL provided")?;

  let loader: Box<dyn ContractLoader> = match network.kind {
    NetworkKind::Evm => {
      let contract_path = args.contract_path
        .ok_or("Contract path required for EVM networks")?;
      let contract_name = args.contract_name
        .ok_or("Contract name required for EVM networks")?;
      Box::new(EvmLoader::new(contract_path, contract_name))
    }
    NetworkKind::Stellar => {
      let network_passphrase = args.network_passphrase
        .or_else(|| network.network_passphrase.clone())
        .ok_or("No network passphrase provided")?;
      Box::new(StellarLoader::new(Some(network_passphrase)))
    }
  };

  let comparator = ContractComparator::new(loader);
  match comparator.compare(&args.local, &args.remote, &rpc_url).await {
    Ok(true) => println!("✅ Contracts match!"),
    Ok(false) => {
      eprintln!("❌ Contracts do not match!");
      std::process::exit(1);
    }
    Err(e) => {
      eprintln!("Error comparing contracts: {}", e);
      std::process::exit(1);
    }
  }

  Ok(())
}


#[cfg(test)]
mod tests {
  use super::*;
  use tokio;
  use std::str::FromStr;

  const STELLAR_MAINNET_CONTRACT: &str = "CB5HA53QWBLOCD7LQOFZ4FIOSQS2ZUA7KIBZYOV6D4CPJWXIYGX2OBAC";
  const STELLAR_TESTNET_CONTRACT: &str = "CCHXQJ5YDCIRGCBUTLC5BF2V2DKHULVPTQJGD4BAHW46JQWVRQNGA2LU";
  const SEPOLIA_CONTRACT_ADDRESS: &str = "0x1B9ec5Cc45977927fe6707f2A02F51e1415f2052";
  const SEPOLIA_RPC: &str = "https://ethereum-sepolia-rpc.publicnode.com";

  async fn run_comparison(args: Args) -> Result<bool, Box<dyn std::error::Error>> {
    let network = Network::from_str(&args.network)?;
    
    let rpc_url = args.rpc_url
      .or_else(|| network.default_rpc.clone())
      .ok_or("No RPC URL provided")?;

    let network_passphrase = args.network_passphrase
      .or_else(|| network.network_passphrase.clone());

    let loader: Box<dyn ContractLoader> = match network.kind {
      NetworkKind::Evm => {
        let contract_path = args.contract_path
          .ok_or("Contract path required for EVM networks")?;
        let contract_name = args.contract_name
          .ok_or("Contract name required for EVM networks")?;
        Box::new(EvmLoader::new(contract_path, contract_name))
      }
      NetworkKind::Stellar => {
        Box::new(StellarLoader::new(network_passphrase))
      }
    };

    let comparator = ContractComparator::new(loader);
    comparator.compare(&args.local, &args.remote, &rpc_url).await
  }

  #[tokio::test]
  async fn test_compare_stellar_mainnet_contracts() {
    let args = Args {
      local: "./fixture/artifact.wasm".to_string(),
      remote: STELLAR_MAINNET_CONTRACT.to_string(),
      network: "stellar".to_string(),
      rpc_url: None,
      network_passphrase: None,
      contract_path: None,
      contract_name: None,
    };

    let result = run_comparison(args).await;
    assert!(result.is_ok(), "Failed to compare contracts: {:?}", result.err());
    assert!(result.unwrap(), "Contracts should match");
  }

  #[tokio::test]
  async fn test_compare_stellar_testnet_contracts() {
    let args = Args {
      local: "./fixture/artifact-testnet.wasm".to_string(),
      remote: STELLAR_TESTNET_CONTRACT.to_string(),
      network: "stellar-testnet".to_string(),
      rpc_url: None,
      network_passphrase: None, // Will use default from Network
      contract_path: None,
      contract_name: None,
    };

    let result = run_comparison(args).await;
    assert!(result.is_ok(), "Failed to compare contracts: {:?}", result.err());
    assert!(result.unwrap(), "Contracts should match");
  }

  #[tokio::test]
  async fn test_compare_evm_contracts() {
    let args = Args {
      local: "./fixture/artifact.json".to_string(),
      remote: SEPOLIA_CONTRACT_ADDRESS.to_string(),
      network: "sepolia".to_string(),
      rpc_url: Some(SEPOLIA_RPC.to_string()),
      network_passphrase: None,
      contract_path: Some("contracts/Box.sol".to_string()),
      contract_name: Some("Box".to_string()),
    };

    let result = run_comparison(args).await;
    assert!(result.is_ok(), "Failed to compare contracts: {:?}", result.err());
    assert!(result.unwrap(), "Contracts should match");
  }

  #[tokio::test]
  async fn test_missing_contract_info_for_evm() {
    let args = Args {
      local: "./fixture/artifact.json".to_string(),
      remote: SEPOLIA_CONTRACT_ADDRESS.to_string(),
      network: "sepolia".to_string(),
      rpc_url: Some(SEPOLIA_RPC.to_string()),
      network_passphrase: None,
      contract_path: None, // Missing required field
      contract_name: None, // Missing required field
    };

    let result = run_comparison(args).await;
    assert!(result.is_err(), "Should fail when contract info is missing");
  }

  #[tokio::test]
  async fn test_invalid_network() {
    let args = Args {
      local: "./fixture/artifact.wasm".to_string(),
      remote: "some-address".to_string(),
      network: "invalid-network".to_string(),
      rpc_url: None,
      network_passphrase: None,
      contract_path: None,
      contract_name: None,
    };

    let result = run_comparison(args).await;
    assert!(result.is_err(), "Should fail with invalid network");
  }
}