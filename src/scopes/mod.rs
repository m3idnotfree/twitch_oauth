mod scope_builder;
pub use scope_builder::*;

use std::fmt::Display;

#[derive(Debug, Clone)]
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
        }
    }
}
