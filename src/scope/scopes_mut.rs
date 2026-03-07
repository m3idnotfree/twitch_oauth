use std::collections::HashSet;

use super::Scope;

/// A builder for adding OAuth scopes, inspired by
/// [`PathSegmentsMut`](https://docs.rs/url/latest/src/url/path_segments.rs.html#37-42).
///
/// # Method Prefixes
///
/// - `on_*` - EventSub subscription helpers
#[derive(Debug)]
pub struct ScopesMut<'a> {
    scopes: &'a mut HashSet<Scope>,
}

pub fn new(scopes: &mut HashSet<Scope>) -> ScopesMut<'_> {
    ScopesMut { scopes }
}

impl ScopesMut<'_> {
    pub fn clear(&mut self) -> &mut Self {
        self.scopes.clear();
        self
    }

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

// Ads
impl ScopesMut<'_> {
    #[inline]
    pub fn ads_api(&mut self) -> &mut Self {
        self.start_commercial().get_ad_schedule().snooze_next_ad()
    }

    /// <https://dev.twitch.tv/docs/api/reference/#start-commercial>
    pub fn start_commercial(&mut self) -> &mut Self {
        self.push(Scope::ChannelEditCommercial);
        self
    }

    /// <https://dev.twitch.tv/docs/api/reference/#get-ad-schedule>
    pub fn get_ad_schedule(&mut self) -> &mut Self {
        self.push(Scope::ChannelReadAds);
        self
    }

    /// <https://dev.twitch.tv/docs/api/reference/#snooze-next-ad>
    pub fn snooze_next_ad(&mut self) -> &mut Self {
        self.push(Scope::ChannelManageAds);
        self
    }
}

// Analytics
impl ScopesMut<'_> {
    pub fn analytics_api(&mut self) -> &mut Self {
        self.get_extension_analytics().get_game_analytics()
    }

    /// <https://dev.twitch.tv/docs/api/reference/#get-extension-analytics>
    pub fn get_extension_analytics(&mut self) -> &mut Self {
        self.push(Scope::AnalyticsReadExtensions);
        self
    }

    /// <https://dev.twitch.tv/docs/api/reference/#get-game-analytics>
    pub fn get_game_analytics(&mut self) -> &mut Self {
        self.push(Scope::AnalyticsReadGames);
        self
    }
}

// Bits
impl ScopesMut<'_> {
    pub fn bits_api(&mut self) -> &mut Self {
        self.get_bits_leaderboard()
            .get_cheermotes()
            .get_extension_transactions()
    }

    /// <https://dev.twitch.tv/docs/api/reference/#get-bits-leaderboard>
    pub fn get_bits_leaderboard(&mut self) -> &mut Self {
        self.push(Scope::BitsRead);
        self
    }

    /// <https://dev.twitch.tv/docs/api/reference/#get-cheermotes>
    pub fn get_cheermotes(&mut self) -> &mut Self {
        self
    }

    /// <https://dev.twitch.tv/docs/api/reference/#get-extension-transactions>
    pub fn get_extension_transactions(&mut self) -> &mut Self {
        self
    }
}

// Channels
impl ScopesMut<'_> {
    pub fn channel_api(&mut self) -> &mut Self {
        self.get_channel_info()
            .modify_channel_info()
            .get_channel_editors()
            .get_followed_channels()
            .get_channel_followers()
    }

    /// <https://dev.twitch.tv/docs/api/reference/#get-channel-information>
    pub fn get_channel_info(&mut self) -> &mut Self {
        self
    }

    /// <https://dev.twitch.tv/docs/api/reference/#modify-channel-information>
    pub fn modify_channel_info(&mut self) -> &mut Self {
        self.push(Scope::ChannelManageBroadcast);
        self
    }

    /// <https://dev.twitch.tv/docs/api/reference/#get-channel-editors>
    pub fn get_channel_editors(&mut self) -> &mut Self {
        self.push(Scope::ChannelReadEditors);
        self
    }

    /// <https://dev.twitch.tv/docs/api/reference/#get-followed-channels>
    pub fn get_followed_channels(&mut self) -> &mut Self {
        self.push(Scope::UserReadFollows);
        self
    }

    /// <https://dev.twitch.tv/docs/api/reference/#get-channel-followers>
    pub fn get_channel_followers(&mut self) -> &mut Self {
        self.push(Scope::ModeratorReadFollowers);
        self
    }

    /// <https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelban>
    /// <https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelunban>
    pub fn channel_ban_unban(&mut self) -> &mut Self {
        self.push(Scope::ChannelModerate);
        self
    }
}

// Chanel Points
impl ScopesMut<'_> {
    pub fn channel_points_api(&mut self) -> &mut Self {
        self.create_custom_reward()
            .delete_custom_reward()
            .get_custom_reward()
            .get_custom_reward_redemption()
            .update_custom_reward()
            .update_redemption_status()
    }

    /// <https://dev.twitch.tv/docs/api/reference/#create-custom-rewards>
    pub fn create_custom_reward(&mut self) -> &mut Self {
        self.push(Scope::ChannelManageRedemptions);
        self
    }

    /// <https://dev.twitch.tv/docs/api/reference/#delete-custom-reward>
    pub fn delete_custom_reward(&mut self) -> &mut Self {
        self.push(Scope::ChannelManageRedemptions);
        self
    }

    /// Accepts `channel:read:redemptions` or `channel:manage:redemptions`.
    /// Uses `channel:read:redemptions`.
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#get-custom-reward>
    pub fn get_custom_reward(&mut self) -> &mut Self {
        self.push(Scope::ChannelReadRedemptions);
        self
    }

    /// Accepts `channel:read:redemptions` or `channel:manage:redemptions scope`.
    /// Uses `channel:read:redemptions`.
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#get-custom-reward-redemption>
    pub fn get_custom_reward_redemption(&mut self) -> &mut Self {
        self.push(Scope::ChannelReadRedemptions);
        self
    }

    /// <https://dev.twitch.tv/docs/api/reference/#update-custom-reward>
    pub fn update_custom_reward(&mut self) -> &mut Self {
        self.push(Scope::ChannelManageRedemptions);
        self
    }

    /// <https://dev.twitch.tv/docs/api/reference/#update-redemption-status>
    pub fn update_redemption_status(&mut self) -> &mut Self {
        self.push(Scope::ChannelManageRedemptions);
        self
    }
}

// Charity
impl ScopesMut<'_> {
    pub fn charity_api(&mut self) -> &mut Self {
        self.get_charity_campaign().get_charity_campaign_donations()
    }

    /// <https://dev.twitch.tv/docs/api/reference/#get-charity-campaign>
    pub fn get_charity_campaign(&mut self) -> &mut Self {
        self.push(Scope::ChannelReadCharity);
        self
    }

    /// <https://dev.twitch.tv/docs/api/reference/#get-charity-campaign-donations>
    pub fn get_charity_campaign_donations(&mut self) -> &mut Self {
        self.push(Scope::ChannelReadCharity);
        self
    }
}

// Chat
impl ScopesMut<'_> {
    pub fn chat_api(&mut self) -> &mut Self {
        self.get_chatters()
            .get_channel_emotes()
            .get_global_emotes()
            .get_channel_chat_badges()
            .get_global_chat_badges()
            .get_chat_settings()
            .get_shard_chat_session()
            .get_user_emotes()
            .update_chat_settings()
            .send_chat_announcement()
            .send_shoutout()
            .send_chat_message()
            .get_user_chat_color()
            .update_user_chat_color()
    }

    /// <https://dev.twitch.tv/docs/api/reference/#get-chatters>
    pub fn get_chatters(&mut self) -> &mut Self {
        self.push(Scope::ModeratorReadChatters);
        self
    }

    /// <https://dev.twitch.tv/docs/api/reference/#get-channel-emotes>
    pub fn get_channel_emotes(&mut self) -> &mut Self {
        self
    }

    /// <https://dev.twitch.tv/docs/api/reference/#get-global-emotes>
    pub fn get_global_emotes(&mut self) -> &mut Self {
        self
    }

    /// <https://dev.twitch.tv/docs/api/reference/#get-emote-sets>
    pub fn get_emote_sets(&mut self) -> &mut Self {
        self
    }

    /// <https://dev.twitch.tv/docs/api/reference/#get-channel-chat-badges>
    pub fn get_channel_chat_badges(&mut self) -> &mut Self {
        self
    }

    /// <https://dev.twitch.tv/docs/api/reference/#get-global-chat-badges>
    pub fn get_global_chat_badges(&mut self) -> &mut Self {
        self
    }

    /// <https://dev.twitch.tv/docs/api/reference/#get-chat-settings>
    pub fn get_chat_settings(&mut self) -> &mut Self {
        self
    }

    /// <https://dev.twitch.tv/docs/api/reference/#get-shared-chat-session>
    pub fn get_shard_chat_session(&mut self) -> &mut Self {
        self
    }

    /// <https://dev.twitch.tv/docs/api/reference/#get-user-emotes>
    pub fn get_user_emotes(&mut self) -> &mut Self {
        self.push(Scope::UserReadEmotes);
        self
    }

    /// <https://dev.twitch.tv/docs/api/reference/#update-chat-settings>
    pub fn update_chat_settings(&mut self) -> &mut Self {
        self.push(Scope::ModeratorManageChatSettings);
        self
    }

    /// <https://dev.twitch.tv/docs/api/reference/#send-chat-announcement>
    pub fn send_chat_announcement(&mut self) -> &mut Self {
        self.push(Scope::ModeratorManageAnnouncements);
        self
    }

    /// <https://dev.twitch.tv/docs/api/reference/#send-a-shoutout>
    pub fn send_shoutout(&mut self) -> &mut Self {
        self.push(Scope::ModeratorManageShoutouts);
        self
    }

    /// <https://dev.twitch.tv/docs/api/reference/#send-chat-message>
    pub fn send_chat_message(&mut self) -> &mut Self {
        self.push(Scope::UserWriteChat);
        self
    }

    /// <https://dev.twitch.tv/docs/api/reference/#get-user-chat-color>
    pub fn get_user_chat_color(&mut self) -> &mut Self {
        self
    }

    /// <https://dev.twitch.tv/docs/api/reference/#update-user-chat-color>
    pub fn update_user_chat_color(&mut self) -> &mut Self {
        self.push(Scope::UserManageChatColor);
        self
    }
}

// Clips
impl ScopesMut<'_> {
    pub fn clips_api(&mut self) -> &mut Self {
        self.create_clip().get_clip()
    }

    /// <https://dev.twitch.tv/docs/api/reference/#create-clip>
    pub fn create_clip(&mut self) -> &mut Self {
        self.push(Scope::ClipsEdit);
        self
    }

    /// Accepts `editor:manage:clips` or `channel:manage:clips`.
    /// Uses `editor:manage:clips`.
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#create-clip-from-vod>
    pub fn create_clip_from_vod(&mut self) -> &mut Self {
        self.push(Scope::EditorManageClips);
        self
    }

    /// <https://dev.twitch.tv/docs/api/reference/#get-clips>
    pub fn get_clip(&mut self) -> &mut Self {
        self
    }

    /// Accepts `editor:manage:clips` or `channel:manage:clips`.
    /// Uses `editor:manage:clips`.
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#get-clips-download>
    pub fn get_clips_download(&mut self) -> &mut Self {
        self.push(Scope::EditorManageClips);
        self
    }
}

// CCLs
impl ScopesMut<'_> {
    pub fn ccl_api(&mut self) -> &mut Self {
        self.get_content_classification_labels()
    }

    /// <https://dev.twitch.tv/docs/api/reference/#get-content-classification-labels>
    pub fn get_content_classification_labels(&mut self) -> &mut Self {
        self
    }
}

// Games
impl ScopesMut<'_> {
    pub fn games_api(&mut self) -> &mut Self {
        self.get_top_games().get_games()
    }

    /// <https://dev.twitch.tv/docs/api/reference/#get-top-games>
    pub fn get_top_games(&mut self) -> &mut Self {
        self
    }

    /// <https://dev.twitch.tv/docs/api/reference/#get-games>
    pub fn get_games(&mut self) -> &mut Self {
        self
    }
}

// Goals
impl ScopesMut<'_> {
    pub fn goals_api(&mut self) -> &mut Self {
        self.get_creator_goals()
    }

    /// <https://dev.twitch.tv/docs/api/reference/#get-creator-goals>
    pub fn get_creator_goals(&mut self) -> &mut Self {
        self.push(Scope::ChannelReadGoals);
        self
    }
}

// Guest Star
impl ScopesMut<'_> {
    pub fn guest_star_api(&mut self) -> &mut Self {
        self.get_channel_guest_star_setings()
            .update_channel_guest_star_setings()
            .get_guest_star_session()
            .create_guest_star_session()
            .end_guest_star_session()
            .get_guest_star_invites()
            .send_guest_star_invite()
            .delete_guest_star_invite()
            .assign_guest_star_slot()
            .update_guest_star_slot()
            .delete_guest_star_slot()
            .update_guest_star_slot_settings()
    }

    /// Accepts `channel:read:guest_star`, `channel:manage:guest_star`,
    /// `moderator:read:guest_star`, or `moderator:manage:guest_star`.
    /// Uses `channel:read:guest_star`.
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#get-channel-guest-star-settings>
    pub fn get_channel_guest_star_setings(&mut self) -> &mut Self {
        self.push(Scope::ChannelReadGuestStar);
        self
    }

    /// <https://dev.twitch.tv/docs/api/reference/#update-channel-guest-star-settings>
    pub fn update_channel_guest_star_setings(&mut self) -> &mut Self {
        self.push(Scope::ChannelManageGuestStar);
        self
    }

    /// Accepts `channel:read:guest_star`, `channel:manage:guest_star`,
    /// `moderator:read:guest_star`, or `moderator:manage:guest_star`.
    /// Uses `channel:read:guest_star`.
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#get-guest-star-session>
    pub fn get_guest_star_session(&mut self) -> &mut Self {
        self.push(Scope::ChannelReadGuestStar);
        self
    }

    /// <https://dev.twitch.tv/docs/api/reference/#create-guest-star-session>
    pub fn create_guest_star_session(&mut self) -> &mut Self {
        self.push(Scope::ChannelManageGuestStar);
        self
    }

    /// <https://dev.twitch.tv/docs/api/reference/#end-guest-star-session>
    pub fn end_guest_star_session(&mut self) -> &mut Self {
        self.push(Scope::ChannelManageGuestStar);
        self
    }

    /// Accepts `channel:read:guest_star`, `channel:manage:guest_star`,
    /// `moderator:read:guest_star`, or `moderator:manage:guest_star`.
    /// Uses `channel:read:guest_star`.
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#get-guest-star-invites>
    pub fn get_guest_star_invites(&mut self) -> &mut Self {
        self.push(Scope::ChannelReadGuestStar);
        self
    }

    /// Accepts `channel:manage:guest_star` or `moderator:manage:guest_star`.
    /// Uses `channel:manage:guest_star`.
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#send-guest-star-invite>
    pub fn send_guest_star_invite(&mut self) -> &mut Self {
        self.push(Scope::ChannelManageGuestStar);
        self
    }

    /// Accepts `channel:manage:guest_star` or `moderator:manage:guest_star`.
    /// Uses `channel:manage:guest_star`.
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#delete-guest-star-invite>
    pub fn delete_guest_star_invite(&mut self) -> &mut Self {
        self.push(Scope::ChannelManageGuestStar);
        self
    }

    /// Accepts `channel:manage:guest_star` or `moderator:manage:guest_star`.
    /// Uses `channel:manage:guest_star`.
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#assign-guest-star-slot>
    pub fn assign_guest_star_slot(&mut self) -> &mut Self {
        self.push(Scope::ChannelManageGuestStar);
        self
    }

    /// Accepts `channel:manage:guest_star` or `moderator:manage:guest_star`.
    /// Uses `channel:manage:guest_star`.
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#update-guest-star-slot>
    pub fn update_guest_star_slot(&mut self) -> &mut Self {
        self.push(Scope::ChannelManageGuestStar);
        self
    }

    /// Accepts `channel:manage:guest_star` or `moderator:manage:guest_star`.
    /// Uses `channel:manage:guest_star`.
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#delete-guest-star-slot>
    pub fn delete_guest_star_slot(&mut self) -> &mut Self {
        self.push(Scope::ChannelManageGuestStar);
        self
    }

    /// Accepts `channel:manage:guest_star` or `moderator:manage:guest_star`.
    /// Uses `channel:manage:guest_star`.
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#update-guest-star-slot-settings>
    pub fn update_guest_star_slot_settings(&mut self) -> &mut Self {
        self.push(Scope::ChannelManageGuestStar);
        self
    }
}

// Hype train
impl ScopesMut<'_> {
    pub fn hype_train_api(&mut self) -> &mut Self {
        self.get_hype_train_status()
    }

    /// <https://dev.twitch.tv/docs/api/reference/#get-hype-train-status>
    pub fn get_hype_train_status(&mut self) -> &mut Self {
        self.push(Scope::ChannelReadHypeTrain);
        self
    }
}

// Moderation
impl ScopesMut<'_> {
    pub fn moderation_api(&mut self) -> &mut Self {
        self.check_automod_status()
            .manage_held_automod_messages()
            .get_automod_settings()
            .update_automod_settings()
            .get_banned_users()
            .ban_user()
            .unban_user()
            .get_unban_requests()
            .resolve_unban_requests()
            .get_blocked_terms()
            .add_blocked_term()
            .remove_blocked_term()
            .delete_chat_messages()
            .get_moderated_channels()
            .get_moderators()
            .add_channel_moderator()
            .remove_channel_moderator()
            .get_vips()
            .add_channel_vip()
            .remove_channel_vip()
            .update_shield_mode_status()
            .get_shield_mode_status()
            .warn_chat_user()
            .add_suspicious_status_to_chat_user()
            .remove_suspicious_status_from_chat_user()
    }

    /// <https://dev.twitch.tv/docs/api/reference/#check-automod-status>
    pub fn check_automod_status(&mut self) -> &mut Self {
        self.push(Scope::ModerationRead);
        self
    }

    /// <https://dev.twitch.tv/docs/api/reference/#manage-held-automod-messages>
    pub fn manage_held_automod_messages(&mut self) -> &mut Self {
        self.push(Scope::ModeratorManageAutomod);
        self
    }

    /// Accepts `moderator:read:automod_settings` or `moderator:manage:automod_settings`.
    /// Uses `moderator:read:automod_settings`.
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#get-automod-settings>
    pub fn get_automod_settings(&mut self) -> &mut Self {
        self.push(Scope::ModeratorReadAutomodSettings);
        self
    }

    /// <https://dev.twitch.tv/docs/api/reference/#update-automod-settings>
    pub fn update_automod_settings(&mut self) -> &mut Self {
        self.push(Scope::ModeratorManageAutomodSettings);
        self
    }

    /// Accepts `moderation:read` or `moderator:manage:banned_users`.
    /// Uses `moderation:read`.
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#get-banned-users>
    pub fn get_banned_users(&mut self) -> &mut Self {
        self.push(Scope::ModerationRead);
        self
    }

    /// <https://dev.twitch.tv/docs/api/reference/#ban-user>
    pub fn ban_user(&mut self) -> &mut Self {
        self.push(Scope::ModeratorManageBannedUsers);
        self
    }

    /// <https://dev.twitch.tv/docs/api/reference/#unban-user>
    pub fn unban_user(&mut self) -> &mut Self {
        self.push(Scope::ModeratorManageBannedUsers);
        self
    }

    /// Accepts `moderator:read:unban_requests` or `moderator:manage:unban_requests`.
    /// Uses `moderator:read:unban_requests`.
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#get-unban-requests>
    pub fn get_unban_requests(&mut self) -> &mut Self {
        self.push(Scope::ModeratorReadUnbanRequests);
        self
    }

    /// <https://dev.twitch.tv/docs/api/reference/#resolve-unban-requests>
    pub fn resolve_unban_requests(&mut self) -> &mut Self {
        self.push(Scope::ModeratorManageUnbanRequests);
        self
    }

    /// Accepts `moderator:read:blocked_terms` or `moderator:manage:blocked_terms`.
    /// Uses `moderator:read:blocked_terms`.
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#get-blocked-terms>
    pub fn get_blocked_terms(&mut self) -> &mut Self {
        self.push(Scope::ModeratorReadBlockedTerms);
        self
    }

    /// <https://dev.twitch.tv/docs/api/reference/#add-blocked-term>
    pub fn add_blocked_term(&mut self) -> &mut Self {
        self.push(Scope::ModeratorManageBlockedTerms);
        self
    }

    /// <https://dev.twitch.tv/docs/api/reference/#remove-blocked-term>
    pub fn remove_blocked_term(&mut self) -> &mut Self {
        self.push(Scope::ModeratorManageBlockedTerms);
        self
    }

    /// <https://dev.twitch.tv/docs/api/reference/#delete-chat-messages>
    pub fn delete_chat_messages(&mut self) -> &mut Self {
        self.push(Scope::ModeratorManageChatMessages);
        self
    }

    /// <https://dev.twitch.tv/docs/api/reference/#get-moderated-channels>
    pub fn get_moderated_channels(&mut self) -> &mut Self {
        self.push(Scope::UserReadModeratedChannels);
        self
    }

    /// Accepts `moderation:read` or `channel:manage:moderators`.
    /// Uses `moderation:read`.
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#get-moderators>
    pub fn get_moderators(&mut self) -> &mut Self {
        self.push(Scope::ModerationRead);
        self
    }

    /// <https://dev.twitch.tv/docs/api/reference/#add-channel-moderator>
    pub fn add_channel_moderator(&mut self) -> &mut Self {
        self.push(Scope::ChannelManageModerators);
        self
    }

    /// <https://dev.twitch.tv/docs/api/reference/#remove-channel-moderator>
    pub fn remove_channel_moderator(&mut self) -> &mut Self {
        self.push(Scope::ChannelManageModerators);
        self
    }

    /// Accepts `channel:read:vips` or `channel:manage:vips`.
    /// Uses `channel:read:vips`.
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#get-vips>
    pub fn get_vips(&mut self) -> &mut Self {
        self.push(Scope::ChannelReadVips);
        self
    }

    /// <https://dev.twitch.tv/docs/api/reference/#add-channel-vip>
    pub fn add_channel_vip(&mut self) -> &mut Self {
        self.push(Scope::ChannelManageVips);
        self
    }

    /// <https://dev.twitch.tv/docs/api/reference/#remove-channel-vip>
    pub fn remove_channel_vip(&mut self) -> &mut Self {
        self.push(Scope::ChannelManageVips);
        self
    }

    /// <https://dev.twitch.tv/docs/api/reference/#update-shield-mode-status>
    pub fn update_shield_mode_status(&mut self) -> &mut Self {
        self.push(Scope::ModeratorManageShieldMode);
        self
    }

    /// Accepts `moderator:read:shield_mode` or `moderator:manage:shield_mode`.
    /// Uses `moderator:read:shield_mode`.
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#get-shield-mode-status>
    pub fn get_shield_mode_status(&mut self) -> &mut Self {
        self.push(Scope::ModeratorReadShieldMode);
        self
    }

    /// <https://dev.twitch.tv/docs/api/reference/#warn-chat-user>
    pub fn warn_chat_user(&mut self) -> &mut Self {
        self.push(Scope::ModeratorManageWarnings);
        self
    }

    /// <https://dev.twitch.tv/docs/api/reference/#add-suspicious-status-to-chat-user>
    pub fn add_suspicious_status_to_chat_user(&mut self) -> &mut Self {
        self.push(Scope::ModeratorManageSuspiciousUsers);
        self
    }

    /// <https://dev.twitch.tv/docs/api/reference/#remove-suspicious-status-from-chat-user>
    pub fn remove_suspicious_status_from_chat_user(&mut self) -> &mut Self {
        self.push(Scope::ModeratorManageSuspiciousUsers);
        self
    }
}

// Polls
impl ScopesMut<'_> {
    pub fn polls_api(&mut self) -> &mut Self {
        self.get_polls().create_poll().end_poll()
    }

    /// Accepts `channel:read:polls` or `channel:manage:polls`.
    /// Uses `channel:read:polls`.
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#get-polls>
    pub fn get_polls(&mut self) -> &mut Self {
        self.push(Scope::ChannelReadPolls);
        self
    }

    /// <https://dev.twitch.tv/docs/api/reference/#create-poll>
    pub fn create_poll(&mut self) -> &mut Self {
        self.push(Scope::ChannelManagePolls);
        self
    }

    /// <https://dev.twitch.tv/docs/api/reference/#end-poll>
    pub fn end_poll(&mut self) -> &mut Self {
        self.push(Scope::ChannelManagePolls);
        self
    }
}

// Predictions
impl ScopesMut<'_> {
    pub fn predictions_api(&mut self) -> &mut Self {
        self.get_predictions().create_prediction().end_prediction()
    }

    /// Accepts `channel:read:predictions` or `channel:manage:predictions`.
    /// Uses `channel:read:predictions`.
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#get-predictions>
    pub fn get_predictions(&mut self) -> &mut Self {
        self.push(Scope::ChannelReadPredictions);
        self
    }

    /// <https://dev.twitch.tv/docs/api/reference/#create-prediction>
    pub fn create_prediction(&mut self) -> &mut Self {
        self.push(Scope::ChannelManagePredictions);
        self
    }

    /// <https://dev.twitch.tv/docs/api/reference/#end-prediction>
    pub fn end_prediction(&mut self) -> &mut Self {
        self.push(Scope::ChannelManagePredictions);
        self
    }
}

// Raids
impl ScopesMut<'_> {
    pub fn raids_api(&mut self) -> &mut Self {
        self.start_raid().cancel_raid()
    }

    /// <https://dev.twitch.tv/docs/api/reference/#start-a-raid>
    pub fn start_raid(&mut self) -> &mut Self {
        self.push(Scope::ChannelManageRaids);
        self
    }

    /// <https://dev.twitch.tv/docs/api/reference/#cancel-a-raid>
    pub fn cancel_raid(&mut self) -> &mut Self {
        self.push(Scope::ChannelManageRaids);
        self
    }
}

// Schedule
impl ScopesMut<'_> {
    pub fn schedule_api(&mut self) -> &mut Self {
        self.get_channel_stream_schedule()
            .get_channel_icalendar()
            .update_channel_stream_schedule()
            .create_channel_stream_schedule_segment()
            .update_channel_stream_schedule_segment()
            .delete_channel_stream_schedule_segment()
    }

    /// <https://dev.twitch.tv/docs/api/reference/#get-channel-stream-schedule>
    pub fn get_channel_stream_schedule(&mut self) -> &mut Self {
        self
    }

    /// <https://dev.twitch.tv/docs/api/reference/#get-channel-icalendar>
    pub fn get_channel_icalendar(&mut self) -> &mut Self {
        self
    }

    /// <https://dev.twitch.tv/docs/api/reference/#update-channel-stream-schedule>
    pub fn update_channel_stream_schedule(&mut self) -> &mut Self {
        self.push(Scope::ChannelManageSchedule);
        self
    }

    /// <https://dev.twitch.tv/docs/api/reference/#create-channel-stream-schedule-segment>
    pub fn create_channel_stream_schedule_segment(&mut self) -> &mut Self {
        self.push(Scope::ChannelManageSchedule);
        self
    }

    /// <https://dev.twitch.tv/docs/api/reference/#update-channel-stream-schedule-segment>
    pub fn update_channel_stream_schedule_segment(&mut self) -> &mut Self {
        self.push(Scope::ChannelManageSchedule);
        self
    }

    /// <https://dev.twitch.tv/docs/api/reference/#delete-channel-stream-schedule-segment>
    pub fn delete_channel_stream_schedule_segment(&mut self) -> &mut Self {
        self.push(Scope::ChannelManageSchedule);
        self
    }
}

// Search
impl ScopesMut<'_> {
    pub fn search_api(&mut self) -> &mut Self {
        self.search_categories().channels_search()
    }

    /// <https://dev.twitch.tv/docs/api/reference/#search-categories>
    pub fn search_categories(&mut self) -> &mut Self {
        self
    }

    /// <https://dev.twitch.tv/docs/api/reference/#search-channels>
    pub fn channels_search(&mut self) -> &mut Self {
        self
    }
}

// Streams
impl ScopesMut<'_> {
    pub fn streams_api(&mut self) -> &mut Self {
        self.get_stream_key()
            .get_streams()
            .get_followed_streams()
            .create_stream_marker()
            .get_stream_markers()
    }

    /// <https://dev.twitch.tv/docs/api/reference/#get-stream-key>
    pub fn get_stream_key(&mut self) -> &mut Self {
        self.push(Scope::ChannelReadStreamKey);
        self
    }

    /// <https://dev.twitch.tv/docs/api/reference/#get-streams>
    pub fn get_streams(&mut self) -> &mut Self {
        self
    }

    /// <https://dev.twitch.tv/docs/api/reference/#get-followed-streams>
    pub fn get_followed_streams(&mut self) -> &mut Self {
        self.push(Scope::UserReadFollows);
        self
    }

    /// <https://dev.twitch.tv/docs/api/reference/#create-stream-marker>
    pub fn create_stream_marker(&mut self) -> &mut Self {
        self.push(Scope::ChannelManageBroadcast);
        self
    }

    /// Accepts `user:read:broadcast` or `channel:manage:broadcast`.
    /// Uses `user:read:broadcast`.
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#get-stream-markers>
    pub fn get_stream_markers(&mut self) -> &mut Self {
        self.push(Scope::UserReadBroadcast);
        self
    }
}

// Subscriptions
impl ScopesMut<'_> {
    pub fn subscriptions_api(&mut self) -> &mut Self {
        self.get_broadcaster_subscriptions()
            .check_user_subscription()
    }

    /// <https://dev.twitch.tv/docs/api/reference/#get-broadcaster-subscriptions>
    pub fn get_broadcaster_subscriptions(&mut self) -> &mut Self {
        self.push(Scope::ChannelReadSubscriptions);
        self
    }

    /// <https://dev.twitch.tv/docs/api/reference/#check-user-subscription>
    pub fn check_user_subscription(&mut self) -> &mut Self {
        self.push(Scope::UserReadSubscriptions);
        self
    }
}

// Teams
impl ScopesMut<'_> {
    pub fn teams_api(&mut self) -> &mut Self {
        self.get_channel_teams().get_teams()
    }

    /// <https://dev.twitch.tv/docs/api/reference/#get-channel-teams>
    pub fn get_channel_teams(&mut self) -> &mut Self {
        self
    }

    /// <https://dev.twitch.tv/docs/api/reference/#get-teams>
    pub fn get_teams(&mut self) -> &mut Self {
        self
    }
}

// Users
impl ScopesMut<'_> {
    pub fn users_api(&mut self) -> &mut Self {
        self.get_users()
            .update_user()
            .get_authorization_by_user()
            .get_user_block_list()
            .block_unblock_user()
            .get_user_extensions()
            .get_user_active_extensions()
            .update_user_extensions()
    }

    /// <https://dev.twitch.tv/docs/api/reference/#get-users>
    pub fn get_users(&mut self) -> &mut Self {
        self.push(Scope::UserReadEmail);
        self
    }

    /// Uses `user:edit`.
    /// Also adds `user:read:email` to include the verified email address in the response.
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#update-user>
    pub fn update_user(&mut self) -> &mut Self {
        self.extend([Scope::UserReadEmail, Scope::UserEdit]);
        self
    }

    /// <https://dev.twitch.tv/docs/api/reference/#get-authorization-by-user>
    pub fn get_authorization_by_user(&mut self) -> &mut Self {
        self
    }

    /// <https://dev.twitch.tv/docs/api/reference/#get-user-block-list>
    pub fn get_user_block_list(&mut self) -> &mut Self {
        self.push(Scope::UserReadBlockedUsers);
        self
    }

    pub fn block_unblock_user(&mut self) -> &mut Self {
        self.block_user().unblock_user();
        self
    }

    /// <https://dev.twitch.tv/docs/api/reference/#block-user>
    pub fn block_user(&mut self) -> &mut Self {
        self.push(Scope::UserManageBlockedUsers);
        self
    }

    /// <https://dev.twitch.tv/docs/api/reference/#unblock-user>
    pub fn unblock_user(&mut self) -> &mut Self {
        self.push(Scope::UserManageBlockedUsers);
        self
    }

    /// Accepts `user:read:broadcast` or `user:edit:broadcast`.
    /// Uses `user:read:broadcast`.
    /// To include inactive extensions, use `user:edit:broadcast` instead.
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#get-user-extensions>
    pub fn get_user_extensions(&mut self) -> &mut Self {
        self.push(Scope::UserReadBroadcast);
        self
    }

    /// Accepts `user:read:broadcast` or `user:edit:broadcast`.
    /// Uses `user:read:broadcast`.
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#get-user-active-extensions>
    pub fn get_user_active_extensions(&mut self) -> &mut Self {
        self.push(Scope::UserReadBroadcast);
        self
    }

    /// <https://dev.twitch.tv/docs/api/reference/#update-user-extensions>
    pub fn update_user_extensions(&mut self) -> &mut Self {
        self.push(Scope::UserEditBroadcast);
        self
    }
}

// Videos
impl ScopesMut<'_> {
    pub fn videos_api(&mut self) -> &mut Self {
        self.get_videos().delete_videos()
    }

    /// <https://dev.twitch.tv/docs/api/reference/#get-videos>
    pub fn get_videos(&mut self) -> &mut Self {
        self
    }

    /// <https://dev.twitch.tv/docs/api/reference/#delete-videos>
    pub fn delete_videos(&mut self) -> &mut Self {
        self.push(Scope::ChannelManageVideos);
        self
    }
}

// Whispers
impl ScopesMut<'_> {
    pub fn whisper_api(&mut self) -> &mut Self {
        self.send_whisper()
    }

    /// <https://dev.twitch.tv/docs/api/reference/#send-whisper>
    pub fn send_whisper(&mut self) -> &mut Self {
        self.push(Scope::UserManageWhispers);
        self
    }
}

// Chatbot
impl ScopesMut<'_> {
    /// Scopes required for the bot account in cloud chatbots.
    ///
    /// Uses `user:read:chat`, `user:write:chat`, and `user:bot`.
    pub fn cloud_chatbot_account(&mut self) -> &mut Self {
        self.extend([Scope::UserReadChat, Scope::UserWriteChat, Scope::UserBot]);
        self
    }

    /// Scopes required for the broadcaster in cloud chatbots.
    ///
    /// Uses `channel:bot`.
    pub fn cloud_chatbot_broadcaster(&mut self) -> &mut Self {
        self.push(Scope::ChannelBot);
        self
    }

    /// Scopes for installed chatbots.
    ///
    /// Uses `user:read:chat` and `user:write:chat`.
    pub fn installed_chatbot(&mut self) -> &mut Self {
        self.extend([Scope::UserReadChat, Scope::UserWriteChat]);
        self
    }

    /// Scopes for chat clients.
    ///
    /// Uses `user:read:chat` and `user:write:chat`.
    pub fn chat_client(&mut self) -> &mut Self {
        self.extend([Scope::UserReadChat, Scope::UserWriteChat]);
        self
    }
}

// EventSub - Channel Subscriptions
impl ScopesMut<'_> {
    /// Covers `automod.message.hold`, `automod.message.hold.v2`,
    /// `automod.message.update`, and `automod.message.update.v2`.
    pub fn on_automod_message(&mut self) -> &mut Self {
        self.push(Scope::ModeratorManageAutomod);
        self
    }

    /// Covers `automod.settings.update`.
    pub fn on_automod_settings_update(&mut self) -> &mut Self {
        self.push(Scope::ModeratorReadAutomodSettings);
        self
    }

    /// Covers `automod.terms.update`.
    pub fn on_automod_terms_update(&mut self) -> &mut Self {
        self.push(Scope::ModeratorManageAutomod);
        self
    }

    /// Covers `channel.bits.use`.
    pub fn on_channel_bits_use(&mut self) -> &mut Self {
        self.push(Scope::BitsRead);
        self
    }

    /// Covers `channel.update`.
    pub fn on_channel_update(&mut self) -> &mut Self {
        self
    }

    /// Covers `channel.follow`.
    pub fn on_channel_follow(&mut self) -> &mut Self {
        self.push(Scope::ModeratorReadFollowers);
        self
    }

    /// Covers `channel.ad_break.begin`.
    pub fn on_channel_ad_break_begin(&mut self) -> &mut Self {
        self.push(Scope::ChannelReadAds);
        self
    }

    /// Covers `channel.chat.clear`, `channel.chat.clear_user_messages`,
    /// `channel.chat.message`, `channel.chat.message_delete`,
    /// `channel.chat.notification`, `channel.chat.settings_update`,
    /// `channel.chat.user_message_hold`, and `channel.chat.user_message_update`.
    pub fn on_channel_chat(&mut self) -> &mut Self {
        self.push(Scope::UserReadChat);
        self
    }

    /// Covers `channel.subscribe`, `channel.subscription.end`,
    /// `channel.subscription.gift`, and `channel.subscription.message`.
    pub fn on_channel_subscription(&mut self) -> &mut Self {
        self.push(Scope::ChannelReadSubscriptions);
        self
    }

    /// Covers `channel.cheer`.
    pub fn on_channel_cheer(&mut self) -> &mut Self {
        self.push(Scope::BitsRead);
        self
    }

    /// Covers `channel.raid`.
    pub fn on_channel_raid(&mut self) -> &mut Self {
        self
    }

    /// Covers `channel.ban` and `channel.unban`.
    pub fn on_channel_ban(&mut self) -> &mut Self {
        self.push(Scope::ChannelModerate);
        self
    }

    /// Covers `channel.unban_request.create` and `channel.unban_request.resolve`.
    ///
    /// Accepts `moderator:read:unban_requests` or `moderator:manage:unban_requests`.
    /// Uses `moderator:read:unban_requests`.
    pub fn on_channel_unban_request(&mut self) -> &mut Self {
        self.push(Scope::ModeratorReadUnbanRequests);
        self
    }

    /// Covers `channel.moderate` and `channel.moderate.v2`.
    ///
    /// Uses `moderator:read:blocked_terms`, `moderator:read:chat_settings`,
    /// `moderator:read:unban_requests`, `moderator:read:banned_users`,
    /// `moderator:read:chat_messages`, `moderator:read:moderators`,
    /// and `moderator:read:vips`.
    pub fn on_channel_moderate(&mut self) -> &mut Self {
        self.extend([
            Scope::ModeratorReadBlockedTerms,
            Scope::ModeratorReadChatSettings,
            Scope::ModeratorReadUnbanRequests,
            Scope::ModeratorReadBannedUsers,
            Scope::ModeratorReadChatMessages,
            Scope::ModeratorReadModerators,
            Scope::ModeratorReadVips,
        ]);
        self
    }

    /// Covers `channel.moderator.add` and `channel.moderator.remove`.
    pub fn on_channel_moderator(&mut self) -> &mut Self {
        self.push(Scope::ModerationRead);
        self
    }

    /// Covers `channel.guest_star_session.begin`, `channel.guest_star_session.end`,
    /// `channel.guest_star_guest.update`, and `channel.guest_star_settings.update`.
    ///
    /// Accepts `channel:read:guest_star`, `channel:manage:guest_star`,
    /// `moderator:read:guest_star`, or `moderator:manage:guest_star`.
    /// Uses `channel:read:guest_star`.
    pub fn on_channel_guest_star(&mut self) -> &mut Self {
        self.push(Scope::ChannelReadGuestStar);
        self
    }

    /// Covers `channel.channel_points_automatic_reward_redemption.add`,
    /// `channel.channel_points_automatic_reward_redemption.add.v2`,
    /// `channel.channel_points_custom_reward.add`,
    /// `channel.channel_points_custom_reward.update`,
    /// `channel.channel_points_custom_reward.remove`,
    /// `channel.channel_points_custom_reward_redemption.add`,
    /// and `channel.channel_points_custom_reward_redemption.update`.
    ///
    /// Accepts `channel:read:redemptions` or `channel:manage:redemptions`.
    /// Uses `channel:read:redemptions`.
    pub fn on_channel_channel_points(&mut self) -> &mut Self {
        self.push(Scope::ChannelReadRedemptions);
        self
    }

    /// Covers `channel.poll.begin`, `channel.poll.progress`, and `channel.poll.end`.
    ///
    /// Accepts `channel:read:polls` or `channel:manage:polls`.
    /// Uses `channel:read:polls`.
    pub fn on_channel_poll(&mut self) -> &mut Self {
        self.push(Scope::ChannelReadPolls);
        self
    }

    /// Covers `channel.prediction.begin`, `channel.prediction.progress`,
    /// `channel.prediction.lock`, and `channel.prediction.end`.
    ///
    /// Accepts `channel:read:predictions` or `channel:manage:predictions`.
    /// Uses `channel:read:predictions`.
    pub fn on_channel_prediction(&mut self) -> &mut Self {
        self.push(Scope::ChannelReadPredictions);
        self
    }

    /// Covers `channel.suspicious_user.message` and `channel.suspicious_user.update`.
    pub fn on_channel_suspicious_user(&mut self) -> &mut Self {
        self.push(Scope::ModeratorReadSuspiciousUsers);
        self
    }

    /// Covers `channel.vip.add` and `channel.vip.remove`.
    ///
    /// Accepts `channel:read:vips` or `channel:manage:vips`.
    /// Uses `channel:read:vips`.
    pub fn on_channel_vip(&mut self) -> &mut Self {
        self.push(Scope::ChannelReadVips);
        self
    }

    /// Covers `channel.warning.acknowledge` and `channel.warning.send`.
    ///
    /// Accepts `moderator:read:warnings` or `moderator:manage:warnings`.
    /// Uses `moderator:read:warnings`.
    pub fn on_channel_warning(&mut self) -> &mut Self {
        self.push(Scope::ModeratorReadWarnings);
        self
    }

    /// Covers `channel.hype_train.begin`, `channel.hype_train.progress`,
    /// and `channel.hype_train.end`.
    pub fn on_channel_hype_train(&mut self) -> &mut Self {
        self.push(Scope::ChannelReadHypeTrain);
        self
    }

    /// Covers `channel.charity_campaign.donate`,
    /// `channel.charity_campaign.start`,
    /// `channel.charity_campaign.progress`,
    /// and `channel.charity_campaign.stop`.
    pub fn on_channel_charity_campaign(&mut self) -> &mut Self {
        self.push(Scope::ChannelReadCharity);
        self
    }

    /// Covers `channel.shared_chat.begin`, `channel.shared_chat.update`,
    /// and `channel.shared_chat.end`.
    pub fn on_channel_shared_chat(&mut self) -> &mut Self {
        self
    }

    /// Covers `channel.shield_mode.begin` and `channel.shield_mode.end`.
    ///
    /// Accepts `moderator:read:shield_mode` or `moderator:manage:shield_mode`.
    /// Uses `moderator:read:shield_mode`.
    pub fn on_channel_shield_mode(&mut self) -> &mut Self {
        self.push(Scope::ModeratorReadShieldMode);
        self
    }

    /// Covers `channel.shoutout.create` and `channel.shoutout.receive`.
    ///
    /// Accepts `moderator:read:shoutouts` or `moderator:manage:shoutouts`.
    /// Uses `moderator:read:shoutouts`.
    pub fn on_channel_shoutout(&mut self) -> &mut Self {
        self.push(Scope::ModeratorReadShoutouts);
        self
    }

    /// Covers `conduit.shard.disabled`.
    pub fn on_conduit_shard_disabled(&mut self) -> &mut Self {
        self
    }

    /// Covers `drop.entitlement.grant`.
    pub fn on_drop_entitlement_grant(&mut self) -> &mut Self {
        self
    }

    /// Covers `extension.bits_transaction.create`.
    pub fn on_extension_bits_transaction_create(&mut self) -> &mut Self {
        self
    }
}

// EventSub - Goal Subscriptions
impl ScopesMut<'_> {
    /// Covers `channel.goal.begin`, `channel.goal.progress`, and `channel.goal.end`.
    pub fn on_channel_goal(&mut self) -> &mut Self {
        self.push(Scope::ChannelReadGoals);
        self
    }
}

// EventSub - Stream Subscriptions
impl ScopesMut<'_> {
    /// Covers `stream.online` and `stream.offline`.
    pub fn on_stream(&mut self) -> &mut Self {
        self
    }
}

// EventSub - User Subscriptions
impl ScopesMut<'_> {
    /// Covers `user.authorization.grant` and `user.authorization.revoke`.
    pub fn on_user_authorization(&mut self) -> &mut Self {
        self
    }

    /// Covers `user.update`.
    ///
    /// If you have the `user:read:email` scope, the notification will include the email field.
    pub fn on_user_update(&mut self) -> &mut Self {
        self
    }

    /// Covers `user.whisper.message`.
    ///
    /// Accepts `user:read:whispers` or `user:manage:whispers`.
    /// Uses `user:read:whispers`.
    pub fn on_user_whisper_message(&mut self) -> &mut Self {
        self.push(Scope::UserReadWhispers);
        self
    }
}
