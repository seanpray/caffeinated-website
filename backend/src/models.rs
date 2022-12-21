use std::{
    collections::HashMap,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use blake3::hash;
use notion::models::{properties::PropertyValue, users::UserCommon};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub(crate) struct User {
    username: String,
    pass_hash: Vec<u8>,
    admin: bool,
}

#[derive(Deserialize)]
pub(crate) struct LoginData {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub(crate) struct TokenResponse {
    pub token: [u8; 32],
}

#[derive(Deserialize)]
pub(crate) struct AccessToken {
    token: String,
    granted: String,
    expiration: u64,
}

#[derive(Clone, Serialize)]
pub(crate) struct PostAuthor {
    name: String,
    pfp: String,
}

impl PostAuthor {
    pub fn from_user_common(user: &UserCommon) -> Self {
        // TODO replace avatar url with unwrap to placeholder
        Self {
            name: user.name.clone().unwrap_or_default(),
            pfp: user.avatar_url.clone().unwrap_or_default(),
        }
    }
}

#[derive(Clone, Serialize)]
pub(crate) struct BlogPost {
    created_date: Option<String>,
    last_update: Option<String>,
    title: String,
    content: String,
    author: Option<PostAuthor>,
}

impl BlogPost {
    pub fn from_property_map(map: &HashMap<String, PropertyValue>) -> Option<Self> {
        let created_date = match map.get("Created") {
            Some(PropertyValue::CreatedTime {
                created_time,
                id: _,
            }) => Some(created_time.timestamp().to_string()),
            _ => None,
        };
        let last_update = match map.get("Last Edited Time") {
            Some(PropertyValue::LastEditedTime {
                last_edited_time,
                id: _,
            }) => Some(last_edited_time.timestamp().to_string()),
            _ => None,
        };
        let title = match map.get("Name") {
            Some(PropertyValue::Title { id: _, title }) => title
                .iter()
                .map(|x| x.plain_text())
                .collect::<Vec<&str>>()
                .join(" "),
            _ => return None,
        };
        let content = match map.get("Text") {
            Some(PropertyValue::Text { rich_text, id: _ }) => rich_text
                .iter()
                .map(|x| x.plain_text())
                .collect::<Vec<&str>>()
                .join(" "),
            _ => return None,
        };
        let author = match map.get("Created By") {
            Some(PropertyValue::CreatedBy {
                created_by: notion::models::users::User::Person { common, person: _ },
                id: _,
            }) => Some(PostAuthor::from_user_common(common)),
            _ => None,
        };
        Some(Self {
            created_date,
            last_update,
            title,
            content,
            author,
        })
    }
    pub fn title(&self) -> &str {
        &self.title
    }
}

impl AccessToken {
    pub fn new(user: &str) -> Self {
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::from_secs(0))
            .as_secs();
        // 4 is random enough for this
        Self {
            token: hash(format!("{time}{}{}", &user, time % 4).as_bytes())
                .to_hex()
                .to_string(),
            granted: user.to_owned(),
            expiration: time + 86400,
        }
    }
    #[inline(always)]
    pub fn blank() -> Self {
        Self {
            token: String::new(),
            granted: String::new(),
            expiration: 0,
        }
    }
}
