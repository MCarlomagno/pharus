use base64::Engine;

fn process_args() -> (String, String, String) {
  let args: Vec<String> = std::env::args().collect();

  let mut network = String::new();
  let mut local = String::new();
  let mut remote = String::new();

  let mut i = 1;
  while i < args.len() {
    match args[i].as_str() {
      "--network" => {
        if i + 1 < args.len() {
          network = args[i + 1].clone();
          i += 2;
        }
      }
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

  (network, local, remote)
}

fn load_local_file(path: String) -> Result<String, Box<dyn std::error::Error>> {
  let wasm_bytes = std::fs::read(&path)?;
  String::from_utf8(wasm_bytes).map_err(|e| e.into())
}

// fn load_remote_file(contract_id: String) -> Result<String, std::error::Error> {
//   // TODO: use stellar sdk to load on-chain wasm code
// }
 
fn main() {
  let (network, local, remote) = process_args();

  if network.is_empty() || local.is_empty() || remote.is_empty() {
    eprintln!("Error: Missing required arguments");
    eprintln!("Usage: program --network <network> --local <local_path_to_wasm> --remote <contract_address>");
    std::process::exit(1);
  }

  let wasm_string = match load_local_file(local.clone()) {
    Ok(content) => content,
    Err(e) => {
      eprintln!("Error reading WASM file: {}", e);
      std::process::exit(1);
    }
  };

  println!("Network: {}", network);
  println!("Local path: {}", local);
  println!("Remote: {}", remote);
}
