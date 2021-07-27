use std::{
    net::{AddrParseError, IpAddr},
    str::FromStr,
};
use url::Url;

pub enum Error {
    AddrParseError(AddrParseError),
    ReqwestError(reqwest::Error),
}

pub async fn get(url: Url) -> Result<IpAddr, Error> {
    match reqwest::get(url).await {
        Ok(response) => match response.text().await {
            Ok(text) => match IpAddr::from_str(text.as_str()) {
                Ok(ip) => Ok(ip),
                Err(error) => Err(Error::AddrParseError(error)),
            },
            Err(error) => Err(Error::ReqwestError(error)),
        },
        Err(error) => Err(Error::ReqwestError(error)),
    }
}
