//! This module contains structs and functions these can be used
//! for working with the `serverinfo` API request.

#[cfg(not(feature = "raw"))]
mod raw;
#[cfg(feature = "raw")]
pub mod raw;

use chrono::{Date, NaiveDate, Utc};
use raw::*;
use reqwest::Error;
use url::Url;

/// An enum representing a parsed API response for the `serverinfo` request.
pub enum Response {
    /// Successful response.
    Success(SuccessResponse),
    /// Unsuccessful response.
    Error(ErrorResponse),
}

impl From<RawResponse> for Response {
    fn from(raw: RawResponse) -> Self {
        if let Some(error) = raw.error {
            Self::Error(ErrorResponse { error })
        } else {
            Self::Success(SuccessResponse {
                cooldown: raw.cooldown.unwrap(),
                servers: raw
                    .servers
                    .unwrap()
                    .into_iter()
                    .map(ServerInfo::from)
                    .collect(),
            })
        }
    }
}

/// A struct representing a successful API response for the `serverinfo` request.
#[derive(Clone, Default)]
pub struct SuccessResponse {
    cooldown: u64,
    servers: Vec<ServerInfo>,
}

impl SuccessResponse {
    /// Get a reference to the success response's cooldown.
    pub fn cooldown(&self) -> u64 {
        self.cooldown
    }

    /// Get a reference to the success response's servers.
    pub fn servers(&self) -> &[ServerInfo] {
        self.servers.as_slice()
    }

    /// Get a mutable reference to the success response's cooldown.
    pub fn cooldown_mut(&mut self) -> &mut u64 {
        &mut self.cooldown
    }

    /// Get a mutable reference to the success response's servers.
    pub fn servers_mut(&mut self) -> &mut Vec<ServerInfo> {
        &mut self.servers
    }
}

/// A struct representing an unsuccessful API response for the `serverinfo` request.
#[derive(Clone, Default)]
pub struct ErrorResponse {
    error: String,
}

impl ErrorResponse {
    /// Get a reference to the error response's error.
    pub fn error(&self) -> &str {
        self.error.as_str()
    }

    /// Get a mutable reference to the error response's error.
    pub fn error_mut(&mut self) -> &mut String {
        &mut self.error
    }
}

/// A struct representing a server info for the `serverinfo` request.
#[derive(Clone, Default)]
pub struct ServerInfo {
    id: u64,
    port: u16,
    last_online: Option<Date<Utc>>,
    players_count: Option<PlayersCount>,
    players: Option<Vec<Player>>,
    info: Option<String>,
    friendly_fire: Option<bool>,
    whitelist: Option<bool>,
    modded: Option<bool>,
    mods: Option<u64>,
    suppress: Option<bool>,
    auto_suppress: Option<bool>,
}

impl ServerInfo {
    /// Get a reference to the server info's id.
    pub fn id(&self) -> u64 {
        self.id
    }

    /// Get a reference to the server info's port.
    pub fn port(&self) -> u16 {
        self.port
    }

    /// Get a reference to the server info's last online.
    pub fn last_online(&self) -> Option<Date<Utc>> {
        self.last_online
    }

    /// Get a reference to the server info's players count.
    pub fn players_count(&self) -> Option<&PlayersCount> {
        self.players_count.as_ref()
    }

    /// Get a reference to the server info's players.
    pub fn players(&self) -> Option<&Vec<Player>> {
        self.players.as_ref()
    }

    /// Get a reference to the server info's info.
    pub fn info(&self) -> Option<&String> {
        self.info.as_ref()
    }

    /// Get a reference to the server info's friendly fire.
    pub fn friendly_fire(&self) -> Option<bool> {
        self.friendly_fire
    }

    /// Get a reference to the server info's whitelist.
    pub fn whitelist(&self) -> Option<bool> {
        self.whitelist
    }

    /// Get a reference to the server info's modded.
    pub fn modded(&self) -> Option<bool> {
        self.modded
    }

    /// Get a reference to the server info's mods.
    pub fn mods(&self) -> Option<u64> {
        self.mods
    }

    /// Get a reference to the server info's suppress.
    pub fn suppress(&self) -> Option<bool> {
        self.suppress
    }

    /// Get a reference to the server info's auto suppress.
    pub fn auto_suppress(&self) -> Option<bool> {
        self.auto_suppress
    }

    /// Get a mutable reference to the server info's id.
    pub fn id_mut(&mut self) -> &mut u64 {
        &mut self.id
    }

    /// Get a mutable reference to the server info's port.
    pub fn port_mut(&mut self) -> &mut u16 {
        &mut self.port
    }

    /// Get a mutable reference to the server info's last online.
    pub fn last_online_mut(&mut self) -> &mut Option<Date<Utc>> {
        &mut self.last_online
    }

    /// Get a mutable reference to the server info's players count.
    pub fn players_count_mut(&mut self) -> &mut Option<PlayersCount> {
        &mut self.players_count
    }

    /// Get a mutable reference to the server info's players.
    pub fn players_mut(&mut self) -> &mut Option<Vec<Player>> {
        &mut self.players
    }

    /// Get a mutable reference to the server info's info.
    pub fn info_mut(&mut self) -> &mut Option<String> {
        &mut self.info
    }

    /// Get a mutable reference to the server info's friendly fire.
    pub fn friendly_fire_mut(&mut self) -> &mut Option<bool> {
        &mut self.friendly_fire
    }

    /// Get a mutable reference to the server info's whitelist.
    pub fn whitelist_mut(&mut self) -> &mut Option<bool> {
        &mut self.whitelist
    }

    /// Get a mutable reference to the server info's modded.
    pub fn modded_mut(&mut self) -> &mut Option<bool> {
        &mut self.modded
    }

    /// Get a mutable reference to the server info's mods.
    pub fn mods_mut(&mut self) -> &mut Option<u64> {
        &mut self.mods
    }

    /// Get a mutable reference to the server info's suppress.
    pub fn suppress_mut(&mut self) -> &mut Option<bool> {
        &mut self.suppress
    }

    /// Get a mutable reference to the server info's auto suppress.
    pub fn auto_suppress_mut(&mut self) -> &mut Option<bool> {
        &mut self.auto_suppress
    }
}

impl From<RawServerInfo> for ServerInfo {
    fn from(raw: RawServerInfo) -> Self {
        Self {
            id: raw.id,
            port: raw.port,
            last_online: raw.last_online.map(|last_online| {
                Date::from_utc(
                    NaiveDate::parse_from_str(last_online.as_str(), "%Y-%m-%d").unwrap(),
                    Utc,
                )
            }),
            players_count: raw.players_count.map(|players_count| {
                let mut splitted = players_count.split('/');
                PlayersCount {
                    current_players: splitted.next().unwrap().parse().unwrap(),
                    max_players: splitted.next().unwrap().parse().unwrap(),
                }
            }),
            players: raw
                .players
                .map(|players| players.into_iter().map(Player::from).collect()),
            info: raw.info.map(|info| {
                std::str::from_utf8(base64::decode(info).unwrap().as_slice())
                    .unwrap()
                    .to_string()
            }),
            friendly_fire: raw.friendly_fire,
            whitelist: raw.whitelist,
            modded: raw.modded,
            mods: raw.mods,
            suppress: raw.suppress,
            auto_suppress: raw.auto_suppress,
        }
    }
}

/// A struct representing the server's players count.
#[derive(Clone, Default)]
pub struct PlayersCount {
    max_players: u32,
    current_players: u32,
}

impl PlayersCount {
    /// Get a reference to the players count's max players.
    pub fn max_players(&self) -> u32 {
        self.max_players
    }

    /// Get a reference to the players count's current players.
    pub fn current_players(&self) -> u32 {
        self.current_players
    }

    /// Get a mutable reference to the players count's max players.
    pub fn max_players_mut(&mut self) -> &mut u32 {
        &mut self.max_players
    }

    /// Get a mutable reference to the players count's current players.
    pub fn current_players_mut(&mut self) -> &mut u32 {
        &mut self.current_players
    }
}

/// A struct representing a player on the server.
#[derive(Clone, Default)]
pub struct Player {
    id: String,
    nickname: Option<String>,
}

impl Player {
    /// Get a reference to the player's id.
    pub fn id(&self) -> &str {
        self.id.as_str()
    }

    /// Get a reference to the player's nickname.
    pub fn nickname(&self) -> Option<&String> {
        self.nickname.as_ref()
    }
}

impl From<RawPlayer> for Player {
    fn from(raw: RawPlayer) -> Self {
        match raw {
            RawPlayer::UserId(id) => Self { id, nickname: None },
            RawPlayer::UserIdWithNickname { id, nickname } => Self { id, nickname },
        }
    }
}

/// A struct representing a parameters for the `serverinfo` request.
pub struct RequestParameters {
    url: Url,
    id: Option<u64>,
    key: Option<String>,
    last_online: bool,
    players: bool,
    list: bool,
    info: bool,
    pastebin: bool,
    version: bool,
    flags: bool,
    nicknames: bool,
    online: bool,
}

impl RequestParameters {
    /// Returns a new instance of the [`RequestParametersBuilder`].
    pub fn builder() -> RequestParametersBuilder {
        RequestParametersBuilder::new()
    }
}

/// A struct representing a builder for the [`RequestParameters`].
#[derive(Default)]
pub struct RequestParametersBuilder {
    url: Option<Url>,
    id: Option<u64>,
    key: Option<String>,
    last_online: bool,
    players: bool,
    list: bool,
    info: bool,
    pastebin: bool,
    version: bool,
    flags: bool,
    nicknames: bool,
    online: bool,
}

impl RequestParametersBuilder {
    /// Returns a new instance of the [`RequestParametersBuilder`].
    pub fn new() -> Self {
        Default::default()
    }

    /// Consumes the [`RequestParametersBuilder`] instance and returns an instance of the [`RequestParameters`].
    /// # Panics
    /// Panics if `self.url` is [`None`].
    pub fn build(self) -> RequestParameters {
        RequestParameters {
            url: self.url.unwrap(),
            id: self.id,
            key: self.key,
            last_online: self.last_online,
            players: self.players,
            list: self.list,
            info: self.info,
            pastebin: self.pastebin,
            version: self.version,
            flags: self.flags,
            nicknames: self.nicknames,
            online: self.online,
        }
    }

    /// Sets the url to be used.
    pub fn url(mut self, value: Url) -> Self {
        self.url = Some(value);
        self
    }

    /// Sets the `id` query parameter to be used.
    pub fn id(mut self, value: u64) -> Self {
        self.id = Some(value);
        self
    }

    /// Sets the `key` query parameter to be used.
    pub fn key(mut self, value: String) -> Self {
        self.key = Some(value);
        self
    }

    /// Sets the `lo` query parameter to be used.
    pub fn last_online(mut self, value: bool) -> Self {
        self.last_online = value;
        self
    }

    /// Sets the `players` query parameter to be used.
    pub fn players(mut self, value: bool) -> Self {
        self.players = value;
        self
    }

    /// Sets the `list` query parameter to be used.
    pub fn list(mut self, value: bool) -> Self {
        self.list = value;
        self
    }

    /// Sets the `info` query parameter to be used.
    pub fn info(mut self, value: bool) -> Self {
        self.info = value;
        self
    }

    /// Sets the `pastebin` query parameter to be used.
    pub fn pastebin(mut self, value: bool) -> Self {
        self.pastebin = value;
        self
    }

    /// Sets the `version` query parameter to be used.
    pub fn version(mut self, value: bool) -> Self {
        self.version = value;
        self
    }

    /// Sets the `flags` query parameter to be used.
    pub fn flags(mut self, value: bool) -> Self {
        self.flags = value;
        self
    }

    /// Sets the `nicknames` query parameter to be used.
    pub fn nicknames(mut self, value: bool) -> Self {
        self.nicknames = value;
        self
    }

    /// Sets the `online` query parameter to be used.
    pub fn online(mut self, value: bool) -> Self {
        self.online = value;
        self
    }
}

/// Returns info about own servers. See [official API reference](https://api.scpslgame.com/#/default/Get%20Server%20Info).
/// # Errors
/// Returns [`Error`] if there was an error in the [`reqwest`] crate.  
pub async fn get<'a>(parameters: &RequestParameters) -> Result<Response, Error> {
    raw::get(parameters).await.map(|response| response.into())
}
