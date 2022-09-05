use web3::Error;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref ADDR_RE: Regex = Regex::new(r"^(0x){1}(?i)[0-9a-f]{40}$").unwrap();
}

lazy_static! {
    static ref PKEY_RE: Regex = Regex::new(r"^(?i)[0-9a-f]{64}$").unwrap();
}

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

pub fn is_valid_address( address: String) -> bool {
    ADDR_RE.is_match(&address.to_string())
}

pub fn is_valid_secret_key(pkey_raw: String) -> bool {
    PKEY_RE.is_match(&pkey_raw.to_string())
}