use dotenv_codegen::dotenv;
use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Debug;

use crate::error::Error;
use crate::types::ReqResponse;

const API_ENDPOINT: &str = dotenv!("BACKEND_URL");

pub async fn request<B, T>(method: reqwest::Method, function: String, body: B) -> Result<ReqResponse<T>, Error>
where
    B: Serialize + Debug,
    T: DeserializeOwned + 'static + Debug,  
{
    let allow_body = method == reqwest::Method::POST;
    let url = format!("{}/{}", API_ENDPOINT, function);
    let mut builder = reqwest::Client::new()
        .request(method, url)
        .header("Content-Type", "application/json");
    
    if allow_body {
        builder = builder.json(&body);
    }

    match builder.send().await {
        Ok(data) => {
            let data: Result<ReqResponse<T>, _> = data.json::<ReqResponse<T>>().await;
            if ! data.is_ok() {
                return Err(Error::DeserializeError)
            }
            let data: ReqResponse<T> = data.unwrap();
            println!("Resp: {:?}", data);
            Ok(data)
        },
        Err(e) => {
            println!("Error in request: {:?}", e);
            Err(Error::RequestError)
        }
    }
}

pub async fn request_get<T>(function: String) -> Result<ReqResponse<T>, Error>
where
    T: DeserializeOwned + 'static + Debug,
{
    request(reqwest::Method::GET, function, ()).await
}