use super::{Error, RequestParameters};
use serde::Deserialize;
#[cfg(feature = "raw")]
use serde::Serialize;
use url::Url;

#[cfg_attr(feature = "raw", derive(Serialize, Clone))]
#[derive(Deserialize)]
pub struct RawResponse {
    #[serde(rename = "Success")]
    pub success: bool,
    #[serde(rename = "Error", skip_serializing_if = "Option::is_none", default)]
    pub error: Option<String>,
    #[serde(rename = "Servers", skip_serializing_if = "Option::is_none", default)]
    pub servers: Option<Vec<RawServerInfo>>,
    #[serde(rename = "Success", skip_serializing_if = "Option::is_none", default)]
    pub cooldown: Option<u64>,
}

#[cfg(feature = "raw")]
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

#[cfg_attr(feature = "raw", derive(Serialize, Clone))]
#[derive(Deserialize)]
pub struct RawServerInfo {
    #[serde(rename = "ID")]
    pub id: u64,
    #[serde(rename = "Port")]
    pub port: u16,
    #[serde(
        rename = "LastOnline",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub last_online: Option<String>,
    #[serde(rename = "Players", skip_serializing_if = "Option::is_none", default)]
    pub players_count: Option<String>,
    #[serde(
        rename = "PlayersList",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub players: Option<Vec<RawPlayer>>,
    #[serde(rename = "Info", skip_serializing_if = "Option::is_none", default)]
    pub info: Option<String>,
    #[serde(rename = "FF", skip_serializing_if = "Option::is_none", default)]
    pub friendly_fire: Option<bool>,
    #[serde(rename = "WL", skip_serializing_if = "Option::is_none", default)]
    pub whitelist: Option<bool>,
    #[serde(rename = "Modded", skip_serializing_if = "Option::is_none", default)]
    pub modded: Option<bool>,
    #[serde(rename = "Mods", skip_serializing_if = "Option::is_none", default)]
    pub mods: Option<u64>,
    #[serde(rename = "Suppress", skip_serializing_if = "Option::is_none", default)]
    pub suppress: Option<bool>,
    #[serde(
        rename = "AutoSuppress",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub auto_suppress: Option<bool>,
}

#[cfg(feature = "raw")]
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

#[cfg_attr(feature = "raw", derive(Serialize, Clone))]
#[derive(Deserialize)]
#[serde(untagged)]
pub enum RawPlayer {
    UserId(String),
    UserIdWithNickname {
        #[serde(rename = "ID")]
        id: String,
        #[serde(rename = "Nickname", default)]
        nickname: Option<String>,
    },
}

#[cfg(feature = "raw")]
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

/// Returns raw info about own servers. See [official API reference](https://api.scpslgame.com/#/default/Get%20Server%20Info).
/// # Errors
/// Returns [`Error::ReqwestError`] if there was other `reqwest` error.  
/// Returns [`Error::UrlParseError`] if `parameters.url` could not be parsed.  
pub async fn get<'a>(parameters: &'a RequestParameters<'a>) -> Result<RawResponse, Error> {
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
                Ok(raw_response) => Ok(raw_response),
                Err(error) => Err(Error::ReqwestError(error)),
            },

            Err(error) => Err(Error::ReqwestError(error)),
        },
        Err(error) => Err(Error::UrlParseError(error)),
    }
}
