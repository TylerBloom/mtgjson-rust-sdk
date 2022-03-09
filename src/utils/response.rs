use hyper::{body, Response};

use std::error::Error;

pub async fn response_into_string(res: Response<body::Body>) -> Result<String, Box<dyn Error>> {
    let bytes = body::to_bytes(res.into_body()).await?;
    Ok(std::str::from_utf8(&bytes)?.to_string())
}

