use std::str::FromStr;

use soroban_cli::commands::contract::info::shared::{fetch_wasm, Args};
use soroban_cli::config::{network, ContractAddress};
use soroban_cli::print::Print;
use sha2::{Sha256, Digest};

pub struct CmdArgs {
  local: String,
  remote: String,
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
      _ => i += 1,
    }
  }

  return CmdArgs { local, remote }
}

fn load_local_file(path: String) -> Result<String, Box<dyn std::error::Error>> {
  let wasm_bytes = std::fs::read(&path)?;
  let res = hash_wasm(&wasm_bytes);
  Ok(res)
}

async fn load_remote_file(contract_id: String) -> Result<String, Box<dyn std::error::Error>> {
  let print = Print::new(true);
  let contract_id = ContractAddress::from_str(&contract_id).ok();
  let network_args = network::Args {
    rpc_url: Some(String::from("https://mainnet.sorobanrpc.com")),
    network_passphrase: Some(String::from("Public Global Stellar Network ; September 2015")),
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
  let CmdArgs { local, remote } = process_args();

  if local.is_empty() || remote.is_empty() {
    eprintln!("Error: Missing required arguments");
    eprintln!("Usage: program --local <local_path_to_wasm> --remote <contract_address>");
    std::process::exit(1);
  }

  let local_wasm_hash = match load_local_file(local.clone()) {
    Ok(content) => content,
    Err(e) => {
      eprintln!("Error reading WASM file: {}", e);
      std::process::exit(1);
    }
  };

  let remote_wasm_hash = match load_remote_file(remote.clone()).await {
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
    let local = String::from("./fixture/test.wasm");
    let remote = String::from("CB5HA53QWBLOCD7LQOFZ4FIOSQS2ZUA7KIBZYOV6D4CPJWXIYGX2OBAC");

    let local_wasm_hash = match load_local_file(local.clone()) {
        Ok(content) => content,
        Err(e) => {
            panic!("Error reading WASM file: {}", e);
        }
    };

    let remote_wasm_hash = match load_remote_file(remote.clone()).await {
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
}