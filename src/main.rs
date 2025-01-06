use std::str::FromStr;

use soroban_cli::commands::contract::info::shared::{fetch_wasm, Args};
use soroban_cli::config::{network, ContractAddress};
use soroban_cli::print::Print;
use sha2::{Sha256, Digest};

pub struct CmdArgs {
  local: String,
  remote: String,
  network: String,
  rpc_url: Option<String>,
  network_passphrase: Option<String>,
}

fn get_network_defaults(network: &str) -> (Option<String>, Option<String>) {
  match network.to_lowercase().as_str() {
      "stellar" => (
          Some(String::from("https://mainnet.sorobanrpc.com")),
          Some(String::from("Public Global Stellar Network ; September 2015"))
      ),
      "ethereum" => (
          Some(String::from("https://eth.llamarpc.com")),
          None
      ),
      _ => (None, None)
  }
}

fn hash_wasm(bytes: &[u8]) -> String {
  let mut hasher = Sha256::new();
  hasher.update(bytes);
  let result = hasher.finalize();
  format!("{:x}", result)  // converts to hex string
}

fn process_args() -> CmdArgs {
  let args: Vec<String> = std::env::args().collect();

  let mut local = String::new();
  let mut remote = String::new();
  let mut network = String::new(); 
  let mut rpc_url = None;
  let mut network_passphrase = None; 

  let mut i = 1;
  while i < args.len() {
    match args[i].as_str() {
      "--local" => {
        if i + 1 < args.len() {
          local = args[i + 1].clone();
          i += 2;
        }
      }
      "--remote" => {
        if i + 1 < args.len() {
          remote = args[i + 1].clone();
          i += 2;
        }
      }
      "--rpc-url" => {
        if i + 1 < args.len() {
          rpc_url = Some(args[i + 1].clone());
          i += 2;
        }
      }
      "--network-passphrase" => {
        if i + 1 < args.len() {
          network_passphrase = Some(args[i + 1].clone());
          i += 2;
        }
      }
      "--network" => {
        if i + 1 < args.len() {
            let network_arg = args[i + 1].to_lowercase();
            match network_arg.as_str() {
                "stellar" | "ethereum" => {
                    network = network_arg;
                    i += 2;
                }
                _ => {
                    eprintln!("Error: Invalid network. Must be 'stellar' or 'ethereum'");
                    std::process::exit(1);
                }
            }
        }
      }
      _ => i += 1,
    }
  }

  CmdArgs { local, remote, network, rpc_url, network_passphrase }
}

fn load_local_file(path: String) -> Result<String, Box<dyn std::error::Error>> {
  let wasm_bytes = std::fs::read(&path)?;
  let res = hash_wasm(&wasm_bytes);
  Ok(res)
}

async fn load_remote_file(network: String, contract_id: String, rpc_url: Option<String>, network_passphrase: Option<String>) -> Result<String, Box<dyn std::error::Error>> {
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
 
#[tokio::main]
async fn main() {
  let CmdArgs { local, remote, network, rpc_url, network_passphrase } = process_args();

  if local.is_empty() || remote.is_empty() || network.is_empty() {
    eprintln!("Error: Missing required arguments");
    eprintln!("Usage: program --network <stellar|mainnet> --local <local_path_to_wasm> --remote <contract_address>");
    std::process::exit(1);
  }

  let local_wasm_hash = match load_local_file(local.clone()) {
    Ok(content) => content,
    Err(e) => {
      eprintln!("Error reading WASM file: {}", e);
      std::process::exit(1);
    }
  };

  let remote_wasm_hash = match load_remote_file(network.clone(), remote.clone(), rpc_url, network_passphrase).await {
    Ok(wasm) => wasm,
    Err(e) => {
        eprintln!("Error fetching remote WASM: {}", e);
        std::process::exit(1);
    }
  };

  if local_wasm_hash == remote_wasm_hash {
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

    let local_wasm_hash = match load_local_file(local.clone()) {
        Ok(content) => content,
        Err(e) => {
            panic!("Error reading WASM file: {}", e);
        }
    };

    let remote_wasm_hash = match load_remote_file(network.clone(), remote.clone(), None, None).await {
        Ok(wasm) => wasm,
        Err(e) => {
            panic!("Error fetching remote WASM: {}", e);
        }
    };

    assert_eq!(local_wasm_hash, remote_wasm_hash, 
        "WASM hashes don't match!\nLocal: {}\nRemote: {}", 
        local_wasm_hash, remote_wasm_hash
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