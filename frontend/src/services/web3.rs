use web3::Error;

#[derive(Clone, Default)]
pub struct Web3Service {
    pub endpoint: String,
    pub chain_id: String,
    client: Option<web3::Web3<web3::transports::Http>>
}


impl Web3Service {
    pub async fn new(endpoint: String, chain_id: String) -> Result<Self, Error> {
        let transport = web3::transports::Http::new(&endpoint.to_string()).unwrap();
        let client = web3::Web3::new(transport);
        let x = client.eth().block_number().await;
        if let Err(e) = x {
            return Err(e)
        } else {
            Ok(Web3Service { 
                endpoint, 
                chain_id,
                client: Some(client),
            })
        }
        
    }
    
    pub async fn block_height(&self) -> Result<u64, Error> {
        if self.client.is_none() {
            return Err(Error::Unreachable)
        }
        let client = self.client.clone().unwrap();
        let res = client.eth().block_number().await;
        if let Ok(val) = res{
            return Ok(val.as_u64())
        } else {
            return Err(res.err().unwrap())
        }
    }

    pub async fn get_gas(&self) -> Result<String, Error> {
        if self.client.is_none() {
            return Err(Error::Unreachable)
        }
        let client = self.client.clone().unwrap();
        let res = client.eth().gas_price().await;
        if let Ok(val) = res{
            return Ok(val.to_string())
        } else {
            return Err(res.err().unwrap())
        }
    }
}