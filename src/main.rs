mod args;
mod networks;
mod stellar;
mod evm;

use args::{CmdArgs, process_args};

enum NetworkType {
  Stellar(stellar::StellarClient),
  Ethereum(evm::EthereumClient),
}

impl NetworkType {
  fn from_str(network: &str) -> Self {
      match network.to_lowercase().as_str() {
          "stellar" => NetworkType::Stellar(stellar::StellarClient::new()),
          "ethereum" => NetworkType::Ethereum(evm::EthereumClient::new()),
          _ => panic!("Unsupported network: {}", network),
      }
  }

  async fn compare_contracts(&self, local: String, remote: String, rpc_url: Option<String>, network_passphrase: Option<String>) -> Result<bool, Box<dyn std::error::Error>> {
      match self {
          NetworkType::Stellar(loader) => {
              let local_hash = loader.load_local(local)?;
              let remote_hash = loader.load_remote(remote, rpc_url, network_passphrase).await?;
              Ok(local_hash == remote_hash)
          },
          NetworkType::Ethereum(loader) => {
              let local_hash = loader.load_local(local)?;
              let remote_hash = loader.load_remote(remote, rpc_url).await?;
              Ok(local_hash == remote_hash)
          }
      }
  }
}

#[tokio::main]
async fn main() {
  let CmdArgs { local, remote, network, rpc_url, network_passphrase } = process_args();

  if local.is_empty() || remote.is_empty() || network.is_empty() {
    eprintln!("Error: Missing required arguments");
    eprintln!("Usage: program --network <stellar|mainnet> --local <local_path_to_wasm> --remote <contract_address>");
    std::process::exit(1);
  }

  let network_type = NetworkType::from_str(&network);

  match network_type.compare_contracts(local, remote, rpc_url, network_passphrase).await {
    Ok(true) => println!("✅ Contracts match!"),
    Ok(false) => {
        eprintln!("❌ Contracts do not match!");
        std::process::exit(1);
    },
    Err(e) => {
        eprintln!("Error comparing contracts: {}", e);
        std::process::exit(1);
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use tokio;

  #[tokio::test]
  async fn test_compare_stellar_contracts() {
    let local = String::from("./fixture/test.wasm");
    let remote = String::from("CB5HA53QWBLOCD7LQOFZ4FIOSQS2ZUA7KIBZYOV6D4CPJWXIYGX2OBAC");


    let network_type = NetworkType::from_str("stellar");

    let result = network_type.compare_contracts(local, remote, None, None).await;

    assert!(result.is_ok(), "result is ok");
    assert!(result.unwrap(), "Contracts match");
  }
}