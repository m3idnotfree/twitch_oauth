mod scopes_mut;

pub use scopes_mut::ScopesMut;

use std::{
    collections::HashSet,
    error::Error as StdError,
    fmt::{Display, Formatter, Result as FmtResult},
    str::FromStr,
};

use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub(crate) fn scopes_mut(scopes: &mut HashSet<Scope>) -> ScopesMut<'_> {
    scopes_mut::new(scopes)
}

macro_rules! scopes {
    ($($(#[$meta:meta])*
        $scope:ident => $name:literal,
    )*) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        #[non_exhaustive]
        pub enum Scope {
            $($(#[$meta])*
                $scope,)*
        }

        impl Scope {
            pub const ALL: &[Scope] = &[$(Self::$scope),*];

            pub const fn as_str(&self) -> &'static str {
                match self {
                    $(Self::$scope => $name,)*
                }
            }
        }

        impl FromStr for Scope {
            type Err = ParseScopeError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $($name => Ok(Self::$scope),)*
                    _ => Err(ParseScopeError(s.to_string())),
                }
            }
        }
    };
}

scopes!(
    /// View analytics data for the Twitch Extensions owned by the authenticated account.
    /// API
    /// Get Extension Analytics
    /// <https://dev.twitch.tv/docs/api/reference/#get-extension-analytics>
    AnalyticsReadExtensions => "analytics:read:extensions",
    /// View analytics data for the games owned by the authenticated account.
    /// API
    /// Get Game Analytics
    /// <https://dev.twitch.tv/docs/api/reference/#get-game-analytics>
    AnalyticsReadGames => "analytics:read:games",
    /// View Bits information for a channel.
    ///
    /// API
    /// Get Bits Leaderboard
    /// <https://dev.twitch.tv/docs/api/reference/#get-bits-leaderboard>
    ///
    /// EventSub
    /// Channel Cheer
    /// <https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelcheer>
    BitsRead => "bits:read",
    /// Joins your channel’s chatroom as a bot user, and perform chat-related actions as that user.
    ///
    /// API
    /// Send Chat Message
    /// <https://dev.twitch.tv/docs/api/reference/#send-chat-message>
    ///
    /// EventSub
    /// Channel Chat Clear
    /// <https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchatclear>
    /// Channel Chat Clear User Messages
    /// <https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchatclear_user_messages>
    /// Channel Chat Message
    /// <https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchatmessage>
    /// Channel Chat Message Delete
    /// <https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchatmessage_delete>
    /// Channel Chat Notification
    /// <https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchatnotification>
    /// Channel Chat Settings Update
    /// <https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchat_settingsupdate>
    ChannelBot => "channel:bot",
    /// Manage ads schedule on a channel.
    ///
    /// API
    /// Snooze Next Ad
    /// <https://dev.twitch.tv/docs/api/reference/#snooze-next-ad>
    ChannelManageAds => "channel:manage:ads",
    /// Read the ads schedule and details on your channel.
    ///
    /// API
    /// Get Ad Schedule
    /// <https://dev.twitch.tv/docs/api/reference/#get-ad-schedule>
    ///
    /// EventSub
    /// Channel Ad Break Begin
    /// <https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelad_breakbegin>
    ChannelReadAds => "channel:read:ads",
    /// Manage a channel’s broadcast configuration, including updating channel configuration and managing stream markers and stream tags.
    ///
    /// API
    /// Modify Channel Information
    /// Create Stream Marker
    /// Replace Stream Tags
    ChannelManageBroadcast => "channel:manage:broadcast",
    /// Read charity campaign details and user donations on your channel.
    ///
    /// API
    /// Get Charity Campaign
    /// Get Charity Campaign Donations
    ///
    /// EventSub
    /// Charity Donation
    /// Charity Campaign Start
    /// Charity Campaign Progress
    /// Charity Campaign Stop
    ChannelReadCharity => "channel:read:charity",
    /// Run commercials on a channel.
    ///
    /// API
    /// Start Commercial
    ChannelEditCommercial => "channel:edit:commercial",
    /// View a list of users with the editor role for a channel.
    ///
    /// API
    /// Get Channel Editors
    ChannelReadEditors => "channel:read:editors",
    /// Manage a channel’s Extension configuration, including activating Extensions.
    ///
    /// API
    /// Get User Active Extensions
    /// Update User Extensions
    ChannelManageExtensions => "channel:manage:extensions",
    /// View Creator Goals for a channel.
    ///
    /// API
    /// Get Creator Goals
    ///
    /// EventSub
    /// Goal Begin
    /// Goal Progress
    /// Goal End
    ChannelReadGoals => "channel:read:goals",
    /// Read Guest Star details for your channel.
    ///
    /// API
    /// Get Channel Guest Star Settings
    /// Get Guest Star Session
    /// Get Guest Star Invites
    ///
    /// EventSub
    /// Channel Guest Star Session Begin
    /// Channel Guest Star Session End
    /// Channel Guest Star Guest Update
    /// Channel Guest Star Settings Update
    ChannelReadGuestStar => "channel:read:guest_star",
    /// Manage Guest Star for your channel.
    ///
    /// API
    /// Update Channel Guest Star Settings
    /// Create Guest Star Session
    /// End Guest Star Session
    /// Send Guest Star Invite
    /// Delete Guest Star Invite
    /// Assign Guest Star Slot
    /// Update Guest Star Slot
    /// Delete Guest Star Slot
    /// Update Guest Star Slot Settings
    ///
    /// EventSub
    /// Channel Guest Star Session Begin
    /// Channel Guest Star Session End
    /// Channel Guest Star Guest Update
    /// Channel Guest Star Settings Update
    ChannelManageGuestStar => "channel:manage:guest_star",
    /// View Hype Train information for a channel.
    ///
    /// API
    /// Get Hype Train Events
    ///
    /// EventSub
    /// Hype Train Begin
    /// Hype Train Progress
    /// Hype Train End
    ChannelReadHypeTrain => "channel:read:hype_train",
    /// Add or remove the moderator role from users in your channel.
    ///
    /// API
    /// Add Channel Moderator
    /// Remove Channel Moderator
    /// Get Moderators
    ChannelManageModerators => "channel:manage:moderators",
    /// Perform moderation actions in a channel.
    ///
    /// EventSub
    /// Channel Moderate
    /// Channel Moderate v2
    ChannelModerate => "channel:moderate",
    /// View a channel’s polls.
    ///
    /// API
    /// Get Polls
    ///
    /// EventSub
    /// Channel Poll Begin
    /// Channel Poll Progress
    /// Channel Poll End
    ChannelReadPolls => "channel:read:polls",
    /// Manage a channel’s polls.
    ///
    /// API
    /// Get Polls
    /// Create Poll
    /// End Poll
    ///
    /// EventSub
    /// Channel Poll Begin
    /// Channel Poll Progress
    /// Channel Poll End
    ChannelManagePolls => "channel:manage:polls",
    /// View a channel’s Channel Points Predictions.
    ///
    /// API
    /// Get Channel Points Predictions
    ///
    /// EventSub
    /// Channel Prediction Begin
    /// Channel Prediction Progress
    /// Channel Prediction Lock
    /// Channel Prediction End
    ChannelReadPredictions => "channel:read:predictions",
    /// Manage of channel’s Channel Points Predictions
    ///
    /// API
    /// Get Channel Points Predictions
    /// Create Channel Points Prediction
    /// End Channel Points Prediction
    ///
    /// EventSub
    /// Channel Prediction Begin
    /// Channel Prediction Progress
    /// Channel Prediction Lock
    /// Channel Prediction End
    ChannelManagePredictions => "channel:manage:predictions",
    /// Manage a channel raiding another channel.
    ///
    /// API
    /// Start a raid
    /// Cancel a raid
    ChannelManageRaids => "channel:manage:raids",
    /// View Channel Points custom rewards and their redemptions on a channel.
    ///
    /// API
    /// Get Custom Reward
    /// Get Custom Reward Redemption
    ///
    /// EventSub
    /// Channel Points Automatic Reward Redemption
    /// Channel Points Custom Reward Add
    /// Channel Points Custom Reward Update
    /// Channel Points Custom Reward Remove
    /// Channel Points Custom Reward Redemption Add
    /// Channel Points Custom Reward Redemption Update
    ChannelReadRedemptions => "channel:read:redemptions",
    /// Manage Channel Points custom rewards and their redemptions on a channel.
    ///
    /// API
    /// Get Custom Reward
    /// Get Custom Reward Redemption
    /// Create Custom Rewards
    /// Delete Custom Reward
    /// Update Custom Reward
    /// Update Redemption Status
    ///
    /// EventSub
    /// Channel Points Automatic Reward Redemption
    /// Channel Points Custom Reward Add
    /// Channel Points Custom Reward Update
    /// Channel Points Custom Reward Remove
    /// Channel Points Custom Reward Redemption Add
    /// Channel Points Custom Reward Redemption Update
    ChannelManageRedemptions => "channel:manage:redemptions",
    /// Manage a channel’s stream schedule.
    ///
    /// API
    /// Update Channel Stream Schedule
    /// Create Channel Stream Schedule Segment
    /// Update Channel Stream Schedule Segment
    /// Delete Channel Stream Schedule Segment
    ChannelManageSchedule => "channel:manage:schedule",
    /// View an authorized user’s stream key.
    ///
    /// API
    /// Get Stream Key
    ChannelReadStreamKey => "channel:read:stream_key",
    /// View a list of all subscribers to a channel and check if a user is subscribed to a channel.
    ///
    /// API
    /// Get Broadcaster Subscriptions
    ///
    /// EventSub
    /// Channel Subscribe
    /// Channel Subscription End
    /// Channel Subscription Gift
    /// Channel Subscription Message
    ChannelReadSubscriptions => "channel:read:subscriptions",
    /// Manage a channel’s videos, including deleting videos.
    ///
    /// API
    /// Delete Videos
    ChannelManageVideos => "channel:manage:videos",
    /// Read the list of VIPs in your channel.
    ///
    /// API
    /// Get VIPs
    ///
    /// EventSub
    /// Channel VIP Add
    /// Channel VIP Remove
    ChannelReadVips => "channel:read:vips",
    /// Add or remove the VIP role from users in your channel.
    ///
    /// API
    /// Get VIPs
    /// Add Channel VIP
    /// Remove Channel VIP
    ///
    /// EventSub
    /// Channel VIP Add
    /// Channel VIP Remove
    ChannelManageVips => "channel:manage:vips",
    /// Manage Clips for a channel.
    ///
    /// API
    /// Create Clip
    ClipsEdit => "clips:edit",
    /// View a channel’s moderation data including Moderators, Bans, Timeouts, and Automod settings.
    ///
    /// API
    /// Check AutoMod Status
    /// Get Banned Users
    /// Get Moderators
    ///
    /// EventSub
    /// Channel Moderator Add
    /// Channel Moderator Remove
    ChannelManageClips => "channel:manage:clips",
    EditorManageClips => "editor:manage:clips",
    ModerationRead => "moderation:read",
    /// Send announcements in channels where you have the moderator role.
    ///
    /// API
    /// Send Chat Announcement
    ModeratorManageAnnouncements => "moderator:manage:announcements",
    /// Manage messages held for review by AutoMod in channels where you are a moderator.
    ///
    /// API
    /// Manage Held AutoMod Messages
    ///
    /// EventSub
    /// AutoMod Message Hold
    /// AutoMod Message Update
    /// AutoMod Terms Update
    ModeratorManageAutomod => "moderator:manage:automod",
    /// View a broadcaster’s AutoMod settings.
    ///
    /// API
    /// Get AutoMod Settings
    ///
    /// EventSub
    /// AutoMod Settings Update
    ModeratorReadAutomodSettings => "moderator:read:automod_settings",
    /// Manage a broadcaster’s AutoMod settings.
    ///
    /// API
    /// Update AutoMod Settings
    ModeratorManageAutomodSettings => "moderator:manage:automod_settings",
    /// Read the list of bans or unbans in channels where you have the moderator role.
    ///
    /// EventSub
    /// Channel Moderate
    /// Channel Moderate v2
    ModeratorReadBannedUsers => "moderator:read:banned_users",
    /// Ban and unban users.
    ///
    /// API
    /// Get Banned Users
    /// Ban User
    /// Unban User
    ///
    /// EventSub
    /// Channel Moderate
    /// Channel Moderate v2
    ModeratorManageBannedUsers => "moderator:manage:banned_users",
    /// View a broadcaster’s list of blocked terms.
    ///
    /// API
    /// Get Blocked Terms
    ///
    /// EventSub
    /// Channel Moderate
    ModeratorReadBlockedTerms => "moderator:read:blocked_terms",
    /// Read deleted chat messages in channels where you have the moderator role.
    ///
    /// EventSub
    /// Channel Moderate
    ModeratorReadChatMessages => "moderator:read:chat_messages",
    /// Manage a broadcaster’s list of blocked terms.
    ///
    /// API
    /// Get Blocked Terms
    /// Add Blocked Term
    /// Remove Blocked Term
    ///
    /// EventSub
    /// Channel Moderate
    ModeratorManageBlockedTerms => "moderator:manage:blocked_terms",
    /// Delete chat messages in channels where you have the moderator role
    ///
    /// API
    /// Delete Chat Messages
    ///
    /// EventSub
    /// Channel Moderate
    ModeratorManageChatMessages => "moderator:manage:chat_messages",
    /// View a broadcaster’s chat room settings.
    ///
    /// API
    /// Get Chat Settings
    ///
    /// EventSub
    /// Channel Moderate
    ModeratorReadChatSettings => "moderator:read:chat_settings",
    /// Manage a broadcaster’s chat room settings.
    ///
    /// API
    /// Update Chat Settings
    ///
    /// EventSub
    /// Channel Moderate
    ModeratorManageChatSettings => "moderator:manage:chat_settings",
    /// View the chatters in a broadcaster’s chat room.
    ///
    /// API
    /// Get Chatters
    ModeratorReadChatters => "moderator:read:chatters",
    /// Read the followers of a broadcaster.
    ///
    /// API
    /// Get Channel Followers
    ///
    /// EventSub
    /// Channel Follow
    ModeratorReadFollowers => "moderator:read:followers",
    /// Read Guest Star details for channels where you are a Guest Star moderator.
    ///
    /// API
    /// Get Channel Guest Star Settings
    /// Get Guest Star Session
    /// Get Guest Star Invites
    ///
    /// EventSub
    /// Channel Guest Star Session Begin
    /// Channel Guest Star Session End
    /// Channel Guest Star Guest Update
    /// Channel Guest Star Settings Update
    ModeratorReadGuestStar => "moderator:read:guest_star",
    /// Manage Guest Star for channels where you are a Guest Star moderator.
    ///
    /// API
    /// Send Guest Star Invite
    /// Delete Guest Star Invite
    /// Assign Guest Star Slot
    /// Update Guest Star Slot
    /// Delete Guest Star Slot
    /// Update Guest Star Slot Settings
    ///
    /// EventSub
    /// Channel Guest Star Session Begin
    /// Channel Guest Star Session End
    /// Channel Guest Star Guest Update
    /// Channel Guest Star Settings Update
    ModeratorManageGuestStar => "moderator:manage:guest_star",
    /// Read the list of moderators in channels where you have the moderator role.
    ///
    /// EventSub
    /// Channel Moderate
    /// Channel Moderate v2
    ModeratorReadModerators => "moderator:read:moderators",
    /// View a broadcaster’s Shield Mode status.
    ///
    /// API
    /// Get Shield Mode Status
    ///
    /// EventSub
    /// Shield Mode Begin
    /// Shield Mode End
    ModeratorReadShieldMode => "moderator:read:shield_mode",
    /// Manage a broadcaster’s Shield Mode status.
    ///
    /// API
    /// Update Shield Mode Status
    ///
    /// EventSub
    /// Shield Mode Begin
    /// Shield Mode End
    ModeratorManageShieldMode => "moderator:manage:shield_mode",
    /// View a broadcaster’s shoutouts.
    ///
    /// EventSub
    /// Shoutout Create
    /// Shoutout Received
    ModeratorReadShoutouts => "moderator:read:shoutouts",
    /// Manage a broadcaster’s shoutouts.
    ///
    /// API
    /// Send a Shoutout
    ///
    /// EventSub
    /// Shoutout Create
    /// Shoutout Received
    ModeratorManageShoutouts => "moderator:manage:shoutouts",
    /// Read chat messages from suspicious users and see users flagged as suspicious in channels where you have the moderator role.
    ///
    /// EventSub
    /// Channel Suspicious User Message
    /// Channel Suspicious User Update
    ModeratorReadSuspiciousUsers => "moderator:read:suspicious_users",
    /// View a broadcaster’s unban requests.
    ///
    /// API
    /// Get Unban Requests
    ///
    /// EventSub
    /// Channel Unban Request Create
    /// Channel Unban Request Resolve
    /// Channel Moderate
    ModeratorReadUnbanRequests => "moderator:read:unban_requests",
    /// Manage a broadcaster’s unban requests.
    ///
    /// API
    /// Resolve Unban Requests
    ///
    /// EventSub
    /// Channel Unban Request Create
    /// Channel Unban Request Resolve
    /// Channel Moderate
    ModeratorManageUnbanRequests => "moderator:manage:unban_requests",
    /// Read the list of VIPs in channels where you have the moderator role.
    ///
    /// EventSub
    /// Channel Moderate
    /// Channel Moderate v2
    ModeratorReadVips => "moderator:read:vips",
    /// Read warnings in channels where you have the moderator role.
    ///
    /// EventSub
    /// Channel Moderate v2
    /// Channel Warning Acknowledge
    /// Channel Warning Send
    ModeratorReadWarnings => "moderator:read:warnings",
    /// Warn users in channels where you have the moderator role.
    ///
    /// API
    /// Warn Chat User
    ///
    /// EventSub
    /// Channel Moderate v2
    /// Channel Warning Acknowledge
    /// Channel Warning Send
    ModeratorManageWarnings => "moderator:manage:warnings",
    ModeratorManageSuspiciousUsers => "moderator:manage:suspicious_users",
    /// Join a specified chat channel as your user and appear as a bot,
    /// and perform chat-related actions as your user.
    ///
    /// API
    /// Send Chat Message
    /// <https://dev.twitch.tv/docs/api/reference/#send-chat-message>
    ///
    /// EventSub
    /// Channel Chat Clear
    /// Channel Chat Clear User Messages
    /// Channel Chat Message
    /// Channel Chat Message Delete
    /// Channel Chat Notification
    /// Channel Chat Settings Update
    /// Channel Chat User Message Hold
    /// Channel Chat User Message Update
    UserBot => "user:bot",
    /// Manage a user object.
    ///
    /// API
    /// Update User
    UserEdit => "user:edit",
    /// View and edit a user’s broadcasting configuration, including Extension configurations.
    ///
    /// API
    /// Get User Extensions
    /// Get User Active Extensions
    /// Update User Extensions
    UserEditBroadcast => "user:edit:broadcast",
    /// View the block list of a user.
    ///
    /// API
    /// Get User Block List
    /// <https://dev.twitch.tv/docs/api/reference/#get-user-block-list>
    UserReadBlockedUsers => "user:read:blocked_users",
    /// Manage the block list of a user.
    ///
    /// API
    /// Block User
    /// <https://dev.twitch.tv/docs/api/reference/#block-user>
    /// Unblock User
    /// <https://dev.twitch.tv/docs/api/reference/#unblock-user>
    UserManageBlockedUsers => "user:manage:blocked_users",
    /// View a user’s broadcasting configuration, including Extension configurations.
    ///
    /// API
    /// Get Stream Markers
    /// Get User Extensions
    /// Get User Active Extensions
    UserReadBroadcast => "user:read:broadcast",
    /// Receive chatroom messages and informational notifications relating to a channel’s chatroom.
    ///
    /// EventSub
    /// Channel Chat Clear
    /// Channel Chat Clear User Messages
    /// Channel Chat Message
    /// Channel Chat Message Delete
    /// Channel Chat Notification
    /// Channel Chat Settings Update
    /// Channel Chat User Message Hold
    /// Channel Chat User Message Update
    UserReadChat => "user:read:chat",
    /// Update the color used for the user’s name in chat.
    ///
    /// API
    /// Update User Chat Color
    UserManageChatColor => "user:manage:chat_color",
    /// View a user’s email address.
    ///
    /// API
    /// Get Users (optional)
    /// Update User (optional)
    ///
    /// EventSub
    /// User Update (optional)
    UserReadEmail => "user:read:email",
    /// View emotes available to a user
    ///
    /// API
    /// Get User Emotes
    /// <https://dev.twitch.tv/docs/api/reference/#get-user-emotes>
    UserReadEmotes => "user:read:emotes",
    /// View the list of channels a user follows.
    ///
    /// API
    /// Get Followed Channels
    /// Get Followed Streams
    UserReadFollows => "user:read:follows",
    /// Read the list of channels you have moderator privileges in.
    ///
    /// API
    /// Get Moderated Channels
    UserReadModeratedChannels => "user:read:moderated_channels",
    /// View if an authorized user is subscribed to specific channels.
    ///
    /// API
    /// Check User Subscription
    UserReadSubscriptions => "user:read:subscriptions",
    /// Receive whispers sent to your user.
    ///
    /// EventSub
    /// Whisper Received
    UserReadWhispers => "user:read:whispers",
    /// Receive whispers sent to your user, and send whispers on your user’s behalf.
    ///
    /// API
    /// Send Whisper
    ///
    /// EventSub
    /// Whisper Received
    UserManageWhispers => "user:manage:whispers",
    /// Send chat messages to a chatroom.
    ///
    /// API
    /// Send Chat Message
    /// <https://dev.twitch.tv/docs/api/reference/#send-chat-message>
    UserWriteChat => "user:write:chat",

    /// Send chat messages to a chatroom using an IRC connection.
    ///
    /// <https://dev.twitch.tv/docs/chat/irc>
    ChatEdit => "chat:edit",
    /// View chat messages sent in a chatroom using an IRC connection.
    ///
    /// <https://dev.twitch.tv/docs/chat/irc>
    ChatRead => "chat:read",
    /// The following table lists the scopes used only by PubSub.
    /// There may be additional scopes needed for some PubSub topics, but those are not listed here.
    /// Receive whisper messages for your user using PubSub.
    ///
    /// <https://dev.twitch.tv/docs/pubsub>
    WhispersRead => "whispers:read",
);

impl Display for Scope {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(self.as_str())
    }
}

impl From<Scope> for String {
    fn from(value: Scope) -> Self {
        value.as_str().to_string()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseScopeError(String);

impl ParseScopeError {
    pub fn input(&self) -> &str {
        &self.0
    }
}

impl Display for ParseScopeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "unknown scope: {:?}", self.0)
    }
}

impl StdError for ParseScopeError {}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Serialize for Scope {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for Scope {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::Scope;

    #[test]
    fn round_trip() {
        for &scope in Scope::ALL {
            assert_eq!(scope.as_str().parse::<Scope>().unwrap(), scope);
        }
    }
}
