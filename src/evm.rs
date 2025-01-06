pub struct EthereumClient;

impl EthereumClient {
    pub fn new() -> Self {
        Self
    }

    pub fn load_local(&self, path: String) -> Result<String, Box<dyn std::error::Error>> {
        // Load bytecode from artifact JSON
        let content = std::fs::read_to_string(path)?;
        let artifact: serde_json::Value = serde_json::from_str(&content)?;
        let bytecode = artifact["bytecode"]["object"].as_str()
            .ok_or("Bytecode not found in artifact")?;
        Ok(bytecode.to_string())
    }

    pub async fn load_remote(&self, address: String, rpc_url: Option<String>) -> Result<String, Box<dyn std::error::Error>> {
      panic!("not implemented")
    }
}