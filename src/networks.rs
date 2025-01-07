pub enum Network {
    Stellar,
    Ethereum,
    Sepolia,
}

impl Network {
    pub fn as_str(&self) -> &'static str {
        match self {
            Network::Stellar => "stellar",
            Network::Ethereum => "ethereum",
            Network::Sepolia => "sepolia",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "stellar" => Some(Network::Stellar),
            "ethereum" => Some(Network::Ethereum),
            "sepolia" => Some(Network::Sepolia),
            _ => None,
        }
    }

    pub fn is_evm(&self) -> bool {
        matches!(self.as_str().to_lowercase().as_str(), "ethereum" | "sepolia")
    }
}

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
      "sepolia" => (
          Some(String::from("https://rpc.sepolia.org")),
          None
      ),
      _ => (None, None)
  }
}
