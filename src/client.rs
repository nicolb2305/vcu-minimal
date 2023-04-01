use reqwest::{
    header::{HeaderMap, HeaderValue},
    Certificate, Error as ReqwestError,
};
use serde::{Deserialize, Serialize};
use std::{error::Error, fmt::Display};

use crate::utils::port_and_auth;

#[derive(Debug)]
pub struct ApiClient {
    reqwest_client: reqwest::Client,
    port: u16,
}

pub type ApiResult<T> = Result<T, ApiError>;

impl ApiClient {
    pub fn new() -> ApiResult<Self> {
        let (port, auth) = port_and_auth()?;
        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            HeaderValue::from_str(format!("Basic {}", auth).as_str()).unwrap(),
        );

        let cert = Certificate::from_pem(include_bytes!("../riotgames.pem"))?;
        let reqwest_client = reqwest::ClientBuilder::new()
            .add_root_certificate(cert)
            .default_headers(headers)
            .build()?;

        Ok(ApiClient {
            reqwest_client,
            port,
        })
    }

    pub async fn request_deserialize<T, U>(
        &self,
        method: reqwest::Method,
        endpoint: &str,
        body: Option<U>,
    ) -> ApiResult<T>
    where
        T: for<'a> Deserialize<'a>,
        U: Serialize,
    {
        let mut request = self.reqwest_client.request(
            method,
            format!("https://127.0.0.1:{}{}", self.port, endpoint),
        );

        if let Some(b) = body {
            request = request.json(&b);
        }

        Ok(request.send().await?.json().await?)
    }

    pub async fn request<U>(
        &self,
        method: reqwest::Method,
        endpoint: &str,
        body: Option<U>,
    ) -> ApiResult<()>
    where
        U: Serialize,
    {
        let mut request = self.reqwest_client.request(
            method,
            format!("https://127.0.0.1:{}{}", self.port, endpoint),
        );

        if let Some(b) = body {
            request = request.json(&b);
        }

        request.send().await?;
        Ok(())
    }
}

#[derive(Debug)]
pub enum ApiError {
    ProcessNotFoundError,
    PortNotFoundError,
    TokenNotFoundError,
    ReqwestError(ReqwestError),
}

impl Error for ApiError {}

impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let error_message = match self {
            Self::ProcessNotFoundError => "Could not find a running LCU process".to_owned(),
            Self::PortNotFoundError => "Could not find port for LCU process".to_owned(),
            Self::TokenNotFoundError => "Could not find token for LCU process".to_owned(),
            Self::ReqwestError(err) => err.to_string(),
        };
        write!(f, "{}", error_message)
    }
}

impl From<ReqwestError> for ApiError {
    fn from(value: ReqwestError) -> Self {
        Self::ReqwestError(value)
    }
}
