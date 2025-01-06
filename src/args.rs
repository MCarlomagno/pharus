pub struct CmdArgs {
  pub local: String,
  pub remote: String,
  pub network: String,
  pub rpc_url: Option<String>,
  pub network_passphrase: Option<String>,
}

pub fn process_args() -> CmdArgs {
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
