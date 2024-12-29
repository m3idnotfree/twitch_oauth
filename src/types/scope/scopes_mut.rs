use std::collections::HashSet;

use super::Scope;

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

    /// chat:ediet, chat:read
    pub fn irc_scopes(&mut self) -> &mut Self {
        self.extend([Scope::ChatEdit, Scope::ChatRead]);
        self
    }

    /// get user: user:read:email
    /// update user: user:edit
    /// get user block list: user:read:blocked_users
    /// block, unblock user: user:manage:blocked_users
    /// get user extends: user:read:broadcast, user:edit:broadcast
    pub fn get_users(&mut self) -> &mut Self {
        self.scopes.insert(Scope::UserReadEmail);
        self
    }

    pub fn block(&mut self) -> &mut Self {
        self.scopes.insert(Scope::UserManageBlockedUsers);
        self
    }

    /// eventsub: channel:read:subscriptions
    pub fn read_eventsub(&mut self) -> &mut Self {
        self.scopes.insert(Scope::ChannelReadSubscriptions);
        self
    }

    /// modify channel info: channel:manage:broadcast
    /// get channel edit: channel:read:editors
    /// get followed channels: user:read:follows
    /// get channel followers: moderator:read:followers
    pub fn modify_channel_info(&mut self) -> &mut Self {
        self.scopes.insert(Scope::ChannelManageBroadcast);
        self
    }

    pub fn get_channel_editors(&mut self) -> &mut Self {
        self.scopes.insert(Scope::ChannelReadEditors);
        self
    }

    pub fn get_followed_channels(&mut self) -> &mut Self {
        self.scopes.insert(Scope::UserReadFollows);
        self
    }

    pub fn get_channel_followers(&mut self) -> &mut Self {
        self.scopes.insert(Scope::ModeratorReadFollowers);
        self
    }

    pub fn send_message(&mut self) -> &mut Self {
        self.scopes.insert(Scope::UserWriteChat);
        self
    }

    pub fn send_message_use_app_access_token(&mut self) -> &mut Self {
        self.scopes
            .extend([Scope::UserWriteChat, Scope::UserBot, Scope::ChannelBot]);
        self
    }

    /// moderator:read:chatters
    pub fn get_chatters(&mut self) -> &mut Self {
        self.scopes.insert(Scope::ModeratorReadChatters);
        self
    }

    // get chatters: moderator:read:chatters
    // get user emotes: user:read:emotes
    // update chat settings: moderator:manage:chat_settings
    // send chat announcement: moderator:manage:announcements
    // send shoutout: moderator:manage:shoutouts
    // send chat message: user:write:chat, user:bot, channel:bot
    // update user chat color: user:manage:chat_color
}

// #[cfg(test)]
// mod test {
//     use std::collections::HashSet;
//
//     #[test]
//     fn scopes_mut() {
//         let mut scopes = HashSet::new();
//
//         scopes::new(&mut scopes).irc_scopes();
//         assert_eq!(2, scopes.len());
//     }
// }
