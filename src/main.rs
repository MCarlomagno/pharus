mod args;
mod networks;
mod stellar;
mod evm;

use args::{CmdArgs, process_args};
use networks::Network;
use stellar::StellarClient;
use evm::EthereumClient;

#[derive(Debug)]
enum NetworkType {
    Stellar(StellarClient),
    Evm(EthereumClient),
}

impl NetworkType {
  fn from_network(network: Network) -> Self {
    match network {
      Network::Stellar => NetworkType::Stellar(stellar::StellarClient::new()),
      Network::Ethereum => NetworkType::Evm(evm::EthereumClient::new(network)),
      Network::Sepolia => NetworkType::Evm(evm::EthereumClient::new(network)),
    }
  }

  async fn compare_contracts(
    &self,
    local: String,
    remote: String,
    rpc_url: String,
    network_passphrase: Option<String>,
    contract_path: Option<String>,
    contract_name: Option<String>
  ) -> Result<bool, Box<dyn std::error::Error>> {
    match self {
      NetworkType::Stellar(loader) => {
        let local_hash = loader.load_local(local)?;
        let remote_hash = loader.load_remote(remote, rpc_url, network_passphrase).await?;
        Ok(local_hash == remote_hash)
      },
      NetworkType::Evm(loader) => {
        let contract_path = contract_path.expect("contract_path must be specified");
        let contract_name = contract_name.expect("contract_name must be specified");
        let local_hash = loader.load_local(local, contract_path, contract_name)?;
        let remote_hash = loader.load_remote(remote, rpc_url).await?;
        Ok(local_hash == remote_hash)
      },
    }
  }
}
#[tokio::main]
async fn main() {
  let CmdArgs { local, remote, network, rpc_url, network_passphrase, contract_name, contract_path} = process_args();

  if local.is_empty() || remote.is_empty() || network.is_empty() {
    eprintln!("Error: Missing required arguments");
    eprintln!("Usage: program --network <stellar|mainnet> --local <local_path_to_wasm> --remote <contract_address> [--rpc-url <rpc-url> --network_passphrase <network-passphrase> --contract_path <path/to/Contract.sol> --contract_name <Contract>]");
    std::process::exit(1);
  }

  let network = Network::from_str(&network).expect("Unsupported network");

  let rpc_url = rpc_url
    .or_else(|| network.get_default_rpc().map(String::from))
    .expect("No default rpc url found for provided netwrok, please include --rpc-url parameter");
  let network_passphrase = network_passphrase.or_else(|| network.get_network_passphrase().map(String::from));

  if network.is_evm() && (contract_name.is_none() || contract_path.is_none()) {
    eprintln!("Error: must provide --contract-path and --contract name for evm networks");
    std::process::exit(1);
  }
  let network_type = NetworkType::from_network(network);


  match network_type.compare_contracts(local, remote, rpc_url, network_passphrase, contract_path, contract_name).await {
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
    let local = String::from("./fixture/artifact.wasm");
    let remote = String::from("CB5HA53QWBLOCD7LQOFZ4FIOSQS2ZUA7KIBZYOV6D4CPJWXIYGX2OBAC");
    let network = Network::Stellar;
    let rpc_url = String::from(network.get_default_rpc().unwrap());
    let network_passphrase = Some(String::from(network.get_network_passphrase().unwrap()));
    let network_type = NetworkType::from_network(network);

    let result = network_type.compare_contracts(local, remote, rpc_url, network_passphrase, None, None).await;

    assert!(result.is_ok(), "result is ok");
    assert!(result.unwrap(), "Contracts match");
  }

  #[tokio::test]
  async fn test_compare_evm_contracts() {
    let local = String::from("./fixture/artifact.json");
    let remote = String::from("0x1B9ec5Cc45977927fe6707f2A02F51e1415f2052");
    let contract_path = Some(String::from("contracts/Box.sol"));
    let contract_name = Some(String::from("Box"));
    let network = Network::Sepolia;
    let rpc_url = String::from(network.get_default_rpc().unwrap());
    let network_type = NetworkType::from_network(network);

    let result = network_type.compare_contracts(local.clone(), remote.clone(), rpc_url, None, contract_path, contract_name).await;

    assert!(result.is_ok(), "result is ok");
    assert!(result.unwrap(), "Contracts match");
  }
}