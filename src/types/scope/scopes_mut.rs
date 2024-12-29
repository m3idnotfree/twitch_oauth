use std::collections::HashSet;

use super::Scope;

/// Trait for API-specific scope combinations
pub trait APIScopes {
    // Ads
    // Analytics
    // Bits
    // Channels
    // Channel Points
    // Charity
    // chat
    /// moderator:read:chatters
    /// https://dev.twitch.tv/docs/api/reference/#get-chatters
    fn with_chatters(&mut self) -> &mut Self;
    /// user:read:emotes
    /// https://dev.twitch.tv/docs/api/reference/#get-user-emotes
    fn with_user_emotes_read(&mut self) -> &mut Self;
    /// user:write:chat
    /// https://dev.twitch.tv/docs/api/reference/#send-chat-message
    fn with_chat_write(&mut self) -> &mut Self;
    /// user:write:chat user:bot channel:bot
    fn with_chat_write_as_app(&mut self) -> &mut Self;
    // Clips
    // Conduits
    // CCLs
    // Entitlements
    // Extensions
    // EventSub
    // Games
    // Goals
    // Guest Star
    // Hype Train
    // Moderation
    // Polls
    /// channel:read:polls or channel:manage:polls
    /// https://dev.twitch.tv/docs/api/reference/#get-polls
    fn with_polls_read(&mut self) -> &mut Self;
    /// channel:manage:polls
    /// https://dev.twitch.tv/docs/api/reference/#create-poll
    /// https://dev.twitch.tv/docs/api/reference/#end-poll
    fn with_polls_manage(&mut self) -> &mut Self;
    // Predictions
    /// channel:read:predictions or channel:manage:predictions
    /// https://dev.twitch.tv/docs/api/reference/#get-predictions
    fn with_preictions_read(&mut self) -> &mut Self;
    /// channel:manage:predictions
    /// https://dev.twitch.tv/docs/api/reference/#create-prediction
    /// https://dev.twitch.tv/docs/api/reference/#end-prediction
    fn with_preictions_manage(&mut self) -> &mut Self;
    // Raid
    /// channel:manage:raids
    fn with_raid_manage(&mut self) -> &mut Self;
    // Schedule
    // Search
    // Streams
    // Subscriptions
    // Teams
    // Users
    /// user:read:email
    /// https://dev.twitch.tv/docs/api/reference/#get-users
    fn with_users_read(&mut self) -> &mut Self;
    /// user:read:blocked_users
    /// https://dev.twitch.tv/docs/api/reference/#get-user-block-list
    fn with_block_list_read(&mut self) -> &mut Self;
    /// user:manage:blocked_users
    /// https://dev.twitch.tv/docs/api/reference/#block-user
    /// https://dev.twitch.tv/docs/api/reference/#unblock-user
    fn with_block_list_manage(&mut self) -> &mut Self;
    // Videos
    // Whispers
}

// pub trait EventSubScopes {
//     fn chat_events(&mut self) -> &mut Self;
//     fn channel_events(&mut self) -> &mut Self;
//     fn moderation_events(&mut self) -> &mut Self;
// }

pub trait IRCScopes {
    /// Add all IRC scopes (chat:edit, chat:read)
    fn with_irc_all(&mut self) -> &mut Self;
    /// Add IRC chat edit scope
    fn with_irc_edit(&mut self) -> &mut Self;
    /// Add IRC chat read scope
    fn with_irc_read(&mut self) -> &mut Self;
}

impl APIScopes for ScopesMut<'_> {
    fn with_chatters(&mut self) -> &mut Self {
        self.push(Scope::ModeratorReadChatters);
        self
    }
    fn with_user_emotes_read(&mut self) -> &mut Self {
        self.push(Scope::UserReadEmotes);
        self
    }
    fn with_chat_write(&mut self) -> &mut Self {
        self.push(Scope::UserWriteChat);
        self
    }
    fn with_chat_write_as_app(&mut self) -> &mut Self {
        self.extend([Scope::UserWriteChat, Scope::UserBot, Scope::ChannelBot]);
        self
    }
    fn with_polls_read(&mut self) -> &mut Self {
        self.push(Scope::ChannelReadPolls);
        self
    }
    fn with_polls_manage(&mut self) -> &mut Self {
        self.push(Scope::ChannelManagePolls);
        self
    }
    fn with_preictions_read(&mut self) -> &mut Self {
        self.push(Scope::ChannelReadPredictions);
        self
    }
    fn with_preictions_manage(&mut self) -> &mut Self {
        self.push(Scope::ChannelManagePredictions);
        self
    }
    fn with_raid_manage(&mut self) -> &mut Self {
        self.push(Scope::ChannelManageRaids);
        self
    }
    fn with_users_read(&mut self) -> &mut Self {
        self.push(Scope::UserReadEmail);
        self
    }
    fn with_block_list_read(&mut self) -> &mut Self {
        self.push(Scope::UserReadBlockedUsers);
        self
    }
    fn with_block_list_manage(&mut self) -> &mut Self {
        self.push(Scope::UserManageBlockedUsers);
        self
    }
}

impl IRCScopes for ScopesMut<'_> {
    fn with_irc_all(&mut self) -> &mut Self {
        self.extend([Scope::ChatEdit, Scope::ChatRead]);
        self
    }
    fn with_irc_edit(&mut self) -> &mut Self {
        self.push(Scope::ChatEdit);
        self
    }
    fn with_irc_read(&mut self) -> &mut Self {
        self.push(Scope::ChatRead);
        self
    }
}

/// inspired PathSegmentsMut
/// https://docs.rs/url/latest/src/url/path_segments.rs.html#37-42
#[derive(Debug)]
pub struct ScopesMut<'a> {
    scopes: &'a mut HashSet<Scope>,
}

pub fn new(scopes: &mut HashSet<Scope>) -> ScopesMut<'_> {
    ScopesMut { scopes }
}

impl ScopesMut<'_> {
    pub fn push(&mut self, s: Scope) -> &mut Self {
        self.scopes.insert(s);
        self
    }

    pub fn extend<I>(&mut self, scopes: I) -> &mut Self
    where
        I: IntoIterator<Item = Scope>,
    {
        self.scopes.extend(scopes);
        self
    }
}
