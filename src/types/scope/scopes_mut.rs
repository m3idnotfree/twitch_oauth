use std::collections::HashSet;

use super::Scope;

pub trait AdsScopes {
    fn with_ads_api(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#start-commercial>
    fn with_stard_commercial(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-ad-schedule>
    fn with_ad_schedule_read(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#snooze-next-ad>
    fn with_snooze_next_ad(&mut self) -> &mut Self;
}

pub trait AnalyticsScopes {
    fn with_analytics_api(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-extension-analytics>
    fn with_extension_analytics_read(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-game-analytics>
    fn with_game_analytics_read(&mut self) -> &mut Self;
}

pub trait BitsScopes {
    fn with_bits_api(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-bits-leaderboard>
    fn with_bits_leaderboard_read(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-cheermotes>
    fn with_cheermotes_read(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-extension-transactions>
    fn with_extension_transactions_read(&mut self) -> &mut Self;
}

pub trait ChannelScopes {
    fn with_channel_api(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-channel-information>
    fn with_channel_info_read(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#modify-channel-information>
    fn with_channel_info_manage(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-channel-editors>
    fn with_channel_editors_read(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-followed-channels>
    fn with_followed_channels_read(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-channel-followers>
    fn with_channel_followers_read(&mut self) -> &mut Self;
    fn with_channel_ban_unban(&mut self) -> &mut Self;
}

pub trait ChannelPointsScopes {
    fn with_channel_points_api(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#create-custom-rewards>
    fn with_custom_rewards_create(&mut self) -> &mut Self;
    fn with_custom_reward_delete(&mut self) -> &mut Self;
    fn with_custom_reward_read(&mut self) -> &mut Self;
    fn with_custom_reward_redemption(&mut self) -> &mut Self;
    fn with_custom_reward_manage(&mut self) -> &mut Self;
    fn with_redemption_status_manage(&mut self) -> &mut Self;
}

pub trait CharityScopes {
    fn with_charity_api(&mut self) -> &mut Self;
    fn with_charity_campaign_read(&mut self) -> &mut Self;
    fn with_charity_campaign_notations_read(&mut self) -> &mut Self;
}

pub trait ChatScopes {
    // Chat
    fn with_chat_api(&mut self) -> &mut Self;
    /// moderator:read:chatters
    /// <https://dev.twitch.tv/docs/api/reference/#get-chatters>
    fn with_chatters(&mut self) -> &mut Self;
    fn with_channel_emotes(&mut self) -> &mut Self;
    fn with_global_emotes(&mut self) -> &mut Self;
    fn with_channel_badges(&mut self) -> &mut Self;
    fn with_global_badges(&mut self) -> &mut Self;
    fn with_chat_setting_read(&mut self) -> &mut Self;
    fn with_shard_chat_session_read(&mut self) -> &mut Self;
    /// user:read:emotes
    /// <https://dev.twitch.tv/docs/api/reference/#get-user-emotes>
    fn with_user_emotes_read(&mut self) -> &mut Self;
    fn with_chat_setting_manage(&mut self) -> &mut Self;
    fn with_chat_announcement_write(&mut self) -> &mut Self;
    fn with_shoutout_write(&mut self) -> &mut Self;
    /// user:write:chat
    /// <https://dev.twitch.tv/docs/api/reference/#send-chat-message>
    fn with_chat_write(&mut self) -> &mut Self;
    /// user:write:chat user:bot channel:bot
    fn with_chat_write_as_app(&mut self) -> &mut Self;
    fn with_user_color_read(&mut self) -> &mut Self;
    fn with_user_color_manage(&mut self) -> &mut Self;
}

pub trait ClipsScopes {
    fn with_clips_api(&mut self) -> &mut Self;
    fn with_clip_create(&mut self) -> &mut Self;
    fn with_clip_read(&mut self) -> &mut Self;
}

pub trait ConduitsScopes {
    fn with_conduits_api(&mut self) -> &mut Self;
    fn with_conduits_read(&mut self) -> &mut Self;
    fn with_conduits_create(&mut self) -> &mut Self;
    fn with_conduits_manage(&mut self) -> &mut Self;
    fn with_conduit_delete(&mut self) -> &mut Self;
    fn with_conduit_shards_read(&mut self) -> &mut Self;
}

pub trait CCLsScopes {
    fn with_cclrs_api(&mut self) -> &mut Self;
    fn with_content_classification_labels_read(&mut self) -> &mut Self;
}

pub trait EntitlementScopes {
    fn with_entitlements_api(&mut self) -> &mut Self;
    fn with_entitlements_drops_read(&mut self) -> &mut Self;
    fn with_entitlements_drops_manage(&mut self) -> &mut Self;
}

pub trait ExtensionsScopes {
    fn with_extensions_api(&mut self) -> &mut Self;
    fn with_extension_configuration_segment_read(&mut self) -> &mut Self;
    fn with_extension_configuration_segment_manage(&mut self) -> &mut Self;
    fn with_extion_required_configuration_manage(&mut self) -> &mut Self;
    fn with_extionsion_pubsub_message_write(&mut self) -> &mut Self;
    fn with_extension_live_channel_read(&mut self) -> &mut Self;
    fn with_extension_secrets_read(&mut self) -> &mut Self;
    fn with_extension_secret_create(&mut self) -> &mut Self;
    fn with_extension_chat_message_write(&mut self) -> &mut Self;
    fn with_extensions_read(&mut self) -> &mut Self;
    fn with_released_extensions_read(&mut self) -> &mut Self;
    fn with_extension_bits_products_read(&mut self) -> &mut Self;
    fn with_extension_bits_products_manage(&mut self) -> &mut Self;
}

pub trait EventSubScopes {
    fn with_eventsub_api(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#create-eventsub-subscription>
    fn with_eventsub_create(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#delete-eventsub-subscription>
    fn with_eventsub_delete(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-eventsub-subscriptions>
    fn with_eventsub_read(&mut self) -> &mut Self;
}

pub trait GamesScopes {
    fn with_games_api(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-top-games>
    fn with_top_games_read(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-games>
    fn with_games_read(&mut self) -> &mut Self;
}

pub trait GoalsScopes {
    fn with_golas_api(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-creator-goals>
    fn with_creator_goals_read(&mut self) -> &mut Self;
}

pub trait GuestStarScopes {
    fn with_guest_star_api(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-channel-guest-star-settings>
    fn with_channel_guest_star_setings_read(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#update-channel-guest-star-settings>
    fn with_channel_guest_star_setings_manage(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-guest-star-session>
    fn with_guest_star_session_read(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#create-guest-star-session>
    fn with_guest_star_session_create(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#end-guest-star-session>
    fn with_guest_star_session_end(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-guest-star-invites>
    fn with_guest_star_invites_read(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#send-guest-star-invite>
    fn with_guest_star_invites_write(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#delete-guest-star-invite>
    fn with_guest_star_invites_delete(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#assign-guest-star-slot>
    fn with_gust_star_slot_assign(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#update-guest-star-slot>
    fn with_gust_star_slot_manage(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#delete-guest-star-slot>
    fn with_gust_star_slot_delete(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#update-guest-star-slot-settings>
    fn with_gust_star_slot_settings_manage(&mut self) -> &mut Self;
}

pub trait HypeTrainScopes {
    fn with_hype_train_api(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-hype-train-events>
    fn with_hype_train_events_read(&mut self) -> &mut Self;
}

pub trait ModerationScopes {
    fn with_moderation_api(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#check-automod-status>
    fn with_automod_status_check(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#manage-held-automod-messages>
    fn with_held_automod_messages_manage(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-automod-settings>
    fn with_automod_settings_read(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#update-automod-settings>
    fn with_automod_settings_manage(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-banned-users>
    fn with_banned_users_read(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#ban-user>
    fn with_ban_user(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#unban-user>
    fn with_unban_user(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-unban-requests>
    fn with_unban_requests_read(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#resolve-unban-requests>
    fn with_unban_requests_resolve(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-blocked-terms>
    fn with_blocked_terms_read(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#add-blocked-term>
    fn with_blocked_terms_add(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#remove-blocked-term>
    fn with_blocked_terms_remove(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#delete-chat-messages>
    fn with_chat_messages_delete(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-moderated-channels>
    fn with_moderated_channels_read(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-moderators>
    fn with_moderators_read_as_user(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-moderators>
    fn with_moderators_read_as_app(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#add-channel-moderator>
    fn with_chanel_moderator_add(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#remove-channel-moderator>
    fn with_channel_moderator_remove(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-vips>
    fn with_vips_read_as_user(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-vips>
    fn with_vips_read_as_app(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#add-channel-vip>
    fn with_channel_vip_add(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#remove-channel-vip>
    fn with_channel_vip_remove(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#update-shield-mode-status>
    fn with_shield_mode_status_manage(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-shield-mode-status>
    fn with_shield_mode_status_read(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#warn-chat-user>
    fn with_chat_user_warn(&mut self) -> &mut Self;
}

pub trait PollsScopes {
    fn with_polls_api(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-polls>
    fn with_polls_read(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#create-poll>
    fn with_polls_create(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#end-poll>
    fn with_polls_end(&mut self) -> &mut Self;
}

pub trait PredictionsScopes {
    fn with_predictions_api(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-predictions>
    fn with_predictions_read(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#create-prediction>
    fn with_predictions_create(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#end-prediction>
    fn with_predictions_end(&mut self) -> &mut Self;
}

pub trait RaidsScopes {
    fn with_raids_api(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#start-a-raid>
    fn with_raid_start(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#cancel-a-raid>
    fn with_raid_cancel(&mut self) -> &mut Self;
}

pub trait ScheduleScopes {
    fn with_schedule_api(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-channel-stream-schedule>
    fn with_channel_stream_schedule_read(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-channel-icalendar>
    fn with_channel_icalendar_read(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#update-channel-stream-schedule>
    fn with_channel_stream_schedule_manage(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#create-channel-stream-schedule-segment>
    fn with_channel_stream_schedule_segment_create(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#update-channel-stream-schedule-segment>
    fn with_channel_stream_schedule_segment_manage(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#delete-channel-stream-schedule-segment>
    fn with_channel_stream_schedule_segment_delete(&mut self) -> &mut Self;
}

pub trait SearchScopes {
    fn with_search_api(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#search-categories>
    fn with_categories_search(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#search-channels>
    fn with_channels_search(&mut self) -> &mut Self;
}

pub trait StreamsScopes {
    fn with_streams_api(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-stream-key>
    fn with_stream_key_read(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-streams>
    fn with_streams_read(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-followed-streams>
    fn with_followed_streams_read(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#create-stream-marker>
    fn with_stream_marker_create(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-stream-markers>
    fn with_stream_markers_read(&mut self) -> &mut Self;
}

pub trait SubscriptionsScopes {
    fn with_subscriptions_api(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-all-stream-tags>
    fn with_broadcaster_subscriptions_read(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#check-user-subscription>
    fn with_user_subscription_check(&mut self) -> &mut Self;
}

pub trait TagsScopes {
    fn with_tags_api(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-all-stream-tags>
    fn with_all_stream_tags_read(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-stream-tags>
    fn with_stream_tags_read(&mut self) -> &mut Self;
}

pub trait TeamsScopes {
    fn with_teams_api(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-channel-teams>
    fn with_channel_teams_read(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-teams>
    fn with_teams_read(&mut self) -> &mut Self;
}

pub trait UsersScopes {
    // Users
    fn with_user_api(&mut self) -> &mut Self;
    /// user:read:email
    /// <https://dev.twitch.tv/docs/api/reference/#get-users>
    fn with_users_read(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#update-user>
    fn with_users_manage(&mut self) -> &mut Self;
    /// user:read:blocked_users
    /// <https://dev.twitch.tv/docs/api/reference/#get-user-block-list>
    fn with_block_list_read(&mut self) -> &mut Self;
    /// user:manage:blocked_users
    /// <https://dev.twitch.tv/docs/api/reference/#block-user>
    /// <https://dev.twitch.tv/docs/api/reference/#unblock-user>
    fn with_block_list_manage(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-user-active-extensions>
    fn with_user_extensions_read(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#update-user-extensions>
    fn with_user_extensions_manage(&mut self) -> &mut Self;
}

pub trait VideosScopes {
    fn with_videos_api(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#get-videos>
    fn with_videos_read(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#delete-videos>
    fn with_videos_delete(&mut self) -> &mut Self;
}

pub trait WhispersScopes {
    fn with_whispers_api(&mut self) -> &mut Self;
    /// <https://dev.twitch.tv/docs/api/reference/#send-whisper>
    fn with_whisper_write(&mut self) -> &mut Self;
}

pub trait IRCScopes {
    /// Add all IRC scopes (chat:edit, chat:read)
    fn with_irc_all(&mut self) -> &mut Self;
    /// Add IRC chat edit scope
    fn with_irc_edit(&mut self) -> &mut Self;
    /// Add IRC chat read scope
    fn with_irc_read(&mut self) -> &mut Self;
}

impl AdsScopes for ScopesMut<'_> {
    fn with_ads_api(&mut self) -> &mut Self {
        self.with_stard_commercial()
            .with_ad_schedule_read()
            .with_snooze_next_ad()
    }
    fn with_stard_commercial(&mut self) -> &mut Self {
        self.push(Scope::ChannelEditCommercial);
        self
    }
    fn with_ad_schedule_read(&mut self) -> &mut Self {
        self.push(Scope::ChannelReadAds);
        self
    }
    fn with_snooze_next_ad(&mut self) -> &mut Self {
        self.push(Scope::ChannelManageAds);
        self
    }
}

impl AnalyticsScopes for ScopesMut<'_> {
    fn with_analytics_api(&mut self) -> &mut Self {
        self.with_extension_analytics_read()
            .with_game_analytics_read()
    }
    fn with_extension_analytics_read(&mut self) -> &mut Self {
        self.push(Scope::AnalyticsReadExtensions);
        self
    }
    fn with_game_analytics_read(&mut self) -> &mut Self {
        self.push(Scope::AnalyticsReadGames);
        self
    }
}

impl BitsScopes for ScopesMut<'_> {
    fn with_bits_api(&mut self) -> &mut Self {
        self.with_bits_leaderboard_read()
            .with_cheermotes_read()
            .with_extension_transactions_read()
    }
    fn with_bits_leaderboard_read(&mut self) -> &mut Self {
        self.push(Scope::BitsRead);
        self
    }
    fn with_cheermotes_read(&mut self) -> &mut Self {
        self
    }
    fn with_extension_transactions_read(&mut self) -> &mut Self {
        self
    }
}

impl ChannelScopes for ScopesMut<'_> {
    fn with_channel_api(&mut self) -> &mut Self {
        self.with_channel_info_read()
            .with_channel_info_manage()
            .with_channel_editors_read()
            .with_followed_channels_read()
            .with_channel_followers_read()
    }
    fn with_channel_info_read(&mut self) -> &mut Self {
        self
    }
    fn with_channel_info_manage(&mut self) -> &mut Self {
        self.push(Scope::ChannelManageBroadcast);
        self
    }
    fn with_channel_editors_read(&mut self) -> &mut Self {
        self.push(Scope::ChannelReadEditors);
        self
    }
    fn with_followed_channels_read(&mut self) -> &mut Self {
        self.push(Scope::UserReadFollows);
        self
    }
    fn with_channel_followers_read(&mut self) -> &mut Self {
        self.push(Scope::ModeratorReadFollowers);
        self
    }
    fn with_channel_ban_unban(&mut self) -> &mut Self {
        self.push(Scope::ChannelModerate);
        self
    }
}

impl ChannelPointsScopes for ScopesMut<'_> {
    fn with_channel_points_api(&mut self) -> &mut Self {
        self.with_custom_rewards_create()
            .with_custom_reward_delete()
            .with_custom_reward_read()
            .with_custom_reward_redemption()
            .with_custom_reward_manage()
            .with_redemption_status_manage()
    }
    fn with_custom_rewards_create(&mut self) -> &mut Self {
        self.push(Scope::ChannelManageRedemptions);
        self
    }
    fn with_custom_reward_delete(&mut self) -> &mut Self {
        self.push(Scope::ChannelManageRedemptions);
        self
    }
    fn with_custom_reward_read(&mut self) -> &mut Self {
        self.push(Scope::ChannelReadRedemptions);
        self
    }
    fn with_custom_reward_redemption(&mut self) -> &mut Self {
        self.push(Scope::ChannelReadRedemptions);
        self
    }
    fn with_custom_reward_manage(&mut self) -> &mut Self {
        self.push(Scope::ChannelManageRedemptions);
        self
    }
    fn with_redemption_status_manage(&mut self) -> &mut Self {
        self.push(Scope::ChannelManageRedemptions);
        self
    }
}

impl CharityScopes for ScopesMut<'_> {
    fn with_charity_api(&mut self) -> &mut Self {
        self.with_charity_campaign_read()
            .with_charity_campaign_notations_read()
    }
    fn with_charity_campaign_read(&mut self) -> &mut Self {
        self.push(Scope::ChannelReadCharity);
        self
    }
    fn with_charity_campaign_notations_read(&mut self) -> &mut Self {
        self.push(Scope::ChannelReadCharity);
        self
    }
}

impl ChatScopes for ScopesMut<'_> {
    fn with_chat_api(&mut self) -> &mut Self {
        self.with_chatters()
            .with_channel_emotes()
            .with_global_emotes()
            .with_channel_badges()
            .with_global_badges()
            .with_chat_setting_read()
            .with_shard_chat_session_read()
            .with_user_emotes_read()
            .with_chat_setting_manage()
            .with_chat_announcement_write()
            .with_shoutout_write()
            .with_chat_write()
            .with_chat_write_as_app()
            .with_user_color_read()
            .with_user_color_manage()
    }
    fn with_chatters(&mut self) -> &mut Self {
        self.push(Scope::ModeratorReadChatters);
        self
    }
    fn with_channel_emotes(&mut self) -> &mut Self {
        self
    }
    fn with_global_emotes(&mut self) -> &mut Self {
        self
    }
    fn with_channel_badges(&mut self) -> &mut Self {
        self
    }
    fn with_global_badges(&mut self) -> &mut Self {
        self
    }
    fn with_chat_setting_read(&mut self) -> &mut Self {
        self
    }
    fn with_shard_chat_session_read(&mut self) -> &mut Self {
        self
    }
    fn with_user_emotes_read(&mut self) -> &mut Self {
        self.push(Scope::UserReadEmotes);
        self
    }
    fn with_chat_setting_manage(&mut self) -> &mut Self {
        self.push(Scope::ModeratorManageChatSettings);
        self
    }
    fn with_chat_announcement_write(&mut self) -> &mut Self {
        self.push(Scope::ModeratorManageAnnouncements);
        self
    }
    fn with_shoutout_write(&mut self) -> &mut Self {
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
    fn with_user_color_read(&mut self) -> &mut Self {
        self
    }
    fn with_user_color_manage(&mut self) -> &mut Self {
        self.push(Scope::UserManageChatColor);
        self
    }
}

impl ClipsScopes for ScopesMut<'_> {
    fn with_clips_api(&mut self) -> &mut Self {
        self.with_clip_create().with_clip_read()
    }
    fn with_clip_create(&mut self) -> &mut Self {
        self.push(Scope::ClipsEdit);
        self
    }
    fn with_clip_read(&mut self) -> &mut Self {
        self
    }
}

impl ConduitsScopes for ScopesMut<'_> {
    fn with_conduits_api(&mut self) -> &mut Self {
        self.with_conduits_read()
            .with_conduits_create()
            .with_conduits_manage()
            .with_conduit_delete()
            .with_conduit_shards_read()
    }
    fn with_conduits_read(&mut self) -> &mut Self {
        self
    }
    fn with_conduits_create(&mut self) -> &mut Self {
        self
    }
    fn with_conduits_manage(&mut self) -> &mut Self {
        self
    }
    fn with_conduit_delete(&mut self) -> &mut Self {
        self
    }
    fn with_conduit_shards_read(&mut self) -> &mut Self {
        self
    }
}

impl CCLsScopes for ScopesMut<'_> {
    fn with_cclrs_api(&mut self) -> &mut Self {
        self.with_content_classification_labels_read()
    }
    fn with_content_classification_labels_read(&mut self) -> &mut Self {
        self
    }
}

impl EntitlementScopes for ScopesMut<'_> {
    fn with_entitlements_api(&mut self) -> &mut Self {
        self.with_entitlements_drops_read()
            .with_entitlements_drops_manage()
    }
    fn with_entitlements_drops_read(&mut self) -> &mut Self {
        self
    }
    fn with_entitlements_drops_manage(&mut self) -> &mut Self {
        self
    }
}

impl ExtensionsScopes for ScopesMut<'_> {
    fn with_extensions_api(&mut self) -> &mut Self {
        self.with_extension_configuration_segment_read()
            .with_extension_configuration_segment_manage()
            .with_extion_required_configuration_manage()
            .with_extionsion_pubsub_message_write()
            .with_extension_live_channel_read()
            .with_extension_secrets_read()
            .with_extension_secret_create()
            .with_extension_chat_message_write()
            .with_extensions_read()
            .with_released_extensions_read()
            .with_extension_bits_products_read()
            .with_extension_bits_products_manage()
    }
    fn with_extension_configuration_segment_read(&mut self) -> &mut Self {
        self
    }
    fn with_extension_configuration_segment_manage(&mut self) -> &mut Self {
        self
    }
    fn with_extion_required_configuration_manage(&mut self) -> &mut Self {
        self
    }
    fn with_extionsion_pubsub_message_write(&mut self) -> &mut Self {
        self
    }
    fn with_extension_live_channel_read(&mut self) -> &mut Self {
        self
    }
    fn with_extension_secrets_read(&mut self) -> &mut Self {
        self
    }
    fn with_extension_secret_create(&mut self) -> &mut Self {
        self
    }
    fn with_extension_chat_message_write(&mut self) -> &mut Self {
        self
    }
    fn with_extensions_read(&mut self) -> &mut Self {
        self
    }
    fn with_released_extensions_read(&mut self) -> &mut Self {
        self
    }
    fn with_extension_bits_products_read(&mut self) -> &mut Self {
        self
    }
    fn with_extension_bits_products_manage(&mut self) -> &mut Self {
        self
    }
}

impl EventSubScopes for ScopesMut<'_> {
    fn with_eventsub_api(&mut self) -> &mut Self {
        self.with_eventsub_create()
            .with_eventsub_delete()
            .with_eventsub_read()
    }
    fn with_eventsub_create(&mut self) -> &mut Self {
        self
    }
    fn with_eventsub_delete(&mut self) -> &mut Self {
        self
    }
    fn with_eventsub_read(&mut self) -> &mut Self {
        self
    }
}

impl GamesScopes for ScopesMut<'_> {
    fn with_games_api(&mut self) -> &mut Self {
        self.with_top_games_read().with_games_read()
    }
    fn with_top_games_read(&mut self) -> &mut Self {
        self
    }
    fn with_games_read(&mut self) -> &mut Self {
        self
    }
}

impl GoalsScopes for ScopesMut<'_> {
    fn with_golas_api(&mut self) -> &mut Self {
        self.with_creator_goals_read()
    }
    fn with_creator_goals_read(&mut self) -> &mut Self {
        self.push(Scope::ChannelReadGoals);
        self
    }
}

impl GuestStarScopes for ScopesMut<'_> {
    fn with_guest_star_api(&mut self) -> &mut Self {
        self.with_channel_guest_star_setings_read()
            .with_channel_guest_star_setings_manage()
            .with_guest_star_session_read()
            .with_guest_star_session_create()
            .with_guest_star_session_end()
            .with_guest_star_invites_read()
            .with_guest_star_invites_write()
            .with_guest_star_invites_delete()
            .with_gust_star_slot_assign()
            .with_gust_star_slot_manage()
            .with_gust_star_slot_delete()
            .with_gust_star_slot_settings_manage()
    }
    fn with_channel_guest_star_setings_read(&mut self) -> &mut Self {
        self.extend([
            Scope::ChannelReadGuestStar,
            Scope::ChannelManageGuestStar,
            Scope::ModeratorReadGuestStar,
        ]);
        self
    }
    fn with_channel_guest_star_setings_manage(&mut self) -> &mut Self {
        self.push(Scope::ChannelManageGuestStar);
        self
    }
    fn with_guest_star_session_read(&mut self) -> &mut Self {
        self.extend([
            Scope::ChannelReadGuestStar,
            Scope::ChannelManageGuestStar,
            Scope::ModeratorReadGuestStar,
        ]);
        self
    }
    fn with_guest_star_session_create(&mut self) -> &mut Self {
        self.push(Scope::ChannelManageGuestStar);
        self
    }
    fn with_guest_star_session_end(&mut self) -> &mut Self {
        self.push(Scope::ChannelManageGuestStar);
        self
    }
    fn with_guest_star_invites_read(&mut self) -> &mut Self {
        self.extend([
            Scope::ChannelReadGuestStar,
            Scope::ChannelManageGuestStar,
            Scope::ModeratorReadGuestStar,
        ]);
        self
    }
    fn with_guest_star_invites_write(&mut self) -> &mut Self {
        self.push(Scope::ChannelManageGuestStar);
        self
    }
    fn with_guest_star_invites_delete(&mut self) -> &mut Self {
        self.push(Scope::ChannelManageGuestStar);
        self
    }
    fn with_gust_star_slot_assign(&mut self) -> &mut Self {
        self.push(Scope::ChannelManageGuestStar);
        self
    }
    fn with_gust_star_slot_manage(&mut self) -> &mut Self {
        self.push(Scope::ChannelManageGuestStar);
        self
    }
    fn with_gust_star_slot_delete(&mut self) -> &mut Self {
        self.push(Scope::ChannelManageGuestStar);
        self
    }
    fn with_gust_star_slot_settings_manage(&mut self) -> &mut Self {
        self.push(Scope::ChannelManageGuestStar);
        self
    }
}

impl HypeTrainScopes for ScopesMut<'_> {
    fn with_hype_train_api(&mut self) -> &mut Self {
        self.with_hype_train_events_read()
    }
    fn with_hype_train_events_read(&mut self) -> &mut Self {
        self.push(Scope::ChannelReadHypeTrain);
        self
    }
}

impl ModerationScopes for ScopesMut<'_> {
    fn with_moderation_api(&mut self) -> &mut Self {
        self.with_automod_status_check()
            .with_held_automod_messages_manage()
            .with_automod_settings_read()
            .with_automod_settings_manage()
            .with_banned_users_read()
            .with_ban_user()
            .with_unban_user()
            .with_unban_requests_read()
            .with_unban_requests_resolve()
            .with_blocked_terms_read()
            .with_blocked_terms_add()
            .with_blocked_terms_remove()
            .with_chat_messages_delete()
            .with_moderated_channels_read()
            .with_moderators_read_as_user()
            .with_moderators_read_as_app()
            .with_chanel_moderator_add()
            .with_channel_moderator_remove()
            .with_vips_read_as_user()
            .with_vips_read_as_app()
            .with_channel_vip_add()
            .with_channel_vip_remove()
            .with_shield_mode_status_manage()
            .with_shield_mode_status_read()
            .with_chat_user_warn()
    }
    fn with_automod_status_check(&mut self) -> &mut Self {
        self.push(Scope::ModerationRead);
        self
    }
    fn with_held_automod_messages_manage(&mut self) -> &mut Self {
        self.push(Scope::ModeratorManageAutomod);
        self
    }
    fn with_automod_settings_read(&mut self) -> &mut Self {
        self.push(Scope::ModeratorReadAutomodSettings);
        self
    }
    fn with_automod_settings_manage(&mut self) -> &mut Self {
        self.push(Scope::ModeratorManageAutomodSettings);
        self
    }
    fn with_banned_users_read(&mut self) -> &mut Self {
        self.push(Scope::ModerationRead);
        self
    }
    fn with_ban_user(&mut self) -> &mut Self {
        self.push(Scope::ModeratorManageBannedUsers);
        self
    }
    fn with_unban_user(&mut self) -> &mut Self {
        self.push(Scope::ModeratorManageBannedUsers);
        self
    }
    fn with_unban_requests_read(&mut self) -> &mut Self {
        self.push(Scope::ModeratorReadUnbanRequests);
        self
    }
    fn with_unban_requests_resolve(&mut self) -> &mut Self {
        self.push(Scope::ModeratorReadUnbanRequests);
        self
    }
    fn with_blocked_terms_read(&mut self) -> &mut Self {
        self.push(Scope::ModeratorReadBlockedTerms);
        self
    }
    fn with_blocked_terms_add(&mut self) -> &mut Self {
        self.push(Scope::ModeratorManageBlockedTerms);
        self
    }
    fn with_blocked_terms_remove(&mut self) -> &mut Self {
        self.push(Scope::ModeratorManageBlockedTerms);
        self
    }
    fn with_chat_messages_delete(&mut self) -> &mut Self {
        self.push(Scope::ModeratorManageChatMessages);
        self
    }
    fn with_moderated_channels_read(&mut self) -> &mut Self {
        self.push(Scope::UserReadModeratedChannels);
        self
    }
    fn with_moderators_read_as_user(&mut self) -> &mut Self {
        self.push(Scope::ModerationRead);
        self
    }
    fn with_moderators_read_as_app(&mut self) -> &mut Self {
        self.push(Scope::ChannelManageModerators);
        self
    }
    fn with_chanel_moderator_add(&mut self) -> &mut Self {
        self.push(Scope::ChannelManageModerators);
        self
    }
    fn with_channel_moderator_remove(&mut self) -> &mut Self {
        self.push(Scope::ChannelManageModerators);
        self
    }
    fn with_vips_read_as_user(&mut self) -> &mut Self {
        self.push(Scope::ChannelReadVips);
        self
    }
    fn with_vips_read_as_app(&mut self) -> &mut Self {
        self.push(Scope::ChannelManageVips);
        self
    }
    fn with_channel_vip_add(&mut self) -> &mut Self {
        self.push(Scope::ChannelManageVips);
        self
    }
    fn with_channel_vip_remove(&mut self) -> &mut Self {
        self.push(Scope::ChannelManageVips);
        self
    }
    fn with_shield_mode_status_manage(&mut self) -> &mut Self {
        self.push(Scope::ModeratorManageShieldMode);
        self
    }
    fn with_shield_mode_status_read(&mut self) -> &mut Self {
        self.push(Scope::ModeratorReadShieldMode);
        self
    }
    fn with_chat_user_warn(&mut self) -> &mut Self {
        self.push(Scope::ModeratorManageWarnings);
        self
    }
}

impl PollsScopes for ScopesMut<'_> {
    fn with_polls_api(&mut self) -> &mut Self {
        self.with_polls_read().with_polls_create().with_polls_end()
    }
    fn with_polls_read(&mut self) -> &mut Self {
        self.push(Scope::ChannelReadPolls);
        self
    }
    fn with_polls_create(&mut self) -> &mut Self {
        self.push(Scope::ChannelManagePolls);
        self
    }
    fn with_polls_end(&mut self) -> &mut Self {
        self.push(Scope::ChannelManagePolls);
        self
    }
}

impl PredictionsScopes for ScopesMut<'_> {
    fn with_predictions_api(&mut self) -> &mut Self {
        self.with_predictions_read()
            .with_predictions_create()
            .with_predictions_end()
    }
    fn with_predictions_read(&mut self) -> &mut Self {
        self.push(Scope::ChannelReadPredictions);
        self
    }
    fn with_predictions_create(&mut self) -> &mut Self {
        self.push(Scope::ChannelManagePredictions);
        self
    }
    fn with_predictions_end(&mut self) -> &mut Self {
        self.push(Scope::ChannelManagePredictions);
        self
    }
}

impl RaidsScopes for ScopesMut<'_> {
    fn with_raids_api(&mut self) -> &mut Self {
        self.with_raid_start().with_raid_cancel()
    }
    fn with_raid_start(&mut self) -> &mut Self {
        self.push(Scope::ChannelManageRaids);
        self
    }
    fn with_raid_cancel(&mut self) -> &mut Self {
        self.push(Scope::ChannelManageRaids);
        self
    }
}

impl ScheduleScopes for ScopesMut<'_> {
    fn with_schedule_api(&mut self) -> &mut Self {
        self.with_channel_stream_schedule_read()
            .with_channel_icalendar_read()
            .with_channel_stream_schedule_manage()
            .with_channel_stream_schedule_segment_create()
            .with_channel_stream_schedule_segment_manage()
            .with_channel_stream_schedule_segment_delete()
    }
    fn with_channel_stream_schedule_read(&mut self) -> &mut Self {
        self
    }
    fn with_channel_icalendar_read(&mut self) -> &mut Self {
        self
    }
    fn with_channel_stream_schedule_manage(&mut self) -> &mut Self {
        self.push(Scope::ChannelManageSchedule);
        self
    }
    fn with_channel_stream_schedule_segment_create(&mut self) -> &mut Self {
        self.push(Scope::ChannelManageSchedule);
        self
    }
    fn with_channel_stream_schedule_segment_manage(&mut self) -> &mut Self {
        self.push(Scope::ChannelManageSchedule);
        self
    }
    fn with_channel_stream_schedule_segment_delete(&mut self) -> &mut Self {
        self.push(Scope::ChannelManageSchedule);
        self
    }
}

impl SearchScopes for ScopesMut<'_> {
    fn with_search_api(&mut self) -> &mut Self {
        self.with_categories_search().with_channels_search()
    }
    fn with_categories_search(&mut self) -> &mut Self {
        self
    }
    fn with_channels_search(&mut self) -> &mut Self {
        self
    }
}

impl StreamsScopes for ScopesMut<'_> {
    fn with_streams_api(&mut self) -> &mut Self {
        self.with_stream_key_read()
            .with_streams_read()
            .with_followed_streams_read()
            .with_stream_marker_create()
            .with_stream_markers_read()
    }
    fn with_stream_key_read(&mut self) -> &mut Self {
        self.push(Scope::ChannelReadStreamKey);
        self
    }
    fn with_streams_read(&mut self) -> &mut Self {
        self
    }
    fn with_followed_streams_read(&mut self) -> &mut Self {
        self.push(Scope::UserReadFollows);
        self
    }
    fn with_stream_marker_create(&mut self) -> &mut Self {
        self.push(Scope::ChannelManageBroadcast);
        self
    }
    fn with_stream_markers_read(&mut self) -> &mut Self {
        self.push(Scope::UserReadBroadcast);
        self
    }
}

impl SubscriptionsScopes for ScopesMut<'_> {
    fn with_subscriptions_api(&mut self) -> &mut Self {
        self.with_broadcaster_subscriptions_read()
            .with_user_subscription_check()
    }
    fn with_broadcaster_subscriptions_read(&mut self) -> &mut Self {
        self.push(Scope::ChannelReadSubscriptions);
        self
    }
    fn with_user_subscription_check(&mut self) -> &mut Self {
        self.push(Scope::UserReadSubscriptions);
        self
    }
}

impl TagsScopes for ScopesMut<'_> {
    fn with_tags_api(&mut self) -> &mut Self {
        self.with_all_stream_tags_read().with_stream_tags_read()
    }
    fn with_all_stream_tags_read(&mut self) -> &mut Self {
        self
    }
    fn with_stream_tags_read(&mut self) -> &mut Self {
        self
    }
}

impl TeamsScopes for ScopesMut<'_> {
    fn with_teams_api(&mut self) -> &mut Self {
        self.with_channel_teams_read().with_teams_read()
    }
    fn with_channel_teams_read(&mut self) -> &mut Self {
        self
    }
    fn with_teams_read(&mut self) -> &mut Self {
        self
    }
}
impl UsersScopes for ScopesMut<'_> {
    fn with_user_api(&mut self) -> &mut Self {
        self.with_users_read()
            .with_users_manage()
            .with_block_list_read()
            .with_block_list_manage()
            .with_user_extensions_read()
            .with_user_extensions_manage()
    }
    fn with_users_read(&mut self) -> &mut Self {
        self.push(Scope::UserReadEmail);
        self
    }
    fn with_users_manage(&mut self) -> &mut Self {
        self.extend([Scope::UserReadEmail, Scope::UserEdit]);
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
    fn with_user_extensions_read(&mut self) -> &mut Self {
        self.extend([Scope::UserReadBroadcast, Scope::UserEditBroadcast]);
        self
    }
    fn with_user_extensions_manage(&mut self) -> &mut Self {
        self.push(Scope::UserEditBroadcast);
        self
    }
}

impl VideosScopes for ScopesMut<'_> {
    fn with_videos_api(&mut self) -> &mut Self {
        self.with_videos_read().with_videos_delete()
    }
    fn with_videos_read(&mut self) -> &mut Self {
        self
    }
    fn with_videos_delete(&mut self) -> &mut Self {
        self.push(Scope::ChannelManageVideos);
        self
    }
}

impl WhispersScopes for ScopesMut<'_> {
    fn with_whispers_api(&mut self) -> &mut Self {
        self.with_whisper_write()
    }
    fn with_whisper_write(&mut self) -> &mut Self {
        self.push(Scope::UserManageWhispers);
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
