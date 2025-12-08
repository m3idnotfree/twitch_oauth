use std::collections::HashSet;

use super::Scope;

pub trait AdScopes {
    fn ads_api(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#start-commercial>
    fn start_commercial(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-ad-schedule>
    fn get_ad_schedule(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#snooze-next-ad>
    fn snooze_next_ad(&mut self) -> &mut Self;
}

pub trait AnalyticScopes {
    fn analytics_api(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-extension-analytics>
    fn get_extension_analytics(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-game-analytics>
    fn get_game_analytics(&mut self) -> &mut Self;
}

pub trait BitScopes {
    fn bits_api(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-bits-leaderboard>
    fn get_bits_leaderboard(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-cheermotes>
    fn get_cheermotes(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-extension-transactions>
    fn get_extension_transactions(&mut self) -> &mut Self;
}

pub trait ChannelScopes {
    fn channel_api(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-channel-information>
    fn get_channel_info(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#modify-channel-information>
    fn modify_channel_info(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-channel-editors>
    fn get_channel_editors(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-followed-channels>
    fn get_followed_channels(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-channel-followers>
    fn get_channel_followers(&mut self) -> &mut Self;
    fn channel_ban_unban(&mut self) -> &mut Self;
}

pub trait ChannelPointScopes {
    fn channel_points_api(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#create-custom-rewards>
    fn create_custom_reward(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#delete-custom-reward>
    fn delete_custom_reward(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-custom-reward>
    fn get_custom_reward(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-custom-reward-redemption>
    fn get_custom_reward_redemption(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#update-custom-reward>
    fn update_custom_reward(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#update-redemption-status>
    fn update_redemption_status(&mut self) -> &mut Self;
}

pub trait CharityScopes {
    fn charity_api(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-charity-campaign>
    fn get_charity_campaign(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-charity-campaign-donations>
    fn get_charity_campaign_donations(&mut self) -> &mut Self;
}

pub trait ChatScopes {
    fn chat_api(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-chatters>
    fn get_chatters(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-channel-emotes>
    fn get_channel_emotes(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-global-emotes>
    fn get_global_emotes(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-emote-sets>
    fn get_emote_sets(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-channel-chat-badges>
    fn get_channel_chat_badges(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-global-chat-badges>
    fn get_global_chat_badges(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-chat-settings>
    fn get_chat_settings(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-shared-chat-session>
    fn get_shard_chat_session(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-user-emotes>
    fn get_user_emotes(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#update-chat-settings>
    fn update_chat_settings(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#send-chat-announcement>
    fn send_chat_announcement(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#send-a-shoutout>
    fn send_shoutout(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#send-chat-message>
    fn send_chat_message(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-user-chat-color>
    fn get_user_chat_color(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#update-user-chat-color>
    fn update_user_chat_color(&mut self) -> &mut Self;
}

pub trait ClipScopes {
    fn clips_api(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#create-clip>
    fn create_clip(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-clips>
    fn get_clip(&mut self) -> &mut Self;
}

pub trait ConduitScopes {
    fn conduits_api(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-conduits>
    fn get_conduits(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#create-conduits>
    fn create_conduits(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#update-conduits>
    fn update_conduits(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#delete-conduit>
    fn delete_conduit(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-conduit-shards>
    fn get_conduit_shards(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#update-conduit-shards>
    fn update_conduit_shards(&mut self)->&mut Self;
}

pub trait CCLScopes {
    fn ccl_api(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-content-classification-labels>
    fn get_content_classification_labels(&mut self) -> &mut Self;
}

pub trait EntitlementScopes {
    fn entitlements_api(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-drops-entitlements>
    fn get_drops_entitlements(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#update-drops-entitlements>
    fn update_drops_entitlements(&mut self) -> &mut Self;
}

pub trait ExtensionScopes {
    fn extensions_api(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-extension-configuration-segment>
    fn get_extension_configuration_segment(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#set-extension-configuration-segment>
    fn set_extension_configuration_segment(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#set-extension-required-configuration>
    fn set_extension_required_configuration(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#send-extension-pubsub-message>
    fn send_extension_pubsub_message(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-extension-live-channels>
    fn get_extension_live_channels(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-extension-secrets>
    fn get_extension_secrets(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#create-extension-secret>
    fn create_extension_secret(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#send-extension-chat-message>
    fn send_extension_chat_message(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-extensions>
    fn get_extensions(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-released-extensions>
    fn get_released_extensions(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-extension-bits-products>
    fn get_extension_bits_products(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#update-extension-bits-product>
    fn update_extension_bits_product(&mut self) -> &mut Self;
}

pub trait EventSubScopes {
    fn eventsub_api(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#create-eventsub-subscription>
    fn create_eventsub(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#delete-eventsub-subscription>
    fn delete_eventsub(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-eventsub-subscriptions>
    fn get_eventsub(&mut self) -> &mut Self;
}

pub trait GameScopes {
    fn games_api(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-top-games>
    fn get_top_games(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-games>
    fn get_games(&mut self) -> &mut Self;
}

pub trait GoalScopes {
    fn goals_api(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-creator-goals>
    fn get_creator_goals(&mut self) -> &mut Self;
}

pub trait GuestStarScopes {
    fn guest_star_api(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-channel-guest-star-settings>
    fn get_channel_guest_star_setings(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#update-channel-guest-star-settings>
    fn update_channel_guest_star_setings(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-guest-star-session>
    fn get_guest_star_session(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#create-guest-star-session>
    fn create_guest_star_session(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#end-guest-star-session>
    fn end_guest_star_session(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-guest-star-invites>
    fn get_guest_star_invites(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#send-guest-star-invite>
    fn send_guest_star_invite(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#delete-guest-star-invite>
    fn delete_guest_star_invite(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#assign-guest-star-slot>
    fn assign_guest_star_slot(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#update-guest-star-slot>
    fn update_guest_star_slot(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#delete-guest-star-slot>
    fn delete_guest_star_slot(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#update-guest-star-slot-settings>
    fn update_guest_star_slot_settings(&mut self) -> &mut Self;
}

pub trait HypeTrainScopes {
    fn hype_train_api(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-hype-train-events>
    fn get_hype_train_events(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-hype-train-status>
    fn get_hype_train_status(&mut self) -> &mut Self;
}

pub trait ModerationScopes {
    fn moderation_api(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#check-automod-status>
    fn check_automod_status(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#manage-held-automod-messages>
    fn manage_held_automod_messages(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-automod-settings>
    fn get_automod_settings(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#update-automod-settings>
    fn update_automod_settings(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-banned-users>
    fn get_banned_users(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#ban-user>
    fn ban_user(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#unban-user>
    fn unban_user(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-unban-requests>
    fn get_unban_requests(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#resolve-unban-requests>
    fn resolve_unban_requests(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-blocked-terms>
    fn get_blocked_terms(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#add-blocked-term>
    fn add_blocked_term(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#remove-blocked-term>
    fn remove_blocked_term(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#delete-chat-messages>
    fn delete_chat_messages(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-moderated-channels>
    fn get_moderated_channels(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-moderators>
    fn get_moderators(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#add-channel-moderator>
    fn add_channel_moderator(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#remove-channel-moderator>
    fn remove_channel_moderator(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-vips>
    fn get_vips(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#add-channel-vip>
    fn add_channel_vip(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#remove-channel-vip>
    fn remove_channel_vip(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#update-shield-mode-status>
    fn update_shield_mode_status(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-shield-mode-status>
    fn get_shield_mode_status(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#warn-chat-user>
    fn warn_chat_user(&mut self) -> &mut Self;
}

pub trait PollScopes {
    fn polls_api(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-polls>
    fn get_polls(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#create-poll>
    fn create_poll(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#end-poll>
    fn end_poll(&mut self) -> &mut Self;
}

pub trait PredictionScopes {
    fn predictions_api(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-predictions>
    fn get_predictions(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#create-prediction>
    fn create_prediction(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#end-prediction>
    fn end_prediction(&mut self) -> &mut Self;
}

pub trait RaidScopes {
    fn raids_api(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#start-a-raid>
    fn start_raid(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#cancel-a-raid>
    fn cancel_raid(&mut self) -> &mut Self;
}

pub trait ScheduleScopes {
    fn schedule_api(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-channel-stream-schedule>
    fn get_channel_stream_schedule(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-channel-icalendar>
    fn get_channel_icalendar(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#update-channel-stream-schedule>
    fn update_channel_stream_schedule(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#create-channel-stream-schedule-segment>
    fn create_channel_stream_schedule_segment(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#update-channel-stream-schedule-segment>
    fn update_channel_stream_schedule_segment(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#delete-channel-stream-schedule-segment>
    fn delete_channel_stream_schedule_segment(&mut self) -> &mut Self;
}

pub trait SearchScopes {
    fn search_api(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#search-categories>
    fn search_categories(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#search-channels>
    fn channels_search(&mut self) -> &mut Self;
}

pub trait StreamScopes {
    fn streams_api(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-stream-key>
    fn get_stream_key(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-streams>
    fn get_streams(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-followed-streams>
    fn get_followed_streams(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#create-stream-marker>
    fn create_stream_marker(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-stream-markers>
    fn get_stream_markers(&mut self) -> &mut Self;
}

pub trait SubscriptionScopes {
    fn subscriptions_api(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-broadcaster-subscriptions>
    fn get_broadcaster_subscriptions(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#check-user-subscription>
    fn check_user_subscription(&mut self) -> &mut Self;
}

pub trait TagScopes {
    fn tags_api(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-all-stream-tags>
    fn get_all_stream_tags(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-stream-tags>
    fn get_stream_tags(&mut self) -> &mut Self;
}

pub trait TeamScopes {
    fn teams_api(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-channel-teams>
    fn get_channel_teams(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-teams>
    fn get_teams(&mut self) -> &mut Self;
}

pub trait UserScopes {
    fn users_api(&mut self) -> &mut Self;
    /// user:read:email
    /// <https://dev.twitch.tv/docs/api/reference/#get-users>
    fn get_users(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#update-user>
    fn update_user(&mut self) -> &mut Self;
    /// user:read:blocked_users
    /// <https://dev.twitch.tv/docs/api/reference/#get-user-block-list>
    fn get_user_block_list(&mut self) -> &mut Self;
    /// user:manage:blocked_users
    /// <https://dev.twitch.tv/docs/api/reference/#block-user>
    /// <https://dev.twitch.tv/docs/api/reference/#unblock-user>
    fn block_unblock_user(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-user-active-extensions>
    fn get_user_active_extensions(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#update-user-extensions>
    fn update_user_extensions(&mut self) -> &mut Self;
}

pub trait VideoScopes {
    fn videos_api(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-videos>
    fn get_videos(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#delete-videos>
    fn delete_videos(&mut self) -> &mut Self;
}

pub trait WhisperScopes {
    fn whisper_api(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#send-whisper>
    fn send_whisper(&mut self) -> &mut Self;
}

pub trait IRCScopes {
    /// Add all IRC scopes (chat:edit, chat:read)
    fn irc_all(&mut self) -> &mut Self;
    /// Add IRC chat edit scope
    fn irc_chat_edit(&mut self) -> &mut Self;
    /// Add IRC chat read scope
    fn irc_chat_read(&mut self) -> &mut Self;
}

pub trait ChatbotScopes {
    /// Scopes required for the bot account in cloud chatbots
    ///
    /// Scopes: user:read:chat, user:write:chat, user:bot
    fn cloud_chatbot_account(&mut self) -> &mut Self;
    /// Scopes required for the broadcaster in cloud chatbots
    ///
    /// Scopes: channel:bot
    fn cloud_chatbot_broadcaster(&mut self) -> &mut Self;
    /// Scopes for installed chatbots
    ///
    /// Scopes: user:read:chat, user:write:chat
    fn installed_chatbot(&mut self) -> &mut Self;
    /// Scopes for chat clients
    ///
    /// Scopes: user:read:chat, user:write:chat
    fn chat_client(&mut self) -> &mut Self;
}
impl AdScopes for ScopesMut<'_> {
    fn ads_api(&mut self) -> &mut Self {
        self.start_commercial()
            .get_ad_schedule()
            .snooze_next_ad()
    }
    fn start_commercial(&mut self) -> &mut Self {
        self.push(Scope::ChannelEditCommercial);
        self
    }
    fn get_ad_schedule(&mut self) -> &mut Self {
        self.push(Scope::ChannelReadAds);
        self
    }
    fn snooze_next_ad(&mut self) -> &mut Self {
        self.push(Scope::ChannelManageAds);
        self
    }
}

impl AnalyticScopes for ScopesMut<'_> {
    fn analytics_api(&mut self) -> &mut Self {
        self.get_extension_analytics()
            .get_game_analytics()
    }
    fn get_extension_analytics(&mut self) -> &mut Self {
        self.push(Scope::AnalyticsReadExtensions);
        self
    }
    fn get_game_analytics(&mut self) -> &mut Self {
        self.push(Scope::AnalyticsReadGames);
        self
    }
}

impl BitScopes for ScopesMut<'_> {
    fn bits_api(&mut self) -> &mut Self {
        self.get_bits_leaderboard()
            .get_cheermotes()
            .get_extension_transactions()
    }
    fn get_bits_leaderboard(&mut self) -> &mut Self {
        self.push(Scope::BitsRead);
        self
    }
    fn get_cheermotes(&mut self) -> &mut Self {
        self
    }
    fn get_extension_transactions(&mut self) -> &mut Self {
        self
    }
}

impl ChannelScopes for ScopesMut<'_> {
    fn channel_api(&mut self) -> &mut Self {
        self.get_channel_info()
            .modify_channel_info()
            .get_channel_editors()
            .get_followed_channels()
            .get_channel_followers()
    }
    fn get_channel_info(&mut self) -> &mut Self {
        self
    }
    fn modify_channel_info(&mut self) -> &mut Self {
        self.push(Scope::ChannelManageBroadcast);
        self
    }
    fn get_channel_editors(&mut self) -> &mut Self {
        self.push(Scope::ChannelReadEditors);
        self
    }
    fn get_followed_channels(&mut self) -> &mut Self {
        self.push(Scope::UserReadFollows);
        self
    }
    fn get_channel_followers(&mut self) -> &mut Self {
        self.push(Scope::ModeratorReadFollowers);
        self
    }
    fn channel_ban_unban(&mut self) -> &mut Self {
        self.push(Scope::ChannelModerate);
        self
    }
}

impl ChannelPointScopes for ScopesMut<'_> {
    fn channel_points_api(&mut self) -> &mut Self {
        self.create_custom_reward()
            .delete_custom_reward()
            .get_custom_reward()
            .get_custom_reward_redemption()
            .update_custom_reward()
            .update_redemption_status()
    }
    fn create_custom_reward(&mut self) -> &mut Self {
        self.push(Scope::ChannelManageRedemptions);
        self
    }
    fn delete_custom_reward(&mut self) -> &mut Self {
        self.push(Scope::ChannelManageRedemptions);
        self
    }
    fn get_custom_reward(&mut self) -> &mut Self {
        self.push(Scope::ChannelReadRedemptions);
        self
    }
    fn get_custom_reward_redemption(&mut self) -> &mut Self {
        self.push(Scope::ChannelReadRedemptions);
        self
    }
    fn update_custom_reward(&mut self) -> &mut Self {
        self.push(Scope::ChannelManageRedemptions);
        self
    }
    fn update_redemption_status(&mut self) -> &mut Self {
        self.push(Scope::ChannelManageRedemptions);
        self
    }
}

impl CharityScopes for ScopesMut<'_> {
    fn charity_api(&mut self) -> &mut Self {
        self.get_charity_campaign()
            .get_charity_campaign_donations()
    }
    fn get_charity_campaign(&mut self) -> &mut Self {
        self.push(Scope::ChannelReadCharity);
        self
    }
    fn get_charity_campaign_donations(&mut self) -> &mut Self {
        self.push(Scope::ChannelReadCharity);
        self
    }
}

impl ChatScopes for ScopesMut<'_> {
    fn chat_api(&mut self) -> &mut Self {
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
    fn get_chatters(&mut self) -> &mut Self {
        self.push(Scope::ModeratorReadChatters);
        self
    }
    fn get_channel_emotes(&mut self) -> &mut Self {
        self
    }
    fn get_global_emotes(&mut self) -> &mut Self {
        self
    }
    fn get_emote_sets(&mut self) -> &mut Self {
        self
    }
    fn get_channel_chat_badges(&mut self) -> &mut Self {
        self
    }
    fn get_global_chat_badges(&mut self) -> &mut Self {
        self
    }
    fn get_chat_settings(&mut self) -> &mut Self {
        self
    }
    fn get_shard_chat_session(&mut self) -> &mut Self {
        self
    }
    fn get_user_emotes(&mut self) -> &mut Self {
        self.push(Scope::UserReadEmotes);
        self
    }
    fn update_chat_settings(&mut self) -> &mut Self {
        self.push(Scope::ModeratorManageChatSettings);
        self
    }
    fn send_chat_announcement(&mut self) -> &mut Self {
        self.push(Scope::ModeratorManageAnnouncements);
        self
    }
    fn send_shoutout(&mut self) -> &mut Self {
        self
    }
    fn send_chat_message(&mut self) -> &mut Self {
        self.push(Scope::UserWriteChat);
        self
    }
    fn get_user_chat_color(&mut self) -> &mut Self {
        self
    }
    fn update_user_chat_color(&mut self) -> &mut Self {
        self.push(Scope::UserManageChatColor);
        self
    }
}

impl ClipScopes for ScopesMut<'_> {
    fn clips_api(&mut self) -> &mut Self {
        self.create_clip().get_clip()
    }
    fn create_clip(&mut self) -> &mut Self {
        self.push(Scope::ClipsEdit);
        self
    }
    fn get_clip(&mut self) -> &mut Self {
        self
    }
}

impl ConduitScopes for ScopesMut<'_> {
    fn conduits_api(&mut self) -> &mut Self {
        self.get_conduits()
            .create_conduits()
            .update_conduits()
            .delete_conduit()
            .get_conduit_shards()
            .update_conduit_shards()
    }
    fn get_conduits(&mut self) -> &mut Self {
        self
    }
    fn create_conduits(&mut self) -> &mut Self {
        self
    }
    fn update_conduits(&mut self) -> &mut Self {
        self
    }
    fn delete_conduit(&mut self) -> &mut Self {
        self
    }
    fn get_conduit_shards(&mut self) -> &mut Self {
        self
    }
    fn update_conduit_shards(&mut self)->&mut Self{
        self
    }
}

impl CCLScopes for ScopesMut<'_> {
    fn ccl_api(&mut self) -> &mut Self {
        self.get_content_classification_labels()
    }
    fn get_content_classification_labels(&mut self) -> &mut Self {
        self
    }
}

impl EntitlementScopes for ScopesMut<'_> {
    fn entitlements_api(&mut self) -> &mut Self {
        self.get_drops_entitlements()
            .update_drops_entitlements()
    }
    fn get_drops_entitlements(&mut self) -> &mut Self {
        self
    }
    fn update_drops_entitlements(&mut self) -> &mut Self {
        self
    }
}

impl ExtensionScopes for ScopesMut<'_> {
    fn extensions_api(&mut self) -> &mut Self {
        self.get_extension_configuration_segment()
            .set_extension_configuration_segment()
            .set_extension_required_configuration()
            .send_extension_pubsub_message()
            .get_extension_live_channels()
            .get_extension_secrets()
            .create_extension_secret()
            .send_extension_chat_message()
            .get_extensions()
            .get_released_extensions()
            .get_extension_bits_products()
            .update_extension_bits_product()
    }
    fn get_extension_configuration_segment(&mut self) -> &mut Self {
        self
    }
    fn set_extension_configuration_segment(&mut self) -> &mut Self {
        self
    }
    fn set_extension_required_configuration(&mut self) -> &mut Self {
        self
    }
    fn send_extension_pubsub_message(&mut self) -> &mut Self {
        self
    }
    fn get_extension_live_channels(&mut self) -> &mut Self {
        self
    }
    fn get_extension_secrets(&mut self) -> &mut Self {
        self
    }
    fn create_extension_secret(&mut self) -> &mut Self {
        self
    }
    fn send_extension_chat_message(&mut self) -> &mut Self {
        self
    }
    fn get_extensions(&mut self) -> &mut Self {
        self
    }
    fn get_released_extensions(&mut self) -> &mut Self {
        self
    }
    fn get_extension_bits_products(&mut self) -> &mut Self {
        self
    }
    fn update_extension_bits_product(&mut self) -> &mut Self {
        self
    }
}

impl EventSubScopes for ScopesMut<'_> {
    fn eventsub_api(&mut self) -> &mut Self {
        self.create_eventsub()
            .delete_eventsub()
            .get_eventsub()
    }
    fn create_eventsub(&mut self) -> &mut Self {
        self
    }
    fn delete_eventsub(&mut self) -> &mut Self {
        self
    }
    fn get_eventsub(&mut self) -> &mut Self {
        self
    }
}

impl GameScopes for ScopesMut<'_> {
    fn games_api(&mut self) -> &mut Self {
        self.get_top_games().get_games()
    }
    fn get_top_games(&mut self) -> &mut Self {
        self
    }
    fn get_games(&mut self) -> &mut Self {
        self
    }
}

impl GoalScopes for ScopesMut<'_> {
    fn goals_api(&mut self) -> &mut Self {
        self.get_creator_goals()
    }
    fn get_creator_goals(&mut self) -> &mut Self {
        self.push(Scope::ChannelReadGoals);
        self
    }
}

impl GuestStarScopes for ScopesMut<'_> {
    fn guest_star_api(&mut self) -> &mut Self {
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
    fn get_channel_guest_star_setings(&mut self) -> &mut Self {
        self.extend([
            Scope::ChannelReadGuestStar,
            Scope::ChannelManageGuestStar,
            Scope::ModeratorReadGuestStar,
        ]);
        self
    }
    fn update_channel_guest_star_setings(&mut self) -> &mut Self {
        self.push(Scope::ChannelManageGuestStar);
        self
    }
    fn get_guest_star_session(&mut self) -> &mut Self {
        self.extend([
            Scope::ChannelReadGuestStar,
            Scope::ChannelManageGuestStar,
            Scope::ModeratorReadGuestStar,
        ]);
        self
    }
    fn create_guest_star_session(&mut self) -> &mut Self {
        self.push(Scope::ChannelManageGuestStar);
        self
    }
    fn end_guest_star_session(&mut self) -> &mut Self {
        self.push(Scope::ChannelManageGuestStar);
        self
    }
    fn get_guest_star_invites(&mut self) -> &mut Self {
        self.extend([
            Scope::ChannelReadGuestStar,
            Scope::ChannelManageGuestStar,
            Scope::ModeratorReadGuestStar,
        ]);
        self
    }
    fn send_guest_star_invite(&mut self) -> &mut Self {
        self.push(Scope::ChannelManageGuestStar);
        self
    }
    fn delete_guest_star_invite(&mut self) -> &mut Self {
        self.push(Scope::ChannelManageGuestStar);
        self
    }
    fn assign_guest_star_slot(&mut self) -> &mut Self {
        self.push(Scope::ChannelManageGuestStar);
        self
    }
    fn update_guest_star_slot(&mut self) -> &mut Self {
        self.push(Scope::ChannelManageGuestStar);
        self
    }
    fn delete_guest_star_slot(&mut self) -> &mut Self {
        self.push(Scope::ChannelManageGuestStar);
        self
    }
    fn update_guest_star_slot_settings(&mut self) -> &mut Self {
        self.push(Scope::ChannelManageGuestStar);
        self
    }
}

impl HypeTrainScopes for ScopesMut<'_> {
    fn hype_train_api(&mut self) -> &mut Self {
        self.get_hype_train_events()
            .get_hype_train_status()
    }
    fn get_hype_train_events(&mut self) -> &mut Self {
        self.push(Scope::ChannelReadHypeTrain);
        self
    }
    fn get_hype_train_status(&mut self) -> &mut Self{
        self.push(Scope::ChannelReadHypeTrain);
        self
    }
}

impl ModerationScopes for ScopesMut<'_> {
    fn moderation_api(&mut self) -> &mut Self {
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
    }
    fn check_automod_status(&mut self) -> &mut Self {
        self.push(Scope::ModerationRead);
        self
    }
    fn manage_held_automod_messages(&mut self) -> &mut Self {
        self.push(Scope::ModeratorManageAutomod);
        self
    }
    fn get_automod_settings(&mut self) -> &mut Self {
        self.push(Scope::ModeratorReadAutomodSettings);
        self
    }
    fn update_automod_settings(&mut self) -> &mut Self {
        self.push(Scope::ModeratorManageAutomodSettings);
        self
    }
    fn get_banned_users(&mut self) -> &mut Self {
        self.push(Scope::ModerationRead);
        self
    }
    fn ban_user(&mut self) -> &mut Self {
        self.push(Scope::ModeratorManageBannedUsers);
        self
    }
    fn unban_user(&mut self) -> &mut Self {
        self.push(Scope::ModeratorManageBannedUsers);
        self
    }
    fn get_unban_requests(&mut self) -> &mut Self {
        self.push(Scope::ModeratorReadUnbanRequests);
        self
    }
    fn resolve_unban_requests(&mut self) -> &mut Self {
        self.push(Scope::ModeratorReadUnbanRequests);
        self
    }
    fn get_blocked_terms(&mut self) -> &mut Self {
        self.push(Scope::ModeratorReadBlockedTerms);
        self
    }
    fn add_blocked_term(&mut self) -> &mut Self {
        self.push(Scope::ModeratorManageBlockedTerms);
        self
    }
    fn remove_blocked_term(&mut self) -> &mut Self {
        self.push(Scope::ModeratorManageBlockedTerms);
        self
    }
    fn delete_chat_messages(&mut self) -> &mut Self {
        self.push(Scope::ModeratorManageChatMessages);
        self
    }
    fn get_moderated_channels(&mut self) -> &mut Self {
        self.push(Scope::UserReadModeratedChannels);
        self
    }
    fn get_moderators(&mut self) -> &mut Self {
        self.push(Scope::ModerationRead);
        self
    }
    fn add_channel_moderator(&mut self) -> &mut Self {
        self.push(Scope::ChannelManageModerators);
        self
    }
    fn remove_channel_moderator(&mut self) -> &mut Self {
        self.push(Scope::ChannelManageModerators);
        self
    }
    fn get_vips(&mut self) -> &mut Self {
        self.push(Scope::ChannelReadVips);
        self
    }
    fn add_channel_vip(&mut self) -> &mut Self {
        self.push(Scope::ChannelManageVips);
        self
    }
    fn remove_channel_vip(&mut self) -> &mut Self {
        self.push(Scope::ChannelManageVips);
        self
    }
    fn update_shield_mode_status(&mut self) -> &mut Self {
        self.push(Scope::ModeratorManageShieldMode);
        self
    }
    fn get_shield_mode_status(&mut self) -> &mut Self {
        self.push(Scope::ModeratorReadShieldMode);
        self
    }
    fn warn_chat_user(&mut self) -> &mut Self {
        self.push(Scope::ModeratorManageWarnings);
        self
    }
}

impl PollScopes for ScopesMut<'_> {
    fn polls_api(&mut self) -> &mut Self {
        self.get_polls().create_poll().end_poll()
    }
    fn get_polls(&mut self) -> &mut Self {
        self.push(Scope::ChannelReadPolls);
        self
    }
    fn create_poll(&mut self) -> &mut Self {
        self.push(Scope::ChannelManagePolls);
        self
    }
    fn end_poll(&mut self) -> &mut Self {
        self.push(Scope::ChannelManagePolls);
        self
    }
}

impl PredictionScopes for ScopesMut<'_> {
    fn predictions_api(&mut self) -> &mut Self {
        self.get_predictions()
            .create_prediction()
            .end_prediction()
    }
    fn get_predictions(&mut self) -> &mut Self {
        self.push(Scope::ChannelReadPredictions);
        self
    }
    fn create_prediction(&mut self) -> &mut Self {
        self.push(Scope::ChannelManagePredictions);
        self
    }
    fn end_prediction(&mut self) -> &mut Self {
        self.push(Scope::ChannelManagePredictions);
        self
    }
}

impl RaidScopes for ScopesMut<'_> {
    fn raids_api(&mut self) -> &mut Self {
        self.start_raid().cancel_raid()
    }
    fn start_raid(&mut self) -> &mut Self {
        self.push(Scope::ChannelManageRaids);
        self
    }
    fn cancel_raid(&mut self) -> &mut Self {
        self.push(Scope::ChannelManageRaids);
        self
    }
}

impl ScheduleScopes for ScopesMut<'_> {
    fn schedule_api(&mut self) -> &mut Self {
        self.get_channel_stream_schedule()
            .get_channel_icalendar()
            .update_channel_stream_schedule()
            .create_channel_stream_schedule_segment()
            .update_channel_stream_schedule_segment()
            .delete_channel_stream_schedule_segment()
    }
    fn get_channel_stream_schedule(&mut self) -> &mut Self {
        self
    }
    fn get_channel_icalendar(&mut self) -> &mut Self {
        self
    }
    fn update_channel_stream_schedule(&mut self) -> &mut Self {
        self.push(Scope::ChannelManageSchedule);
        self
    }
    fn create_channel_stream_schedule_segment(&mut self) -> &mut Self {
        self.push(Scope::ChannelManageSchedule);
        self
    }
    fn update_channel_stream_schedule_segment(&mut self) -> &mut Self {
        self.push(Scope::ChannelManageSchedule);
        self
    }
    fn delete_channel_stream_schedule_segment(&mut self) -> &mut Self {
        self.push(Scope::ChannelManageSchedule);
        self
    }
}

impl SearchScopes for ScopesMut<'_> {
    fn search_api(&mut self) -> &mut Self {
        self.search_categories().channels_search()
    }
    fn search_categories(&mut self) -> &mut Self {
        self
    }
    fn channels_search(&mut self) -> &mut Self {
        self
    }
}

impl StreamScopes for ScopesMut<'_> {
    fn streams_api(&mut self) -> &mut Self {
        self.get_stream_key()
            .get_streams()
            .get_followed_streams()
            .create_stream_marker()
            .get_stream_markers()
    }
    fn get_stream_key(&mut self) -> &mut Self {
        self.push(Scope::ChannelReadStreamKey);
        self
    }
    fn get_streams(&mut self) -> &mut Self {
        self
    }
    fn get_followed_streams(&mut self) -> &mut Self {
        self.push(Scope::UserReadFollows);
        self
    }
    fn create_stream_marker(&mut self) -> &mut Self {
        self.push(Scope::ChannelManageBroadcast);
        self
    }
    fn get_stream_markers(&mut self) -> &mut Self {
        self.push(Scope::UserReadBroadcast);
        self
    }
}

impl SubscriptionScopes for ScopesMut<'_> {
    fn subscriptions_api(&mut self) -> &mut Self {
        self.get_broadcaster_subscriptions()
            .check_user_subscription()
    }
    fn get_broadcaster_subscriptions(&mut self) -> &mut Self {
        self.push(Scope::ChannelReadSubscriptions);
        self
    }
    fn check_user_subscription(&mut self) -> &mut Self {
        self.push(Scope::UserReadSubscriptions);
        self
    }
}

impl TagScopes for ScopesMut<'_> {
    fn tags_api(&mut self) -> &mut Self {
        self.get_all_stream_tags().get_stream_tags()
    }
    fn get_all_stream_tags(&mut self) -> &mut Self {
        self
    }
    fn get_stream_tags(&mut self) -> &mut Self {
        self
    }
}

impl TeamScopes for ScopesMut<'_> {
    fn teams_api(&mut self) -> &mut Self {
        self.get_channel_teams().get_teams()
    }
    fn get_channel_teams(&mut self) -> &mut Self {
        self
    }
    fn get_teams(&mut self) -> &mut Self {
        self
    }
}
impl UserScopes for ScopesMut<'_> {
    fn users_api(&mut self) -> &mut Self {
        self.get_users()
            .update_user()
            .get_user_block_list()
            .block_unblock_user()
            .get_user_active_extensions()
            .update_user_extensions()
    }
    fn get_users(&mut self) -> &mut Self {
        self.push(Scope::UserReadEmail);
        self
    }
    fn update_user(&mut self) -> &mut Self {
        self.extend([Scope::UserReadEmail, Scope::UserEdit]);
        self
    }
    fn get_user_block_list(&mut self) -> &mut Self {
        self.push(Scope::UserReadBlockedUsers);
        self
    }
    fn block_unblock_user(&mut self) -> &mut Self {
        self.push(Scope::UserManageBlockedUsers);
        self
    }
    fn get_user_active_extensions(&mut self) -> &mut Self {
        self.extend([Scope::UserReadBroadcast, Scope::UserEditBroadcast]);
        self
    }
    fn update_user_extensions(&mut self) -> &mut Self {
        self.push(Scope::UserEditBroadcast);
        self
    }
}

impl VideoScopes for ScopesMut<'_> {
    fn videos_api(&mut self) -> &mut Self {
        self.get_videos().delete_videos()
    }
    fn get_videos(&mut self) -> &mut Self {
        self
    }
    fn delete_videos(&mut self) -> &mut Self {
        self.push(Scope::ChannelManageVideos);
        self
    }
}

impl WhisperScopes for ScopesMut<'_> {
    fn whisper_api(&mut self) -> &mut Self {
        self.send_whisper()
    }
    fn send_whisper(&mut self) -> &mut Self {
        self.push(Scope::UserManageWhispers);
        self
    }
}

impl IRCScopes for ScopesMut<'_> {
    fn irc_all(&mut self) -> &mut Self {
        self.extend([Scope::ChatEdit, Scope::ChatRead]);
        self
    }
    fn irc_chat_edit(&mut self) -> &mut Self {
        self.push(Scope::ChatEdit);
        self
    }
    fn irc_chat_read(&mut self) -> &mut Self {
        self.push(Scope::ChatRead);
        self
    }
}

impl ChatbotScopes for ScopesMut<'_> {
    fn cloud_chatbot_account(&mut self) -> &mut Self {
        self.extend([Scope::UserReadChat, Scope::UserWriteChat, Scope::UserBot]);
        self
    }
    fn cloud_chatbot_broadcaster(&mut self) -> &mut Self {
        self.push(Scope::ChannelBot);
        self
    }
    fn installed_chatbot(&mut self) -> &mut Self {
        self.extend([Scope::UserReadChat, Scope::UserWriteChat]);
        self
    }
    fn chat_client(&mut self) -> &mut Self {
        self.extend([Scope::UserReadChat, Scope::UserWriteChat]);
        self
    }
}
/// inspired PathSegmentsMut
/// <https://docs.rs/url/latest/src/url/path_segments.rs.html#37-42>
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
