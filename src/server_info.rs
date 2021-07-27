use chrono::{Date, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use url::{ParseError, Url};

#[derive(Deserialize, Serialize)]
struct RawResponse {
    #[serde(rename = "Success")]
    success: bool,
    #[serde(rename = "Error", skip_serializing_if = "Option::is_none", default)]
    error: Option<String>,
    #[serde(rename = "Servers", skip_serializing_if = "Option::is_none", default)]
    servers: Option<Vec<RawServerInfo>>,
    #[serde(rename = "Success", skip_serializing_if = "Option::is_none", default)]
    cooldown: Option<u64>,
}

impl From<Response> for RawResponse {
    fn from(response: Response) -> Self {
        match response {
            Response::Success(success) => RawResponse {
                success: true,
                error: None,
                servers: Some(
                    success
                        .servers
                        .into_iter()
                        .map(RawServerInfo::from)
                        .collect(),
                ),
                cooldown: Some(success.cooldown),
            },
            Response::Error(error) => RawResponse {
                success: false,
                error: Some(error.error),
                servers: None,
                cooldown: None,
            },
        }
    }
}

#[derive(Deserialize, Serialize)]
struct RawServerInfo {
    #[serde(rename = "ID")]
    id: u64,
    #[serde(rename = "Port")]
    port: u16,
    #[serde(
        rename = "LastOnline",
        skip_serializing_if = "Option::is_none",
        default
    )]
    last_online: Option<String>,
    #[serde(rename = "Players", skip_serializing_if = "Option::is_none", default)]
    players_count: Option<String>,
    #[serde(
        rename = "PlayersList",
        skip_serializing_if = "Option::is_none",
        default
    )]
    players: Option<Vec<RawPlayer>>,
    #[serde(rename = "Info", skip_serializing_if = "Option::is_none", default)]
    info: Option<String>,
    #[serde(rename = "FF", skip_serializing_if = "Option::is_none", default)]
    friendly_fire: Option<bool>,
    #[serde(rename = "WL", skip_serializing_if = "Option::is_none", default)]
    whitelist: Option<bool>,
    #[serde(rename = "Modded", skip_serializing_if = "Option::is_none", default)]
    modded: Option<bool>,
    #[serde(rename = "Mods", skip_serializing_if = "Option::is_none", default)]
    mods: Option<u64>,
    #[serde(rename = "Suppress", skip_serializing_if = "Option::is_none", default)]
    suppress: Option<bool>,
    #[serde(
        rename = "AutoSuppress",
        skip_serializing_if = "Option::is_none",
        default
    )]
    auto_suppress: Option<bool>,
}

impl From<ServerInfo> for RawServerInfo {
    fn from(server_info: ServerInfo) -> Self {
        Self {
            id: server_info.id,
            port: server_info.port,
            last_online: server_info
                .last_online
                .map(|date| date.format("%Y-%m-%d").to_string()),
            players_count: server_info.players_count.map(|players_count| {
                format!(
                    "{}/{}",
                    players_count.current_players, players_count.max_players
                )
            }),
            players: server_info
                .players
                .map(|players| players.into_iter().map(RawPlayer::from).collect()),
            info: server_info.info.map(base64::encode),
            friendly_fire: server_info.friendly_fire,
            whitelist: server_info.whitelist,
            modded: server_info.modded,
            mods: server_info.mods,
            suppress: server_info.suppress,
            auto_suppress: server_info.auto_suppress,
        }
    }
}

#[derive(Deserialize, Serialize)]
#[serde(untagged)]
enum RawPlayer {
    UserId(String),
    UserIdWithNickname {
        #[serde(rename = "ID")]
        id: String,
        #[serde(rename = "Nickname", default)]
        nickname: Option<String>,
    },
}

impl From<Player> for RawPlayer {
    fn from(player: Player) -> Self {
        if let Some(nickname) = player.nickname {
            Self::UserIdWithNickname {
                id: player.id,
                nickname: Some(nickname),
            }
        } else {
            Self::UserId(player.id)
        }
    }
}

pub enum Response {
    Success(SuccessResponse),
    Error(ErrorResponse),
}

impl FromStr for Response {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match serde_json::from_str::<RawResponse>(s) {
            Ok(raw) => Ok(raw.into()),
            Err(error) => Err(error),
        }
    }
}

impl From<Response> for String {
    fn from(val: Response) -> Self {
        serde_json::to_string(&RawResponse::from(val)).unwrap()
    }
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

#[derive(Clone, Default)]
pub struct ErrorResponse {
    error: String,
}

impl ErrorResponse {
    pub fn new(error: String) -> Self {
        Self { error }
    }

    /// Get a reference to the error response's error.
    pub fn error(&self) -> &str {
        self.error.as_str()
    }

    /// Get a mutable reference to the error response's error.
    pub fn error_mut(&mut self) -> &mut String {
        &mut self.error
    }
}

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

pub struct RequestParameters<'a> {
    url: &'a str,
    id: Option<u64>,
    key: Option<&'a str>,
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

impl<'a> RequestParameters<'a> {
    pub fn builder() -> RequestParametersBuilder<'a> {
        RequestParametersBuilder::new()
    }
}

#[derive(Default)]
pub struct RequestParametersBuilder<'a> {
    url: Option<&'a str>,
    id: Option<u64>,
    key: Option<&'a str>,
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

impl<'a> RequestParametersBuilder<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn build(self) -> RequestParameters<'a> {
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

    pub fn url(mut self, value: &'a str) -> Self {
        self.url = Some(value);
        self
    }

    pub fn id(mut self, value: u64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn key(mut self, value: &'a str) -> Self {
        self.key = Some(value);
        self
    }

    pub fn last_online(mut self, value: bool) -> Self {
        self.last_online = value;
        self
    }

    pub fn players(mut self, value: bool) -> Self {
        self.players = value;
        self
    }

    pub fn list(mut self, value: bool) -> Self {
        self.list = value;
        self
    }

    pub fn info(mut self, value: bool) -> Self {
        self.info = value;
        self
    }

    pub fn pastebin(mut self, value: bool) -> Self {
        self.pastebin = value;
        self
    }

    pub fn version(mut self, value: bool) -> Self {
        self.version = value;
        self
    }

    pub fn flags(mut self, value: bool) -> Self {
        self.flags = value;
        self
    }

    pub fn nicknames(mut self, value: bool) -> Self {
        self.nicknames = value;
        self
    }

    pub fn online(mut self, value: bool) -> Self {
        self.online = value;
        self
    }
}

pub enum Error {
    BadRequest,
    Unauthorized,
    IpNotVerified,
    RateLimitExceeded,
    UrlParseError(ParseError),
    ReqwestError(reqwest::Error),
}

pub async fn get<'a>(parameters: &'a RequestParameters<'a>) -> Result<Response, Error> {
    let mut query_parameters = Vec::new();
    let id;

    if let Some(id_) = parameters.id {
        id = id_.to_string();
        query_parameters.push(("id", id.as_str()));
    }
    if let Some(key) = parameters.key {
        query_parameters.push(("key", key));
    }
    if parameters.last_online {
        query_parameters.push(("lo", "true"));
    }
    if parameters.players {
        query_parameters.push(("players", "true"));
    }
    if parameters.list {
        query_parameters.push(("list", "true"));
    }
    if parameters.info {
        query_parameters.push(("info", "true"));
    }
    if parameters.pastebin {
        query_parameters.push(("pastebin", "true"));
    }
    if parameters.version {
        query_parameters.push(("version", "true"));
    }
    if parameters.flags {
        query_parameters.push(("flags", "true"));
    }
    if parameters.nicknames {
        query_parameters.push(("nicknames", "true"));
    }
    if parameters.online {
        query_parameters.push(("online", "true"));
    }

    match Url::parse_with_params(parameters.url, query_parameters) {
        Ok(url) => match reqwest::get(url).await {
            Ok(response) => match response.json::<RawResponse>().await {
                Ok(raw_response) => Ok(raw_response.into()),
                Err(error) => Err(Error::ReqwestError(error)),
            },
            Err(error) => Err(Error::ReqwestError(error)),
        },
        Err(error) => Err(Error::UrlParseError(error)),
    }
}
