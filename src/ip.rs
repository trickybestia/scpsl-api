//! This module contains functionality that can be used for
//! working with the `ip` API request.

use std::{
    error::Error as StdError,
    fmt::{Display, Formatter},
    net::IpAddr,
    str::FromStr,
};
use url::Url;

/// An enum representing an error for the `ip` request.
#[derive(Debug)]
pub enum Error {
    /// An enum variant representing [`AddrParseError`].
    AddrParseError(),
    /// An enum variant representing [`reqwest::Error`].
    ReqwestError(reqwest::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::AddrParseError() => write!(f, "could not parse ip address from the response"),
            Error::ReqwestError(error) => write!(f, "reqwest error: `{}`", error),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::ReqwestError(error) => Some(error),
            _ => None,
        }
    }
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
                Err(_) => Err(Error::AddrParseError()),
            },
            Err(error) => Err(Error::ReqwestError(error)),
        },
        Err(error) => Err(Error::ReqwestError(error)),
    }
}
