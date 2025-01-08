use async_trait::async_trait;
use std::error::Error;

#[async_trait]
pub trait ContractLoader {
  async fn load_local(&self, path: &str) -> Result<String, Box<dyn Error>>;
  async fn load_remote(&self, address: &str, rpc_url: &str) -> Result<String, Box<dyn Error>>;
}

pub struct ContractComparator {
  loader: Box<dyn ContractLoader>,
}

impl ContractComparator {
  pub fn new(loader: Box<dyn ContractLoader>) -> Self {
    Self { loader }
  }

  pub async fn compare(
    &self,
    local_path: &str,
    remote_address: &str,
    rpc_url: &str,
  ) -> Result<bool, Box<dyn Error>> {
    let local_hash = self.loader.load_local(local_path).await?;
    let remote_hash = self.loader.load_remote(remote_address, rpc_url).await?;
    
    Ok(local_hash == remote_hash)
  }
}