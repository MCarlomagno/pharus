mod args;
mod networks;
mod wasm;

use args::{CmdArgs, process_args};
use wasm::{load_local_file, load_remote_file};

#[tokio::main]
async fn main() {
  let CmdArgs { local, remote, network, rpc_url, network_passphrase } = process_args();

  if local.is_empty() || remote.is_empty() || network.is_empty() {
    eprintln!("Error: Missing required arguments");
    eprintln!("Usage: program --network <stellar|mainnet> --local <local_path_to_wasm> --remote <contract_address>");
    std::process::exit(1);
  }

  let local_hash = match load_local_file(local.clone()) {
    Ok(content) => content,
    Err(e) => {
      eprintln!("Error reading WASM file: {}", e);
      std::process::exit(1);
    }
  };

  let remote_hash = match load_remote_file(network.clone(), remote.clone(), rpc_url, network_passphrase).await {
    Ok(wasm) => wasm,
    Err(e) => {
        eprintln!("Error fetching remote WASM: {}", e);
        std::process::exit(1);
    }
  };

  if local_hash == remote_hash {
    println!("✅ WASM files match!");
  } else {
      eprintln!("❌ WASM files do not match!");
      std::process::exit(1);
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use tokio;

  #[tokio::test]
  async fn test_compare_remote_vs_local_contracts() {
    let network = String::from("stellar");
    let local = String::from("./fixture/test.wasm");
    let remote = String::from("CB5HA53QWBLOCD7LQOFZ4FIOSQS2ZUA7KIBZYOV6D4CPJWXIYGX2OBAC");

    let local_hash = match load_local_file(local.clone()) {
        Ok(content) => content,
        Err(e) => {
            panic!("Error reading WASM file: {}", e);
        }
    };

    let remote_hash = match load_remote_file(network.clone(), remote.clone(), None, None).await {
        Ok(wasm) => wasm,
        Err(e) => {
            panic!("Error fetching remote WASM: {}", e);
        }
    };

    assert_eq!(local_hash, remote_hash, 
        "WASM hashes don't match!\nLocal: {}\nRemote: {}", 
        local_hash, remote_hash
    );
  }

  #[tokio::test]
  async fn test_invalid_passphrase() {
    let network = String::from("stellar");
    let remote: String = String::from("CB5HA53QWBLOCD7LQOFZ4FIOSQS2ZUA7KIBZYOV6D4CPJWXIYGX2OBAC");
    let network_passphrase = Some(String::from("invalid passphrase"));

    let result = load_remote_file(network.clone(), remote.clone(), None, network_passphrase.clone()).await;
    assert!(result.is_err(), "Expected error with invalid passphrase");
  }

  #[tokio::test]
  async fn test_invalid_rpc() {
    let network = String::from("stellar");
    let remote: String = String::from("CB5HA53QWBLOCD7LQOFZ4FIOSQS2ZUA7KIBZYOV6D4CPJWXIYGX2OBAC");
    let rpc_url = Some(String::from("https://invalid_rpc.com"));

    let result = load_remote_file(network.clone(), remote.clone(), rpc_url.clone(), None).await;
    assert!(result.is_err(), "Expected error with invalid rpc url");
  }

}