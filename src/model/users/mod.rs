//! Selfbot specific api parts
use std::ops::Deref;

use super::{
    id::{ChannelId, GuildId, MessageId, UserId},
    prelude::OnlineStatus,
    user::UserPublicFlags,
};
use crate::model::{deserialize_u16};

/// Summary of messages since last login.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ReadState {
    /// The channel's Id.
    pub id: ChannelId,
    /// The Id of the latest message sent to the channel.
    #[serde(default)]
    pub last_message_id: Option<MessageId>,
    /// The timestmap of the latest pinned message in the channel.
    #[serde(default)]
    pub last_pin_timestamp: Option<String>,
    /// The amount of times you've been mentioned in the channel.
    #[serde(default)]
    pub mention_count: u64,
}

/// The type of a relationship between two users.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(u8)]
pub enum RelationshipType {
    /// When a friend request was ignored.
    Ignored = 0,
    /// When users are friends.
    Friends = 1,
    /// When one user blocked the other one.
    Blocked = 2,
    /// When an incoming friend request was received.
    IncomingRequest = 3,
    /// When an outgoing friend request was sent.
    OutgoingRequest = 4,
    /// Unknown handling.
    Unknown = !0
}

enum_number!(RelationshipType {
    Ignored,
    Friends,
    Blocked,
    IncomingRequest,
    OutgoingRequest,

});

impl Default for RelationshipType {
    fn default() -> Self {
        Self::Ignored
    }
}



/// Information about a relationship that a user has with another user.
#[derive(Default, Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct Relationship {
    /// Id of the first relationship participant.
    // #[serde(deserialize_with = "deserialize_user_id")]
    pub id: UserId,
    /// Nickname of the user. Users only.
    pub nickname: Option<String>,
    /// Type of the relationship such as blocked, friends etc.
    #[serde(rename = "type")]
    pub kind: RelationshipType,
    /// User associated with the relationship.
    pub user: RequestUser,
}

#[derive(Default, Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct RequestUser {
    // #[serde(deserialize_with = "deserialize_user_id")]
    pub id: UserId,
    #[serde(rename = "username")]
    pub name: String,
    #[serde(deserialize_with = "deserialize_u16")]
    pub discriminator: u16,
    pub avatar: Option<String>,
    #[serde(rename = "public_flags")]
    pub flags: Option<UserPublicFlags>,
}

/// RequestUser implements a Deref to UserId so it gains the convenience methods
/// for converting it into a [`User`] instance.
impl Deref for RequestUser {
    type Target = UserId;

    fn deref(&self) -> &Self::Target {
        &self.id
    }
}
/// The current user's progress through the Discord tutorial.
#[derive(Default, Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct Tutorial {
    pub indicators_confirmed: Vec<String>,
    pub indicators_suppressed: bool,
}

/// Settings about a guild in regards to notification configuration
#[derive(Default, Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct UserGuildSettings {
    pub channel_overrides: Vec<ChannelOverride>,
    pub guild_id: Option<GuildId>,
    pub message_notifications: NotificationLevel,
    pub mobile_push: bool,
    pub muted: bool,
    pub suppress_everyone: bool,
}

/// An override for a [channel][`Channel`].
///
/// [`Channel`]: enum.Channel.html
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ChannelOverride {
    /// The channel this override is for.
    pub channel_id: ChannelId,
    /// The notification level to use for the channel.
    pub message_notifications: NotificationLevel,
    /// Whether or not the channel is muted; while this will not show a
    /// notification indicator for the channel, it will continue to show when the
    /// user is mentioned in it.
    pub muted: bool,
}

/// Identifier for the notification level of a channel.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum NotificationLevel {
    All = 0,
    Mentions = 1,
    Nothing = 2,
    Parent = 3,
}
impl Default for NotificationLevel {
    fn default() -> Self {
        Self::All
    }
}

/// User settings usually used to influence client behavior.
#[derive(Default, Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct UserSettings {
    pub convert_emoticons: bool,
    pub enable_tts_command: bool,
    pub friend_source_flags: FriendSourceFlags,
    pub inline_attachment_media: bool,
    pub inline_embed_media: bool,
    pub locale: String,
    pub message_display_compact: bool,
    pub render_embeds: bool,
    pub restricted_guilds: Vec<GuildId>,
    pub show_current_game: bool,
    pub status: OnlineStatus,
    pub theme: String,
}

/// Flags about who may or may not add the current user as a friend.
#[derive(Default, Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct FriendSourceFlags {
    pub all: bool,
    pub mutual_friends: bool,
    pub mutual_guilds: bool,
}

#[cfg(test)]
mod test {
    use serde_json::{to_string, to_value, from_str};

    use crate::{
        cache::{Cache, CacheUpdate, Settings},
        model::prelude::*,
    };

    #[tokio::test]
    // #[allow(clippy::unwrap_used)]
    pub async fn test() {
        let test_relationship = Relationship {
            id: UserId(1000000000000),
            nickname: None,
            kind: RelationshipType::Friends,
            user: RequestUser {
                id: UserId(999999999999),
                name: "friended_user".to_string(),
                discriminator: 9999,
                avatar: None,
                flags: Some(UserPublicFlags {
                    bits: 0,
                }),
            },
        };

        let test_str = r#"{"id":1000000000000,"nickname":null,"type":1,"user":{"id":999999999999,"username":"friended_user","discriminator":9999,"avatar":null,"public_flags":0}}"#;

        assert_eq!(test_str, to_string(&test_relationship).unwrap());

    }
}
