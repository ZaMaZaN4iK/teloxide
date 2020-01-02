use serde::Serialize;

use crate::{
    network,
    requests::{Request, ResponseResult},
    types::{InlineKeyboardMarkup, Message},
    Bot,
};

/// Use this method to stop updating a live location message before live_period
/// expires. On success, if the message was sent by the bot, the sent Message is
/// returned, otherwise True is returned.
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct StopMessageLiveLocationInline<'a> {
    #[serde(skip_serializing)]
    bot: &'a Bot,

    /// Identifier of the inline message
    inline_message_id: String,
    /// A JSON-serialized object for a new inline keyboard.
    reply_markup: Option<InlineKeyboardMarkup>,
}

#[async_trait::async_trait]
impl Request<Message> for StopMessageLiveLocationInline<'_> {
    async fn send(&self) -> ResponseResult<Message> {
        network::request_json(
            self.bot.client(),
            self.bot.token(),
            "stopMessageLiveLocation",
            &serde_json::to_string(self).unwrap(),
        )
        .await
    }
}

impl<'a> StopMessageLiveLocationInline<'a> {
    pub(crate) fn new<I>(bot: &'a Bot, inline_message_id: I) -> Self
    where
        I: Into<String>,
    {
        let inline_message_id = inline_message_id.into();
        Self {
            bot,
            inline_message_id,
            reply_markup: None,
        }
    }

    pub fn inline_message_id<T>(mut self, val: T) -> Self
    where
        T: Into<String>,
    {
        self.inline_message_id = val.into();
        self
    }

    pub fn reply_markup(mut self, val: InlineKeyboardMarkup) -> Self {
        self.reply_markup = Some(val);
        self
    }
}
