use std::fmt;
use std::str::FromStr;
use std::error::Error as StdError;

#[derive(Debug, Clone)]
pub enum NetworkKind {
  Stellar,
  Evm,
}

#[derive(Debug, Clone)]
pub struct Network {
  pub name: String,
  pub kind: NetworkKind,
  pub default_rpc: Option<String>,
  pub network_passphrase: Option<String>,
}

#[derive(Debug)]
pub struct NetworkError(String);

impl fmt::Display for NetworkError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl StdError for NetworkError {}

impl Network {
  // Predefined networks
  pub fn ethereum() -> Self {
    Self {
      name: "ethereum".to_string(),
      kind: NetworkKind::Evm,
      default_rpc: Some("https://eth.llamarpc.com".to_string()),
      network_passphrase: None,
    }
  }

  pub fn stellar() -> Self {
    Self {
      name: "stellar".to_string(),
      kind: NetworkKind::Stellar,
      default_rpc: Some("https://mainnet.sorobanrpc.com".to_string()),
      network_passphrase: Some("Public Global Stellar Network ; September 2015".to_string()),
    }
  }

  pub fn stellar_testnet() -> Self {
    Self {
      name: "stellar-testnet".to_string(),
      kind: NetworkKind::Stellar,
      default_rpc: Some("https://soroban-testnet.stellar.org".to_string()),
      network_passphrase: Some("Test SDF Network ; September 2015".to_string()),
    }
  }

  // Custom network constructor
  pub fn custom_evm(name: String, rpc_url: Option<String>) -> Self {
    Self {
      name,
      kind: NetworkKind::Evm,
      default_rpc: rpc_url,
      network_passphrase: None,
    }
  }
}

impl FromStr for Network {
  type Err = NetworkError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s.to_lowercase().as_str() {
      "ethereum" => Ok(Self::ethereum()),
      "stellar" => Ok(Self::stellar()),
      "stellar-testnet" => Ok(Self::stellar_testnet()),
      name => Ok(Self::custom_evm(name.to_string(), None)),
    }
  }
}

impl fmt::Display for Network {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.name)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_network_from_str() {
    assert!(matches!(Network::from_str("ethereum"), Ok(_)));
    assert!(matches!(Network::from_str("stellar"), Ok(_)));
    assert!(matches!(Network::from_str("stellar-testnet"), Ok(_)));
    assert!(matches!(Network::from_str("sepolia"), Ok(_)));
    assert!(matches!(Network::from_str("custom"), Ok(_)));
  }

  #[test]
  fn test_network_defaults() {
    let stellar = Network::stellar();
    assert!(stellar.network_passphrase.is_some());
    assert!(stellar.default_rpc.is_some());

    let stellar_testnet = Network::stellar_testnet();
    assert!(stellar_testnet.network_passphrase.is_some());
    assert!(stellar_testnet.default_rpc.is_some());

    let ethereum = Network::ethereum();
    assert!(ethereum.network_passphrase.is_none());
    assert!(ethereum.default_rpc.is_some());
  }

  #[test]
  fn test_custom_evm() {
    let custom = Network::custom_evm(
      "my-network".to_string(),
      Some("https://my-rpc.example.com".to_string()),
    );
    assert_eq!(custom.name, "my-network");
    assert_eq!(custom.default_rpc.unwrap(), "https://my-rpc.example.com");
  }
}