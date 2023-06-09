#![allow(clippy::missing_errors_doc)]
use core::fmt::Display;
use reqwest::{
    header::{HeaderMap, HeaderValue, InvalidHeaderValue},
    Certificate, Error as ReqwestError,
};
use serde::{Deserialize, Serialize};
use std::error::Error;

use crate::utils::port_and_auth;

#[derive(Debug)]
pub struct Api {
    reqwest_client: reqwest::Client,
    port: u16,
}

pub type ApiResult<T> = Result<T, ApiError>;

impl Api {
    pub fn new() -> ApiResult<Self> {
        let (port, auth) = port_and_auth()?;
        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            HeaderValue::from_str(format!("Basic {auth}").as_str())?,
        );

        let cert = Certificate::from_pem(include_bytes!("../../riotgames.pem"))?;
        let reqwest_client = reqwest::ClientBuilder::new()
            .add_root_certificate(cert)
            .default_headers(headers)
            .build()?;

        Ok(Api {
            reqwest_client,
            port,
        })
    }

    pub async fn request<T, U>(
        &self,
        method: reqwest::Method,
        endpoint: &str,
        body: T,
    ) -> ApiResult<U>
    where
        T: Serialize,
        U: for<'a> Deserialize<'a>,
    {
        Ok(self
            .reqwest_client
            .request(
                method,
                format!("https://127.0.0.1:{}{}", self.port, endpoint),
            )
            .json(&body)
            .send()
            .await?
            .json()
            .await?)
    }

    pub async fn request_no_response<T>(
        &self,
        method: reqwest::Method,
        endpoint: &str,
        body: T,
    ) -> ApiResult<()>
    where
        T: Serialize,
    {
        self.reqwest_client
            .request(
                method,
                format!("https://127.0.0.1:{}{}", self.port, endpoint),
            )
            .json(&body)
            .send()
            .await?;

        Ok(())
    }
}

#[derive(Debug)]
pub enum ApiError {
    ProcessNotFoundError,
    PortNotFoundError,
    TokenNotFoundError,
    ReqwestError(ReqwestError),
    InvalidHeaderValue(InvalidHeaderValue),
}

impl Error for ApiError {}

impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let error_message = match self {
            Self::ProcessNotFoundError => "Could not find a running LCU process".to_owned(),
            Self::PortNotFoundError => "Could not find port for LCU process".to_owned(),
            Self::TokenNotFoundError => "Could not find token for LCU process".to_owned(),
            Self::InvalidHeaderValue(err) => err.to_string(),
            Self::ReqwestError(err) => err.to_string(),
        };
        write!(f, "{error_message}")
    }
}

impl From<ReqwestError> for ApiError {
    fn from(value: ReqwestError) -> Self {
        Self::ReqwestError(value)
    }
}

impl From<InvalidHeaderValue> for ApiError {
    fn from(value: InvalidHeaderValue) -> Self {
        Self::InvalidHeaderValue(value)
    }
}
