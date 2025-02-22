use serde::{Deserialize, Serialize};

use crate::types::{InlineKeyboardMarkup, InputMessageContent, MimeWrapper, ParseMode};

/// Represents a link to a page containing an embedded video player or a video
/// file.
///
/// By default, this video file will be sent by the user with an optional
/// caption. Alternatively, you can use `input_messaage_content` to send a
/// message with the specified content instead of the video.
///
/// [The official docs](https://core.telegram.org/bots/api#inlinequeryresultvideo).
#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct InlineQueryResultVideo {
    /// Unique identifier for this result, 1-64 bytes.
    pub id: String,

    /// A valid URL for the embedded video player or video file.
    pub video_url: String,

    /// Mime type of the content of video url, `text/html` or `video/mp4`.
    pub mime_type: MimeWrapper,

    /// URL of the thumbnail (jpeg only) for the video.
    pub thumb_url: String,

    /// Title for the result.
    pub title: String,

    /// Caption of the video to be sent, 0-1024 characters.
    pub caption: Option<String>,

    /// Send [Markdown] or [HTML], if you want Telegram apps to show [bold,
    /// italic, fixed-width text or inline URLs] in the media caption.
    ///
    /// [Markdown]: https://core.telegram.org/bots/api#markdown-style
    /// [HTML]: https://core.telegram.org/bots/api#html-style
    /// [bold, italic, fixed-width text or inline URLs]: https://core.telegram.org/bots/api#formatting-options
    pub parse_mode: Option<ParseMode>,

    /// Video width.
    pub video_width: Option<i32>,

    /// Video height.
    pub video_height: Option<i32>,

    /// Video duration in seconds.
    pub video_duration: Option<i32>,

    /// Short description of the result.
    pub description: Option<String>,

    /// [Inline keyboard] attached to the message.
    ///
    /// [Inline keyboard]: https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating
    pub reply_markup: Option<InlineKeyboardMarkup>,

    /// Content of the message to be sent instead of the video. This field is
    /// **required** if [`InlineQueryResultVideo`] is used to send an HTML-page
    /// as a result (e.g., a YouTube video).
    ///
    /// [`InlineQueryResultVideo`]:
    /// crate::types::InlineQueryResultVideo
    pub input_message_content: Option<InputMessageContent>,
}

impl InlineQueryResultVideo {
    pub fn new<S1, S2, S3, S4>(
        id: S1,
        video_url: S2,
        mime_type: MimeWrapper,
        thumb_url: S3,
        title: S4,
    ) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
        S3: Into<String>,
        S4: Into<String>,
    {
        Self {
            id: id.into(),
            video_url: video_url.into(),
            mime_type,
            thumb_url: thumb_url.into(),
            title: title.into(),
            caption: None,
            parse_mode: None,
            video_width: None,
            video_height: None,
            video_duration: None,
            description: None,
            reply_markup: None,
            input_message_content: None,
        }
    }

    pub fn id<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.id = val.into();
        self
    }

    pub fn video_url<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.video_url = val.into();
        self
    }

    pub fn mime_type(mut self, val: MimeWrapper) -> Self {
        self.mime_type = val;
        self
    }

    pub fn thumb_url<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.thumb_url = val.into();
        self
    }

    pub fn title<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.title = val.into();
        self
    }

    pub fn caption<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.caption = Some(val.into());
        self
    }

    pub fn parse_mode(mut self, val: ParseMode) -> Self {
        self.parse_mode = Some(val);
        self
    }

    pub fn video_width(mut self, val: i32) -> Self {
        self.video_width = Some(val);
        self
    }

    pub fn video_height(mut self, val: i32) -> Self {
        self.video_height = Some(val);
        self
    }

    pub fn video_duration(mut self, val: i32) -> Self {
        self.video_duration = Some(val);
        self
    }

    pub fn description<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.description = Some(val.into());
        self
    }

    pub fn reply_markup(mut self, val: InlineKeyboardMarkup) -> Self {
        self.reply_markup = Some(val);
        self
    }

    pub fn input_message_content(mut self, val: InputMessageContent) -> Self {
        self.input_message_content = Some(val);
        self
    }
}
