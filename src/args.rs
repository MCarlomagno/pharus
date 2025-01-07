pub struct CmdArgs {
  pub local: String,
  pub remote: String,
  pub network: String,
  pub rpc_url: Option<String>,
  pub network_passphrase: Option<String>,
  pub contract_path: Option<String>,
  pub contract_name: Option<String>,
}

pub fn process_args() -> CmdArgs {
  let args: Vec<String> = std::env::args().collect();

  let mut local = String::new();
  let mut remote = String::new();
  let mut network = String::new(); 
  let mut rpc_url = None;
  let mut network_passphrase = None; 
  let mut contract_path = None;
  let mut contract_name = None;

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
          network = args[i + 1].clone().to_lowercase();
          i += 2;
        }
      }
      "--contract-path" => {
        if i + 1 < args.len() {
          contract_path = Some(args[i + 1].clone());
          i += 2;
        }
      }
      "--contract-name" => {
        if i + 1 < args.len() {
          contract_name = Some(args[i + 1].clone());
          i += 2;
        }
      }
      _ => i += 1,
    }
  }

  CmdArgs { local, remote, network, rpc_url, network_passphrase, contract_path, contract_name }
}
