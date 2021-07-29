//! This module contains functionality that can be used for
//! working with the `ip` API request.

use std::{
    net::{AddrParseError, IpAddr},
    str::FromStr,
};
use url::Url;

/// An enum representing an error for the `ip` request.
pub enum Error {
    /// An enum variant representing [`AddrParseError`].
    AddrParseError(AddrParseError),
    /// An enum variant representing [`reqwest::Error`].
    ReqwestError(reqwest::Error),
}

/// Returns current ip.
/// # Errors
/// Returns [`Error::AddrParseError`] if there was a returned ip address parse error.
/// Returns [`Error::ReqwestError`] if there was a [`reqwest::Error`].
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
