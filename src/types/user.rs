use serde::{Deserialize, Serialize};

/// This object represents a Telegram user or bot.
///
/// [The official docs](https://core.telegram.org/bots/api#user).
#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct User {
    /// Unique identifier for this user or bot.
    pub id: i32,

    /// `true`, if this user is a bot.
    pub is_bot: bool,

    /// User‘s or bot’s first name.
    pub first_name: String,

    /// User‘s or bot’s last name.
    pub last_name: Option<String>,

    /// User‘s or bot’s username.
    pub username: Option<String>,

    /// [IETF language tag] of the user's language.
    ///
    /// [IETF language tag]: https://en.wikipedia.org/wiki/IETF_language_tag
    pub language_code: Option<String>,
}

impl User {
    pub fn new<S>(id: i32, is_bot: bool, first_name: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            id,
            is_bot,
            first_name: first_name.into(),
            last_name: None,
            username: None,
            language_code: None,
        }
    }

    pub fn id<S>(mut self, val: i32) -> Self {
        self.id = val;
        self
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn is_bot<S>(mut self, val: bool) -> Self {
        self.is_bot = val;
        self
    }

    pub fn first_name<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.first_name = val.into();
        self
    }

    pub fn last_name<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.last_name = Some(val.into());
        self
    }

    pub fn username<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.username = Some(val.into());
        self
    }

    pub fn language_code<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.language_code = Some(val.into());
        self
    }
}

impl User {
    pub fn full_name(&self) -> String {
        match &self.last_name {
            Some(last_name) => (format!("{0} {1}", self.first_name, last_name)),
            None => self.first_name.clone(),
        }
    }

    pub fn mention(&self) -> Option<String> {
        Some(format!("@{}", self.username.as_ref()?))
    }

    pub fn url(&self) -> reqwest::Url {
        reqwest::Url::parse(format!("tg://user/?id={}", self.id).as_str()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize() {
        let json = r#"{
            "id":12345,
            "is_bot":false,
            "first_name":"firstName",
            "last_name":"lastName",
            "username":"Username",
            "language_code":"ru"
        }"#;
        let expected = User {
            id: 12345,
            is_bot: false,
            first_name: "firstName".to_string(),
            last_name: Some("lastName".to_string()),
            username: Some("Username".to_string()),
            language_code: Some(String::from("ru")),
        };
        let actual = serde_json::from_str::<User>(&json).unwrap();
        assert_eq!(actual, expected)
    }
}
