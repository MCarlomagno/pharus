#[derive(Debug, Clone)]
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
      matches!(self, Network::Ethereum | Network::Sepolia)
  }

  pub fn get_default_rpc(&self) -> Option<&'static str> {
    match self {
      Network::Stellar => Some("https://mainnet.sorobanrpc.com"),
      Network::Ethereum => Some("https://eth.llamarpc.com"),
      Network::Sepolia => Some("https://rpc.sepolia.org"),
    }
  }

  pub fn get_network_passphrase(&self) -> Option<&'static str> {
    match self {
      Network::Stellar => Some("Public Global Stellar Network ; September 2015"),
      _ => None,
    }
  }
}