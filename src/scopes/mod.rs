mod scopes_mut;
pub(crate) use scopes_mut::new;
pub use scopes_mut::ScopesMut;
use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};

use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Scopes {
    /// View analytics data for the Twitch Extensions owned by the authenticated account.
    /// API
    /// Get Extension Analytics
    /// https://dev.twitch.tv/docs/api/reference/#get-extension-analytics
    AnalyticsReadExtensions,
    /// View analytics data for the games owned by the authenticated account.
    /// API
    /// Get Game Analytics
    /// https://dev.twitch.tv/docs/api/reference/#get-game-analytics
    AnalyticsReadGames,
    /// View Bits information for a channel.
    ///
    /// API
    /// Get Bits Leaderboard
    /// https://dev.twitch.tv/docs/api/reference/#get-bits-leaderboard
    ///
    /// EventSub
    /// Channel Cheer
    /// https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelcheer
    BitsRead,
    /// Joins your channel’s chatroom as a bot user, and perform chat-related actions as that user.
    ///
    /// API
    /// Send Chat Message
    /// https://dev.twitch.tv/docs/api/reference/#send-chat-message
    ///
    /// EventSub
    /// Channel Chat Clear
    /// https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchatclear
    /// Channel Chat Clear User Messages
    /// https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchatclear_user_messages
    /// Channel Chat Message
    /// https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchatmessage
    /// Channel Chat Message Delete
    /// https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchatmessage_delete
    /// Channel Chat Notification
    /// https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchatnotification
    /// Channel Chat Settings Update
    /// https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchat_settingsupdate
    ChannelBot,
    /// Manage ads schedule on a channel.
    ///
    /// API
    /// Snooze Next Ad
    /// https://dev.twitch.tv/docs/api/reference/#snooze-next-ad
    ChannelManageAds,
    /// Read the ads schedule and details on your channel.
    ///
    /// API
    /// Get Ad Schedule
    /// https://dev.twitch.tv/docs/api/reference/#get-ad-schedule
    ///
    /// EventSub
    /// Channel Ad Break Begin
    /// https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelad_breakbegin
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
    ChannelEditCommercial,
    ChannelReadEditors,
    ChannelManageExtensions,
    ChannelReadGoals,
    ChannelReadGuestStar,
    ChannelManageGuestStar,
    ChannelReadHypeTrain,
    ChannelManageModerators,
    ChannelReadPolls,
    ChannelManagePolls,
    ChannelReadPredictions,
    ChannelManagePredictions,
    ChannelManageRaids,
    ChannelReadRedemptions,
    ChannelManageRedemptions,
    ChannelManageSchedule,
    ChannelReadStreamKey,
    ChannelReadSubscriptions,
    ChannelManageVideos,
    ChannelReadVips,
    ChannelManageVips,
    ClipsEdit,
    //
    ModerationRead,
    ModeratorManageAnnouncements,
    ModeratorManageAutomod,
    ModeratorReadAutomodSettings,
    ModeratorManageAutomodSettings,
    ModeratorReadBannedUsers,
    ModeratorManageBannedUsers,
    ModeratorReadBlockedTerms,
    ModeratorReadChatMessages,
    ModeratorManageBlockedTerms,
    ModeratorManageChatMessages,
    ModeratorReadChatSettings,
    ModeratorManageChatSettings,
    ModeratorReadChatters,
    ModeratorReadFollowers,
    ModeratorReadGuestStar,
    ModeratorManageGuestStar,
    ModeratorReadModerators,
    ModeratorReadShieldMode,
    ModeratorManageShieldMode,
    ModeratorReadShoutouts,
    ModeratorManageShoutouts,
    ModeratorReadSuspiciousUsers,
    ModeratorReadUnbanRequests,
    ModeratorManageUnbanRequests,
    ModeratorReadVips,
    ModeratorReadWarnings,
    ModeratorManageWarnings,
    //
    UserBot,
    UserEdit,
    UserEditBroadcast,
    UserReadBlockedUsers,
    UserManageBlockedUsers,
    UserReadBroadcast,
    UserReadChat,
    UserManageChatColor,
    UserReadEmail,
    UserReadEmotes,
    UserReadFollows,
    UserReadModeratedChannels,
    UserReadSubscriptions,
    UserReadWhispers,
    UserManageWhispers,
    UserWriteChat,

    /// The following table lists the scopes used only by PubSub.
    /// There may be additional scopes needed for some PubSub topics, but those are not listed here.
    /// Receive whisper messages for your user using PubSub.
    WhispersRead,
    /// Send chat messages to a chatroom using an IRC connection.
    ChatEdit,
    /// View chat messages sent in a chatroom using an IRC connection.
    ChatRead,
    EmptyString,
}

impl Scopes {
    pub fn as_str(&self) -> &'static str {
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
            Self::ModeratorManageGuestStar => "moderator:manage:guest_star	",
            Self::ModeratorReadModerators => "moderator:read:moderators	",
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
            Self::EmptyString => "",
        }
    }
}

impl Display for Scopes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AnalyticsReadExtensions => write!(f, "analytics:read:extensions"),
            Self::AnalyticsReadGames => write!(f, "analytics:read:games"),
            Self::BitsRead => write!(f, "bits:read"),
            Self::ChannelBot => write!(f, "channel:bot"),
            Self::ChannelManageAds => write!(f, "channel:manage:ads"),
            Self::ChannelReadAds => write!(f, "channel:read:ads"),
            Self::ChannelManageBroadcast => write!(f, "channel:manage:broadcast"),
            Self::ChannelReadCharity => write!(f, "channel:read:charity"),
            Self::ChannelEditCommercial => write!(f, "channel:edit:commercial"),
            Self::ChannelReadEditors => write!(f, "channel:read:editors"),
            Self::ChannelManageExtensions => write!(f, "channel:manage:extensions"),
            Self::ChannelReadGoals => write!(f, "channel:read:goals"),
            Self::ChannelReadGuestStar => write!(f, "channel:read:guest_star"),
            Self::ChannelManageGuestStar => write!(f, "channel:manage:guest_star"),
            Self::ChannelReadHypeTrain => write!(f, "channel:read:hype_train"),
            Self::ChannelManageModerators => write!(f, "channel:manage:moderators"),
            Self::ChannelReadPolls => write!(f, "channel:read:polls"),
            Self::ChannelManagePolls => write!(f, "channel:manage:polls"),
            Self::ChannelReadPredictions => write!(f, "channel:read:predictions"),
            Self::ChannelManagePredictions => write!(f, "channel:manage:predictions"),
            Self::ChannelManageRaids => write!(f, "channel:manage:raids"),
            Self::ChannelReadRedemptions => write!(f, "channel:read:redemptions"),
            Self::ChannelManageRedemptions => write!(f, "channel:manage:redemptions"),
            Self::ChannelManageSchedule => write!(f, "channel:manage:schedule"),
            Self::ChannelReadStreamKey => write!(f, "channel:read:stream_key"),
            Self::ChannelReadSubscriptions => write!(f, "channel:read:subscriptions"),
            Self::ChannelManageVideos => write!(f, "channel:manage:videos"),
            Self::ChannelReadVips => write!(f, "channel:read:vips"),
            Self::ChannelManageVips => write!(f, "channel:manage:vips"),
            Self::ClipsEdit => write!(f, "clips:edit"),
            Self::ModerationRead => write!(f, "moderation:read"),
            Self::ModeratorManageAnnouncements => write!(f, "moderator:manage:announcements"),
            Self::ModeratorManageAutomod => write!(f, "moderator:manage:automod"),
            Self::ModeratorReadAutomodSettings => write!(f, "moderator:read:automod_settings"),
            Self::ModeratorManageAutomodSettings => write!(f, "moderator:manage:automod_settings"),
            Self::ModeratorReadBannedUsers => write!(f, "moderator:read:banned_users"),
            Self::ModeratorManageBannedUsers => write!(f, "moderator:manage:banned_users"),
            Self::ModeratorReadBlockedTerms => write!(f, "moderator:read:blocked_terms"),
            Self::ModeratorReadChatMessages => write!(f, "moderator:read:chat_messages"),
            Self::ModeratorManageBlockedTerms => write!(f, "moderator:manage:blocked_terms"),
            Self::ModeratorManageChatMessages => write!(f, "moderator:manage:chat_messages"),
            Self::ModeratorReadChatSettings => write!(f, "moderator:read:chat_settings"),
            Self::ModeratorManageChatSettings => write!(f, "moderator:manage:chat_settings"),
            Self::ModeratorReadChatters => write!(f, "moderator:read:chatters"),
            Self::ModeratorReadFollowers => write!(f, "moderator:read:followers"),
            Self::ModeratorReadGuestStar => write!(f, "moderator:read:guest_star"),
            Self::ModeratorManageGuestStar => write!(f, "moderator:manage:guest_star"),
            Self::ModeratorReadModerators => write!(f, "moderator:read:moderators"),
            Self::ModeratorReadShieldMode => write!(f, "moderator:read:shield_mode"),
            Self::ModeratorManageShieldMode => write!(f, "moderator:manage:shield_mode"),
            Self::ModeratorReadShoutouts => write!(f, "moderator:read:shoutouts"),
            Self::ModeratorManageShoutouts => write!(f, "moderator:manage:shoutouts"),
            Self::ModeratorReadSuspiciousUsers => write!(f, "moderator:read:suspicious_users"),
            Self::ModeratorReadUnbanRequests => write!(f, "moderator:read:unban_requests"),
            Self::ModeratorManageUnbanRequests => write!(f, "moderator:manage:unban_requests"),
            Self::ModeratorReadVips => write!(f, "moderator:read:vips"),
            Self::ModeratorReadWarnings => write!(f, "moderator:read:warnings"),
            Self::ModeratorManageWarnings => write!(f, "moderator:manage:warnings"),
            Self::UserBot => write!(f, "user:bot"),
            Self::UserEdit => write!(f, "user:edit"),
            Self::UserEditBroadcast => write!(f, "user:edit:broadcast"),
            Self::UserReadBlockedUsers => write!(f, "user:read:blocked_users"),
            Self::UserManageBlockedUsers => write!(f, "user:manage:blocked_users"),
            Self::UserReadBroadcast => write!(f, "user:read:broadcast"),
            Self::UserReadChat => write!(f, "user:read:chat"),
            Self::UserManageChatColor => write!(f, "user:manage:chat_color"),
            Self::UserReadEmail => write!(f, "user:read:email"),
            Self::UserReadEmotes => write!(f, "user:read:emotes"),
            Self::UserReadFollows => write!(f, "user:read:follows"),
            Self::UserReadModeratedChannels => write!(f, "user:read:moderated_channels"),
            Self::UserReadSubscriptions => write!(f, "user:read:subscriptions"),
            Self::UserReadWhispers => write!(f, "user:read:whispers"),
            Self::UserManageWhispers => write!(f, "user:manage:whispers"),
            Self::UserWriteChat => write!(f, "user:write:chat"),

            Self::WhispersRead => write!(f, "whispers:read"),

            Self::ChatEdit => write!(f, "chat:edit"),
            Self::ChatRead => write!(f, "chat:read"),
            Self::EmptyString => write!(f, ""),
        }
    }
}

impl From<Scopes> for String {
    fn from(value: Scopes) -> Self {
        match value {
            Scopes::AnalyticsReadExtensions => "analytics:read:extensions".to_string(),
            Scopes::AnalyticsReadGames => "analytics:read:games".to_string(),
            Scopes::BitsRead => "bits:read".to_string(),
            Scopes::ChannelBot => "channel:bot".to_string(),
            Scopes::ChannelManageAds => "channel:manage:ads".to_string(),
            Scopes::ChannelReadAds => "channel:read:ads".to_string(),
            Scopes::ChannelManageBroadcast => "channel:manage:broadcast".to_string(),
            Scopes::ChannelReadCharity => "channel:read:charity".to_string(),
            Scopes::ChannelEditCommercial => "channel:edit:commercial".to_string(),
            Scopes::ChannelReadEditors => "channel:read:editors".to_string(),
            Scopes::ChannelManageExtensions => "channel:manage:extensions".to_string(),
            Scopes::ChannelReadGoals => "channel:read:goals".to_string(),
            Scopes::ChannelReadGuestStar => "channel:read:guest_star".to_string(),
            Scopes::ChannelManageGuestStar => "channel:manage:guest_star".to_string(),
            Scopes::ChannelReadHypeTrain => "channel:read:hype_train".to_string(),
            Scopes::ChannelManageModerators => "channel:manage:moderators".to_string(),
            Scopes::ChannelReadPolls => "channel:read:polls".to_string(),
            Scopes::ChannelManagePolls => "channel:manage:polls".to_string(),
            Scopes::ChannelReadPredictions => "channel:read:predictions".to_string(),
            Scopes::ChannelManagePredictions => "channel:manage:predictions".to_string(),
            Scopes::ChannelManageRaids => "channel:manage:raids".to_string(),
            Scopes::ChannelReadRedemptions => "channel:read:redemptions".to_string(),
            Scopes::ChannelManageRedemptions => "channel:manage:redemptions".to_string(),
            Scopes::ChannelManageSchedule => "channel:manage:schedule".to_string(),
            Scopes::ChannelReadStreamKey => "channel:read:stream_key".to_string(),
            Scopes::ChannelReadSubscriptions => "channel:read:subscriptions".to_string(),
            Scopes::ChannelManageVideos => "channel:manage:videos".to_string(),
            Scopes::ChannelReadVips => "channel:read:vips".to_string(),
            Scopes::ChannelManageVips => "channel:manage:vips".to_string(),
            Scopes::ClipsEdit => "clips:edit".to_string(),
            Scopes::ModerationRead => "moderation:read".to_string(),
            Scopes::ModeratorManageAnnouncements => "moderator:manage:announcements".to_string(),
            Scopes::ModeratorManageAutomod => "moderator:manage:automod".to_string(),
            Scopes::ModeratorReadAutomodSettings => "moderator:read:automod_settings".to_string(),
            Scopes::ModeratorManageAutomodSettings => {
                "moderator:manage:automod_settings".to_string()
            }
            Scopes::ModeratorReadBannedUsers => "moderator:read:banned_users".to_string(),
            Scopes::ModeratorManageBannedUsers => "moderator:manage:banned_users".to_string(),
            Scopes::ModeratorReadBlockedTerms => "moderator:read:blocked_terms".to_string(),
            Scopes::ModeratorReadChatMessages => "moderator:read:chat_messages".to_string(),
            Scopes::ModeratorManageBlockedTerms => "moderator:manage:blocked_terms".to_string(),
            Scopes::ModeratorManageChatMessages => "moderator:manage:chat_messages".to_string(),
            Scopes::ModeratorReadChatSettings => "moderator:read:chat_settings".to_string(),
            Scopes::ModeratorManageChatSettings => "moderator:manage:chat_settings".to_string(),
            Scopes::ModeratorReadChatters => "moderator:read:chatters".to_string(),
            Scopes::ModeratorReadFollowers => "moderator:read:followers".to_string(),
            Scopes::ModeratorReadGuestStar => "moderator:read:guest_star".to_string(),
            Scopes::ModeratorManageGuestStar => "moderator:manage:guest_star".to_string(),
            Scopes::ModeratorReadModerators => "moderator:read:moderators	".to_string(),
            Scopes::ModeratorReadShieldMode => "moderator:read:shield_mode".to_string(),
            Scopes::ModeratorManageShieldMode => "moderator:manage:shield_mode".to_string(),
            Scopes::ModeratorReadShoutouts => "moderator:read:shoutouts".to_string(),
            Scopes::ModeratorManageShoutouts => "moderator:manage:shoutouts".to_string(),
            Scopes::ModeratorReadSuspiciousUsers => "moderator:read:suspicious_users".to_string(),
            Scopes::ModeratorReadUnbanRequests => "moderator:read:unban_requests".to_string(),
            Scopes::ModeratorManageUnbanRequests => "moderator:manage:unban_requests".to_string(),
            Scopes::ModeratorReadVips => "moderator:read:vips".to_string(),
            Scopes::ModeratorReadWarnings => "moderator:read:warnings".to_string(),
            Scopes::ModeratorManageWarnings => "moderator:manage:warnings".to_string(),
            Scopes::UserBot => "user:bot".to_string(),
            Scopes::UserEdit => "user:edit".to_string(),
            Scopes::UserEditBroadcast => "user:edit:broadcast".to_string(),
            Scopes::UserReadBlockedUsers => "user:read:blocked_users".to_string(),
            Scopes::UserManageBlockedUsers => "user:manage:blocked_users".to_string(),
            Scopes::UserReadBroadcast => "user:read:broadcast".to_string(),
            Scopes::UserReadChat => "user:read:chat".to_string(),
            Scopes::UserManageChatColor => "user:manage:chat_color".to_string(),
            Scopes::UserReadEmail => "user:read:email".to_string(),
            Scopes::UserReadEmotes => "user:read:emotes".to_string(),
            Scopes::UserReadFollows => "user:read:follows".to_string(),
            Scopes::UserReadModeratedChannels => "user:read:moderated_channels".to_string(),
            Scopes::UserReadSubscriptions => "user:read:subscriptions".to_string(),
            Scopes::UserReadWhispers => "user:read:whispers".to_string(),
            Scopes::UserManageWhispers => "user:manage:whispers".to_string(),
            Scopes::UserWriteChat => "user:write:chat".to_string(),

            Scopes::WhispersRead => "whispers:read".to_string(),

            Scopes::ChatEdit => "chat:edit".to_string(),
            Scopes::ChatRead => "chat:read".to_string(),
            Scopes::EmptyString => "".to_string(),
        }
    }
}

// impl Into<String> for Scopes {
//     fn into(self) -> String {
//         match self {
//             Self::AnalyticsReadExtensions => "analytics:read:extensions".to_string(),
//             Self::AnalyticsReadGames => "analytics:read:games".to_string(),
//             Self::BitsRead => "bits:read".to_string(),
//             Self::ChannelBot => "channel:bot".to_string(),
//             Self::ChannelManageAds => "channel:manage:ads".to_string(),
//             Self::ChannelReadAds => "channel:read:ads".to_string(),
//             Self::ChannelManageBroadcast => "channel:manage:broadcast".to_string(),
//             Self::ChannelReadCharity => "channel:read:charity".to_string(),
//             Self::ChannelEditCommercial => "channel:edit:commercial".to_string(),
//             Self::ChannelReadEditors => "channel:read:editors".to_string(),
//             Self::ChannelManageExtensions => "channel:manage:extensions".to_string(),
//             Self::ChannelReadGoals => "channel:read:goals".to_string(),
//             Self::ChannelReadGuestStar => "channel:read:guest_star".to_string(),
//             Self::ChannelManageGuestStar => "channel:manage:guest_star".to_string(),
//             Self::ChannelReadHypeTrain => "channel:read:hype_train".to_string(),
//             Self::ChannelManageModerators => "channel:manage:moderators".to_string(),
//             Self::ChannelReadPolls => "channel:read:polls".to_string(),
//             Self::ChannelManagePolls => "channel:manage:polls".to_string(),
//             Self::ChannelReadPredictions => "channel:read:predictions".to_string(),
//             Self::ChannelManagePredictions => "channel:manage:predictions".to_string(),
//             Self::ChannelManageRaids => "channel:manage:raids".to_string(),
//             Self::ChannelReadRedemptions => "channel:read:redemptions".to_string(),
//             Self::ChannelManageRedemptions => "channel:manage:redemptions".to_string(),
//             Self::ChannelManageSchedule => "channel:manage:schedule".to_string(),
//             Self::ChannelReadStreamKey => "channel:read:stream_key".to_string(),
//             Self::ChannelReadSubscriptions => "channel:read:subscriptions".to_string(),
//             Self::ChannelManageVideos => "channel:manage:videos".to_string(),
//             Self::ChannelReadVips => "channel:read:vips".to_string(),
//             Self::ChannelManageVips => "channel:manage:vips".to_string(),
//             Self::ClipsEdit => "clips:edit".to_string(),
//             Self::ModerationRead => "moderation:read".to_string(),
//             Self::ModeratorManageAnnouncements => "moderator:manage:announcements".to_string(),
//             Self::ModeratorManageAutomod => "moderator:manage:automod".to_string(),
//             Self::ModeratorReadAutomodSettings => "moderator:read:automod_settings".to_string(),
//             Self::ModeratorManageAutomodSettings => "moderator:manage:automod_settings".to_string(),
//             Self::ModeratorReadBannedUsers => "moderator:read:banned_users".to_string(),
//             Self::ModeratorManageBannedUsers => "moderator:manage:banned_users".to_string(),
//             Self::ModeratorReadBlockedTerms => "moderator:read:blocked_terms".to_string(),
//             Self::ModeratorReadChatMessages => "moderator:read:chat_messages".to_string(),
//             Self::ModeratorManageBlockedTerms => "moderator:manage:blocked_terms".to_string(),
//             Self::ModeratorManageChatMessages => "moderator:manage:chat_messages".to_string(),
//             Self::ModeratorReadChatSettings => "moderator:read:chat_settings".to_string(),
//             Self::ModeratorManageChatSettings => "moderator:manage:chat_settings".to_string(),
//             Self::ModeratorReadChatters => "moderator:read:chatters".to_string(),
//             Self::ModeratorReadFollowers => "moderator:read:followers".to_string(),
//             Self::ModeratorReadGuestStar => "moderator:read:guest_star".to_string(),
//             Self::ModeratorManageGuestStar => "moderator:manage:guest_star".to_string(),
//             Self::ModeratorReadModerators => "moderator:read:moderators	".to_string(),
//             Self::ModeratorReadShieldMode => "moderator:read:shield_mode".to_string(),
//             Self::ModeratorManageShieldMode => "moderator:manage:shield_mode".to_string(),
//             Self::ModeratorReadShoutouts => "moderator:read:shoutouts".to_string(),
//             Self::ModeratorManageShoutouts => "moderator:manage:shoutouts".to_string(),
//             Self::ModeratorReadSuspiciousUsers => "moderator:read:suspicious_users".to_string(),
//             Self::ModeratorReadUnbanRequests => "moderator:read:unban_requests".to_string(),
//             Self::ModeratorManageUnbanRequests => "moderator:manage:unban_requests".to_string(),
//             Self::ModeratorReadVips => "moderator:read:vips".to_string(),
//             Self::ModeratorReadWarnings => "moderator:read:warnings".to_string(),
//             Self::ModeratorManageWarnings => "moderator:manage:warnings".to_string(),
//             Self::UserBot => "user:bot".to_string(),
//             Self::UserEdit => "user:edit".to_string(),
//             Self::UserEditBroadcast => "user:edit:broadcast".to_string(),
//             Self::UserReadBlockedUsers => "user:read:blocked_users".to_string(),
//             Self::UserManageBlockedUsers => "user:manage:blocked_users".to_string(),
//             Self::UserReadBroadcast => "user:read:broadcast".to_string(),
//             Self::UserReadChat => "user:read:chat".to_string(),
//             Self::UserManageChatColor => "user:manage:chat_color".to_string(),
//             Self::UserReadEmail => "user:read:email".to_string(),
//             Self::UserReadEmotes => "user:read:emotes".to_string(),
//             Self::UserReadFollows => "user:read:follows".to_string(),
//             Self::UserReadModeratedChannels => "user:read:moderated_channels".to_string(),
//             Self::UserReadSubscriptions => "user:read:subscriptions".to_string(),
//             Self::UserReadWhispers => "user:read:whispers".to_string(),
//             Self::UserManageWhispers => "user:manage:whispers".to_string(),
//             Self::UserWriteChat => "user:write:chat".to_string(),
//
//             Self::WhispersRead => "whispers:read".to_string(),
//
//             Self::ChatEdit => "chat:edit".to_string(),
//             Self::ChatRead => "chat:read".to_string(),
//         }
//     }
// }

impl Serialize for Scopes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for Scopes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
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
            "" => Ok(Self::EmptyString),
            _ => Err(D::Error::custom(format!("unknown scope: '{}'", s))),
        }
    }
}
