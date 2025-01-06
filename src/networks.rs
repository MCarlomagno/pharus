pub fn get_network_defaults(network: &str) -> (Option<String>, Option<String>) {
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