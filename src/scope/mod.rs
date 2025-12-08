mod scopes_mut;

use std::{ops::Deref, str::FromStr};

use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};
use std::{collections::HashSet, fmt::Display};

pub use scopes_mut::{
    AdScopes, AnalyticScopes, BitScopes, CCLScopes, ChannelPointScopes, ChannelScopes,
    CharityScopes, ChatScopes, ChatbotScopes, ClipScopes, ConduitScopes, EntitlementScopes,
    EventSubScopes, ExtensionScopes, GameScopes, GoalScopes, GuestStarScopes, HypeTrainScopes,
    IRCScopes, ModerationScopes, PollScopes, PredictionScopes, RaidScopes, ScheduleScopes,
    ScopesMut, SearchScopes, StreamScopes, SubscriptionScopes, TagScopes, TeamScopes, UserScopes,
    VideoScopes, WhisperScopes,
};

pub(crate) fn scopes_mut(scopes: &mut HashSet<Scope>) -> ScopesMut<'_> {
    scopes_mut::new(scopes)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Scope {
    /// View analytics data for the Twitch Extensions owned by the authenticated account.
    /// API
    /// Get Extension Analytics
    /// <https://dev.twitch.tv/docs/api/reference/#get-extension-analytics>
    AnalyticsReadExtensions,
    /// View analytics data for the games owned by the authenticated account.
    /// API
    /// Get Game Analytics
    /// <https://dev.twitch.tv/docs/api/reference/#get-game-analytics>
    AnalyticsReadGames,
    /// View Bits information for a channel.
    ///
    /// API
    /// Get Bits Leaderboard
    /// <https://dev.twitch.tv/docs/api/reference/#get-bits-leaderboard>
    ///
    /// EventSub
    /// Channel Cheer
    /// <https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelcheer>
    BitsRead,
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
    ChannelBot,
    /// Manage ads schedule on a channel.
    ///
    /// API
    /// Snooze Next Ad
    /// <https://dev.twitch.tv/docs/api/reference/#snooze-next-ad>
    ChannelManageAds,
    /// Read the ads schedule and details on your channel.
    ///
    /// API
    /// Get Ad Schedule
    /// <https://dev.twitch.tv/docs/api/reference/#get-ad-schedule>
    ///
    /// EventSub
    /// Channel Ad Break Begin
    /// <https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelad_breakbegin>
    ChannelReadAds,
    /// Manage a channel’s broadcast configuration, including updating channel configuration and managing stream markers and stream tags.
    ///
    /// API
    /// Modify Channel Information
    /// Create Stream Marker
    /// Replace Stream Tags
    ChannelManageBroadcast,
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
    ChannelReadCharity,
    /// Run commercials on a channel.
    ///
    /// API
    /// Start Commercial
    ChannelEditCommercial,
    /// View a list of users with the editor role for a channel.
    ///
    /// API
    /// Get Channel Editors
    ChannelReadEditors,
    /// Manage a channel’s Extension configuration, including activating Extensions.
    ///
    /// API
    /// Get User Active Extensions
    /// Update User Extensions
    ChannelManageExtensions,
    /// View Creator Goals for a channel.
    ///
    /// API
    /// Get Creator Goals
    ///
    /// EventSub
    /// Goal Begin
    /// Goal Progress
    /// Goal End
    ChannelReadGoals,
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
    ChannelReadGuestStar,
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
    ChannelManageGuestStar,
    /// View Hype Train information for a channel.
    ///
    /// API
    /// Get Hype Train Events
    ///
    /// EventSub
    /// Hype Train Begin
    /// Hype Train Progress
    /// Hype Train End
    ChannelReadHypeTrain,
    /// Add or remove the moderator role from users in your channel.
    ///
    /// API
    /// Add Channel Moderator
    /// Remove Channel Moderator
    /// Get Moderators
    ChannelManageModerators,
    /// Perform moderation actions in a channel.
    ///
    /// EventSub
    /// Channel Moderate
    /// Channel Moderate v2
    ChannelModerate,
    /// View a channel’s polls.
    ///
    /// API
    /// Get Polls
    ///
    /// EventSub
    /// Channel Poll Begin
    /// Channel Poll Progress
    /// Channel Poll End
    ChannelReadPolls,
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
    ChannelManagePolls,
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
    ChannelReadPredictions,
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
    ChannelManagePredictions,
    /// Manage a channel raiding another channel.
    ///
    /// API
    /// Start a raid
    /// Cancel a raid
    ChannelManageRaids,
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
    ChannelReadRedemptions,
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
    ChannelManageRedemptions,
    /// Manage a channel’s stream schedule.
    ///
    /// API
    /// Update Channel Stream Schedule
    /// Create Channel Stream Schedule Segment
    /// Update Channel Stream Schedule Segment
    /// Delete Channel Stream Schedule Segment
    ChannelManageSchedule,
    /// View an authorized user’s stream key.
    ///
    /// API
    /// Get Stream Key
    ChannelReadStreamKey,
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
    ChannelReadSubscriptions,
    /// Manage a channel’s videos, including deleting videos.
    ///
    /// API
    /// Delete Videos
    ChannelManageVideos,
    /// Read the list of VIPs in your channel.
    ///
    /// API
    /// Get VIPs
    ///
    /// EventSub
    /// Channel VIP Add
    /// Channel VIP Remove
    ChannelReadVips,
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
    ChannelManageVips,
    /// Manage Clips for a channel.
    ///
    /// API
    /// Create Clip
    ClipsEdit,
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
    ModerationRead,
    /// Send announcements in channels where you have the moderator role.
    ///
    /// API
    /// Send Chat Announcement
    ModeratorManageAnnouncements,
    /// Manage messages held for review by AutoMod in channels where you are a moderator.
    ///
    /// API
    /// Manage Held AutoMod Messages
    ///
    /// EventSub
    /// AutoMod Message Hold
    /// AutoMod Message Update
    /// AutoMod Terms Update
    ModeratorManageAutomod,
    /// View a broadcaster’s AutoMod settings.
    ///
    /// API
    /// Get AutoMod Settings
    ///
    /// EventSub
    /// AutoMod Settings Update
    ModeratorReadAutomodSettings,
    /// Manage a broadcaster’s AutoMod settings.
    ///
    /// API
    /// Update AutoMod Settings
    ModeratorManageAutomodSettings,
    /// Read the list of bans or unbans in channels where you have the moderator role.
    ///
    /// EventSub
    /// Channel Moderate
    /// Channel Moderate v2
    ModeratorReadBannedUsers,
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
    ModeratorManageBannedUsers,
    /// View a broadcaster’s list of blocked terms.
    ///
    /// API
    /// Get Blocked Terms
    ///
    /// EventSub
    /// Channel Moderate
    ModeratorReadBlockedTerms,
    /// Read deleted chat messages in channels where you have the moderator role.
    ///
    /// EventSub
    /// Channel Moderate
    ModeratorReadChatMessages,
    /// Manage a broadcaster’s list of blocked terms.
    ///
    /// API
    /// Get Blocked Terms
    /// Add Blocked Term
    /// Remove Blocked Term
    ///
    /// EventSub
    /// Channel Moderate
    ModeratorManageBlockedTerms,
    /// Delete chat messages in channels where you have the moderator role
    ///
    /// API
    /// Delete Chat Messages
    ///
    /// EventSub
    /// Channel Moderate
    ModeratorManageChatMessages,
    /// View a broadcaster’s chat room settings.
    ///
    /// API
    /// Get Chat Settings
    ///
    /// EventSub
    /// Channel Moderate
    ModeratorReadChatSettings,
    /// Manage a broadcaster’s chat room settings.
    ///
    /// API
    /// Update Chat Settings
    ///
    /// EventSub
    /// Channel Moderate
    ModeratorManageChatSettings,
    /// View the chatters in a broadcaster’s chat room.
    ///
    /// API
    /// Get Chatters
    ModeratorReadChatters,
    /// Read the followers of a broadcaster.
    ///
    /// API
    /// Get Channel Followers
    ///
    /// EventSub
    /// Channel Follow
    ModeratorReadFollowers,
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
    ModeratorReadGuestStar,
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
    ModeratorManageGuestStar,
    /// Read the list of moderators in channels where you have the moderator role.
    ///
    /// EventSub
    /// Channel Moderate
    /// Channel Moderate v2
    ModeratorReadModerators,
    /// View a broadcaster’s Shield Mode status.
    ///
    /// API
    /// Get Shield Mode Status
    ///
    /// EventSub
    /// Shield Mode Begin
    /// Shield Mode End
    ModeratorReadShieldMode,
    /// Manage a broadcaster’s Shield Mode status.
    ///
    /// API
    /// Update Shield Mode Status
    ///
    /// EventSub
    /// Shield Mode Begin
    /// Shield Mode End
    ModeratorManageShieldMode,
    /// View a broadcaster’s shoutouts.
    ///
    /// EventSub
    /// Shoutout Create
    /// Shoutout Received
    ModeratorReadShoutouts,
    /// Manage a broadcaster’s shoutouts.
    ///
    /// API
    /// Send a Shoutout
    ///
    /// EventSub
    /// Shoutout Create
    /// Shoutout Received
    ModeratorManageShoutouts,
    /// Read chat messages from suspicious users and see users flagged as suspicious in channels where you have the moderator role.
    ///
    /// EventSub
    /// Channel Suspicious User Message
    /// Channel Suspicious User Update
    ModeratorReadSuspiciousUsers,
    /// View a broadcaster’s unban requests.
    ///
    /// API
    /// Get Unban Requests
    ///
    /// EventSub
    /// Channel Unban Request Create
    /// Channel Unban Request Resolve
    /// Channel Moderate
    ModeratorReadUnbanRequests,
    /// Manage a broadcaster’s unban requests.
    ///
    /// API
    /// Resolve Unban Requests
    ///
    /// EventSub
    /// Channel Unban Request Create
    /// Channel Unban Request Resolve
    /// Channel Moderate
    ModeratorManageUnbanRequests,
    /// Read the list of VIPs in channels where you have the moderator role.
    ///
    /// EventSub
    /// Channel Moderate
    /// Channel Moderate v2
    ModeratorReadVips,
    /// Read warnings in channels where you have the moderator role.
    ///
    /// EventSub
    /// Channel Moderate v2
    /// Channel Warning Acknowledge
    /// Channel Warning Send
    ModeratorReadWarnings,
    /// Warn users in channels where you have the moderator role.
    ///
    /// API
    /// Warn Chat User
    ///
    /// EventSub
    /// Channel Moderate v2
    /// Channel Warning Acknowledge
    /// Channel Warning Send
    ModeratorManageWarnings,
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
    UserBot,
    /// Manage a user object.
    ///
    /// API
    /// Update User
    UserEdit,
    /// View and edit a user’s broadcasting configuration, including Extension configurations.
    ///
    /// API
    /// Get User Extensions
    /// Get User Active Extensions
    /// Update User Extensions
    UserEditBroadcast,
    /// View the block list of a user.
    ///
    /// API
    /// Get User Block List
    /// <https://dev.twitch.tv/docs/api/reference/#get-user-block-list>
    UserReadBlockedUsers,
    /// Manage the block list of a user.
    ///
    /// API
    /// Block User
    /// <https://dev.twitch.tv/docs/api/reference/#block-user>
    /// Unblock User
    /// <https://dev.twitch.tv/docs/api/reference/#unblock-user>
    UserManageBlockedUsers,
    /// View a user’s broadcasting configuration, including Extension configurations.
    ///
    /// API
    /// Get Stream Markers
    /// Get User Extensions
    /// Get User Active Extensions
    UserReadBroadcast,
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
    UserReadChat,
    /// Update the color used for the user’s name in chat.
    ///
    /// API
    /// Update User Chat Color
    UserManageChatColor,
    /// View a user’s email address.
    ///
    /// API
    /// Get Users (optional)
    /// Update User (optional)
    ///
    /// EventSub
    /// User Update (optional)
    UserReadEmail,
    /// View emotes available to a user
    ///
    /// API
    /// Get User Emotes
    /// <https://dev.twitch.tv/docs/api/reference/#get-user-emotes>
    UserReadEmotes,
    /// View the list of channels a user follows.
    ///
    /// API
    /// Get Followed Channels
    /// Get Followed Streams
    UserReadFollows,
    /// Read the list of channels you have moderator privileges in.
    ///
    /// API
    /// Get Moderated Channels
    UserReadModeratedChannels,
    /// View if an authorized user is subscribed to specific channels.
    ///
    /// API
    /// Check User Subscription
    UserReadSubscriptions,
    /// Receive whispers sent to your user.
    ///
    /// EventSub
    /// Whisper Received
    UserReadWhispers,
    /// Receive whispers sent to your user, and send whispers on your user’s behalf.
    ///
    /// API
    /// Send Whisper
    ///
    /// EventSub
    /// Whisper Received
    UserManageWhispers,
    /// Send chat messages to a chatroom.
    ///
    /// API
    /// Send Chat Message
    /// <https://dev.twitch.tv/docs/api/reference/#send-chat-message>
    UserWriteChat,

    /// Send chat messages to a chatroom using an IRC connection.
    ///
    /// <https://dev.twitch.tv/docs/chat/irc>
    ChatEdit,
    /// View chat messages sent in a chatroom using an IRC connection.
    ///
    /// <https://dev.twitch.tv/docs/chat/irc>
    ChatRead,
    /// The following table lists the scopes used only by PubSub.
    /// There may be additional scopes needed for some PubSub topics, but those are not listed here.
    /// Receive whisper messages for your user using PubSub.
    ///
    /// <https://dev.twitch.tv/docs/pubsub>
    WhispersRead,
}

impl Scope {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::AnalyticsReadExtensions => "analytics:read:extensions",
            Self::AnalyticsReadGames => "analytics:read:games",
            Self::BitsRead => "bits:read",
            Self::ChannelBot => "channel:bot",
            Self::ChannelManageAds => "channel:manage:ads",
            Self::ChannelReadAds => "channel:read:ads",
            Self::ChannelManageBroadcast => "channel:manage:broadcast",
            Self::ChannelReadCharity => "channel:read:charity",
            Self::ChannelEditCommercial => "channel:edit:commercial",
            Self::ChannelReadEditors => "channel:read:editors",
            Self::ChannelManageExtensions => "channel:manage:extensions",
            Self::ChannelReadGoals => "channel:read:goals",
            Self::ChannelReadGuestStar => "channel:read:guest_star",
            Self::ChannelManageGuestStar => "channel:manage:guest_star",
            Self::ChannelReadHypeTrain => "channel:read:hype_train",
            Self::ChannelManageModerators => "channel:manage:moderators",
            Self::ChannelModerate => "channel:moderate",
            Self::ChannelReadPolls => "channel:read:polls",
            Self::ChannelManagePolls => "channel:manage:polls",
            Self::ChannelReadPredictions => "channel:read:predictions",
            Self::ChannelManagePredictions => "channel:manage:predictions",
            Self::ChannelManageRaids => "channel:manage:raids",
            Self::ChannelReadRedemptions => "channel:read:redemptions",
            Self::ChannelManageRedemptions => "channel:manage:redemptions",
            Self::ChannelManageSchedule => "channel:manage:schedule",
            Self::ChannelReadStreamKey => "channel:read:stream_key",
            Self::ChannelReadSubscriptions => "channel:read:subscriptions",
            Self::ChannelManageVideos => "channel:manage:videos",
            Self::ChannelReadVips => "channel:read:vips",
            Self::ChannelManageVips => "channel:manage:vips",
            Self::ClipsEdit => "clips:edit",
            Self::ModerationRead => "moderation:read",
            Self::ModeratorManageAnnouncements => "moderator:manage:announcements",
            Self::ModeratorManageAutomod => "moderator:manage:automod",
            Self::ModeratorReadAutomodSettings => "moderator:read:automod_settings",
            Self::ModeratorManageAutomodSettings => "moderator:manage:automod_settings",
            Self::ModeratorReadBannedUsers => "moderator:read:banned_users",
            Self::ModeratorManageBannedUsers => "moderator:manage:banned_users",
            Self::ModeratorReadBlockedTerms => "moderator:read:blocked_terms",
            Self::ModeratorReadChatMessages => "moderator:read:chat_messages",
            Self::ModeratorManageBlockedTerms => "moderator:manage:blocked_terms",
            Self::ModeratorManageChatMessages => "moderator:manage:chat_messages",
            Self::ModeratorReadChatSettings => "moderator:read:chat_settings",
            Self::ModeratorManageChatSettings => "moderator:manage:chat_settings",
            Self::ModeratorReadChatters => "moderator:read:chatters",
            Self::ModeratorReadFollowers => "moderator:read:followers",
            Self::ModeratorReadGuestStar => "moderator:read:guest_star",
            Self::ModeratorManageGuestStar => "moderator:manage:guest_star",
            Self::ModeratorReadModerators => "moderator:read:moderators",
            Self::ModeratorReadShieldMode => "moderator:read:shield_mode",
            Self::ModeratorManageShieldMode => "moderator:manage:shield_mode",
            Self::ModeratorReadShoutouts => "moderator:read:shoutouts",
            Self::ModeratorManageShoutouts => "moderator:manage:shoutouts",
            Self::ModeratorReadSuspiciousUsers => "moderator:read:suspicious_users",
            Self::ModeratorReadUnbanRequests => "moderator:read:unban_requests",
            Self::ModeratorManageUnbanRequests => "moderator:manage:unban_requests",
            Self::ModeratorReadVips => "moderator:read:vips",
            Self::ModeratorReadWarnings => "moderator:read:warnings",
            Self::ModeratorManageWarnings => "moderator:manage:warnings",
            Self::UserBot => "user:bot",
            Self::UserEdit => "user:edit",
            Self::UserEditBroadcast => "user:edit:broadcast",
            Self::UserReadBlockedUsers => "user:read:blocked_users",
            Self::UserManageBlockedUsers => "user:manage:blocked_users",
            Self::UserReadBroadcast => "user:read:broadcast",
            Self::UserReadChat => "user:read:chat",
            Self::UserManageChatColor => "user:manage:chat_color",
            Self::UserReadEmail => "user:read:email",
            Self::UserReadEmotes => "user:read:emotes",
            Self::UserReadFollows => "user:read:follows",
            Self::UserReadModeratedChannels => "user:read:moderated_channels",
            Self::UserReadSubscriptions => "user:read:subscriptions",
            Self::UserReadWhispers => "user:read:whispers",
            Self::UserManageWhispers => "user:manage:whispers",
            Self::UserWriteChat => "user:write:chat",

            Self::WhispersRead => "whispers:read",

            Self::ChatEdit => "chat:edit",
            Self::ChatRead => "chat:read",
        }
    }
}

impl Display for Scope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<Scope> for String {
    fn from(value: Scope) -> Self {
        value.as_str().to_string()
    }
}

#[derive(Debug)]
pub struct InvalidScopeError(String);

impl Display for InvalidScopeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid scope: '{}'", self.0)
    }
}

impl std::error::Error for InvalidScopeError {}

impl FromStr for Scope {
    type Err = InvalidScopeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "analytics:read:extensions" => Ok(Self::AnalyticsReadExtensions),
            "analytics:read:games" => Ok(Self::AnalyticsReadGames),
            "bits:read" => Ok(Self::BitsRead),
            "channel:bot" => Ok(Self::ChannelBot),
            "channel:manage:ads" => Ok(Self::ChannelManageAds),
            "channel:read:ads" => Ok(Self::ChannelReadAds),
            "channel:manage:broadcast" => Ok(Self::ChannelManageBroadcast),
            "channel:read:charity" => Ok(Self::ChannelReadCharity),
            "channel:edit:commercial" => Ok(Self::ChannelEditCommercial),
            "channel:read:editors" => Ok(Self::ChannelReadEditors),
            "channel:manage:extensions" => Ok(Self::ChannelManageExtensions),
            "channel:read:goals" => Ok(Self::ChannelReadGoals),
            "channel:read:guest_star" => Ok(Self::ChannelReadGuestStar),
            "channel:manage:guest_star" => Ok(Self::ChannelManageGuestStar),
            "channel:read:hype_train" => Ok(Self::ChannelReadHypeTrain),
            "channel:manage:moderators" => Ok(Self::ChannelManageModerators),
            "channel:moderate" => Ok(Self::ChannelModerate),
            "channel:read:polls" => Ok(Self::ChannelReadPolls),
            "channel:manage:polls" => Ok(Self::ChannelManagePolls),
            "channel:read:predictions" => Ok(Self::ChannelReadPredictions),
            "channel:manage:predictions" => Ok(Self::ChannelManagePredictions),
            "channel:manage:raids" => Ok(Self::ChannelManageRaids),
            "channel:read:redemptions" => Ok(Self::ChannelReadRedemptions),
            "channel:manage:redemptions" => Ok(Self::ChannelManageRedemptions),
            "channel:manage:schedule" => Ok(Self::ChannelManageSchedule),
            "channel:read:stream_key" => Ok(Self::ChannelReadStreamKey),
            "channel:read:subscriptions" => Ok(Self::ChannelReadSubscriptions),
            "channel:manage:videos" => Ok(Self::ChannelManageVideos),
            "channel:read:vips" => Ok(Self::ChannelReadVips),
            "channel:manage:vips" => Ok(Self::ChannelManageVips),
            "clips:edit" => Ok(Self::ClipsEdit),
            "moderation:read" => Ok(Self::ModerationRead),
            "moderator:manage:announcements" => Ok(Self::ModeratorManageAnnouncements),
            "moderator:manage:automod" => Ok(Self::ModeratorManageAutomod),
            "moderator:read:automod_settings" => Ok(Self::ModeratorReadAutomodSettings),
            "moderator:manage:automod_settings" => Ok(Self::ModeratorManageAutomodSettings),
            "moderator:read:banned_users" => Ok(Self::ModeratorReadBannedUsers),
            "moderator:manage:banned_users" => Ok(Self::ModeratorManageBannedUsers),
            "moderator:read:blocked_terms" => Ok(Self::ModeratorReadBlockedTerms),
            "moderator:read:chat_messages" => Ok(Self::ModeratorReadChatMessages),
            "moderator:manage:blocked_terms" => Ok(Self::ModeratorManageBlockedTerms),
            "moderator:manage:chat_messages" => Ok(Self::ModeratorManageChatMessages),
            "moderator:read:chat_settings" => Ok(Self::ModeratorReadChatSettings),
            "moderator:manage:chat_settings" => Ok(Self::ModeratorManageChatSettings),
            "moderator:read:chatters" => Ok(Self::ModeratorReadChatters),
            "moderator:read:followers" => Ok(Self::ModeratorReadFollowers),
            "moderator:read:guest_star" => Ok(Self::ModeratorReadGuestStar),
            "moderator:manage:guest_star" => Ok(Self::ModeratorManageGuestStar),
            "moderator:read:moderators" => Ok(Self::ModeratorReadModerators),
            "moderator:read:shield_mode" => Ok(Self::ModeratorReadShieldMode),
            "moderator:manage:shield_mode" => Ok(Self::ModeratorManageShieldMode),
            "moderator:read:shoutouts" => Ok(Self::ModeratorReadShoutouts),
            "moderator:manage:shoutouts" => Ok(Self::ModeratorManageShoutouts),
            "moderator:read:suspicious_users" => Ok(Self::ModeratorReadSuspiciousUsers),
            "moderator:read:unban_requests" => Ok(Self::ModeratorReadUnbanRequests),
            "moderator:manage:unban_requests" => Ok(Self::ModeratorManageUnbanRequests),
            "moderator:read:vips" => Ok(Self::ModeratorReadVips),
            "moderator:read:warnings" => Ok(Self::ModeratorReadWarnings),
            "moderator:manage:warnings" => Ok(Self::ModeratorManageWarnings),
            "user:bot" => Ok(Self::UserBot),
            "user:edit" => Ok(Self::UserEdit),
            "user:edit:broadcast" => Ok(Self::UserEditBroadcast),
            "user:read:blocked_users" => Ok(Self::UserReadBlockedUsers),
            "user:manage:blocked_users" => Ok(Self::UserManageBlockedUsers),
            "user:read:broadcast" => Ok(Self::UserReadBroadcast),
            "user:read:chat" => Ok(Self::UserReadChat),
            "user:manage:chat_color" => Ok(Self::UserManageChatColor),
            "user:read:email" => Ok(Self::UserReadEmail),
            "user:read:emotes" => Ok(Self::UserReadEmotes),
            "user:read:follows" => Ok(Self::UserReadFollows),
            "user:read:moderated_channels" => Ok(Self::UserReadModeratedChannels),
            "user:read:subscriptions" => Ok(Self::UserReadSubscriptions),
            "user:read:whispers" => Ok(Self::UserReadWhispers),
            "user:manage:whispers" => Ok(Self::UserManageWhispers),
            "user:write:chat" => Ok(Self::UserWriteChat),
            "whispers:read" => Ok(Self::WhispersRead),
            "chat:edit" => Ok(Self::ChatEdit),
            "chat:read" => Ok(Self::ChatRead),
            _ => Err(InvalidScopeError(s.to_string())),
        }
    }
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Deref for Scope {
    type Target = str;

    fn deref(&self) -> &str {
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
        let s = String::deserialize(deserializer)?;
        s.parse()
            .map_err(|e: InvalidScopeError| D::Error::custom(e))
    }
}
