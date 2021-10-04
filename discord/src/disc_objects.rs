use crate::bot;
use crate::discord;
use serde::{Deserialize, Serialize};
use std::fmt::Formatter;

#[derive(Deserialize, Debug, Serialize, Clone)]
#[serde(untagged)]
pub enum Snowflake {
    Integer(u64),
    String(String),
}

impl std::fmt::Display for Snowflake {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let write_string = match &self {
            &Snowflake::Integer(integer_value) => integer_value.to_string(),
            &Snowflake::String(string_value) => string_value.clone(),
        };

        write!(f, "{}", write_string)
    }
}

#[derive(Deserialize, Debug, Serialize, Clone)]
#[serde(untagged)]
pub enum Nonce {
    Integer(u64),
    String(String),
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct ThreadMember {
    pub id: Option<Snowflake>,
    pub user_id: Option<Snowflake>,
    pub join_timestamp: String,
    pub flags: u64,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct PermissionOverwrite {
    pub id: Snowflake,

    #[serde(rename = "type")]
    pub permission_overwrite_type: u64,

    pub allow: String,
    pub deny: String,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct ThreadMetadata {
    pub archived: bool,
    pub auto_archive_duration: u64,
    pub archive_timestamp: String,
    pub locked: bool,
    pub invitable: Option<bool>,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct Channel {
    pub id: Snowflake,

    #[serde(rename = "type")]
    pub channel_type: Option<u64>,
    pub guild_id: Option<Snowflake>,
    pub position: Option<u64>,
    pub permission_overwrite: Option<Vec<PermissionOverwrite>>,
    pub name: Option<String>,
    pub topic: Option<String>,
    pub nsfw: Option<bool>,
    pub last_message_id: Option<Snowflake>,
    pub bitrate: Option<u64>,
    pub user_limit: Option<u64>,
    pub rate_limit_per_user: Option<u64>,
    pub recipients: Option<Vec<User>>,
    pub icon: Option<String>,
    pub owner_id: Option<Snowflake>,
    pub application_id: Option<Snowflake>,
    pub parent_id: Option<Snowflake>,
    pub last_pin_timestamp: Option<String>,
    pub rtc_region: Option<String>,
    pub video_quality_mode: Option<u64>,
    pub message_count: Option<u64>,
    pub member_count: Option<u64>,
    pub thread_metadata: Option<ThreadMetadata>,
    pub member: Option<ThreadMember>,
    pub default_auto_archive_duration: Option<u64>,
    pub permissions: Option<String>,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct User {
    pub id: Snowflake,
    pub username: String,
    pub discriminator: String,
    pub avatar: Option<String>,
    pub bot: Option<bool>,
    pub system: Option<bool>,
    pub mfa_enabled: Option<bool>,
    pub banner: Option<String>,
    pub accent_colour: Option<u64>,
    pub locale: Option<String>,
    pub verified: Option<bool>,
    pub email: Option<String>,
    pub flags: Option<u64>,
    pub premium_type: Option<u64>,
    pub public_flags: Option<u64>,

    pub member: Option<Box<GuildMember>>,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct GuildMember {
    pub user: Option<User>,
    pub nick: Option<String>,
    pub avatar: Option<String>,
    pub roles: Vec<String>,
    pub joined_at: String,
    pub premium_since: Option<String>,
    pub deaf: bool,
    pub mute: bool,
    pub pending: Option<bool>,
    pub permissions: Option<String>,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct Attachment {
    pub id: Snowflake,
    pub filename: String,
    pub content_type: Option<String>,
    pub size: u64,
    pub url: String,
    pub proxy_url: String,
    pub height: Option<u64>,
    pub width: Option<u64>,
    pub ephemeral: Option<bool>,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct EmbedFooter {
    pub text: String,
    pub icon_url: Option<String>,
    pub proxy_icon_url: Option<bool>,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct EmbedAttachment {
    pub url: String,
    pub proxy_url: Option<String>,
    pub height: Option<u64>,
    pub width: Option<u64>,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct EmbedProvider {
    pub name: Option<String>,
    pub url: Option<String>,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct EmbedAuthor {
    pub name: String,
    pub proxy_icon_url: Option<String>,
    pub url: Option<String>,
    pub icon_url: Option<String>,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct EmbedField {
    pub name: String,
    pub value: String,
    pub inline: Option<bool>,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct Embed {
    pub title: Option<String>,

    #[serde(rename = "type")]
    pub embed_type: Option<String>,

    pub description: Option<String>,
    pub url: Option<String>,
    pub timestamp: Option<String>,

    #[serde(rename = "color")]
    pub colour: Option<u64>, //COLOUR!!!! (not color)

    pub footer: Option<EmbedFooter>,
    pub image: Option<EmbedAttachment>,
    pub thumbnail: Option<EmbedAttachment>,
    pub video: Option<EmbedAttachment>,
    pub provider: Option<EmbedProvider>,
    pub author: Option<EmbedAuthor>,
    pub fields: Option<Vec<EmbedField>>,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct Emoji {
    pub id: Option<Snowflake>,
    pub name: Option<String>,
    pub roles: Option<Vec<String>>,
    pub user: Option<User>,
    pub require_colons: Option<bool>,
    pub managed: Option<bool>,
    pub animated: Option<bool>,
    pub available: Option<bool>,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct Reaction {
    pub count: u64,
    pub me: bool,
    pub emoji: Emoji,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct MessageActivity {
    #[serde(rename = "type")]
    pub message_activity_type: u64,
    pub party_id: Option<String>,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct ChannelMention {
    pub id: Snowflake,
    pub guild_id: Snowflake,

    #[serde(rename = "type")]
    pub channel_type: u64,
    pub name: String,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct TeamMember {
    pub membership_state: u64,
    pub permissions: Vec<String>,
    pub team_id: Snowflake,
    pub user: User,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct Team {
    pub icon: Option<String>,
    pub id: Snowflake,
    pub members: Vec<TeamMember>,
    pub name: String,
    pub owner_user_id: Snowflake,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct SelectOption {
    pub label: String,
    pub value: String,
    pub description: Option<String>,
    pub emoji: Option<Emoji>,
    pub default: Option<bool>,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct Component {
    #[serde(rename = "type")]
    pub component_type: u64,
    pub custom_id: Option<String>,
    pub disabled: Option<bool>,
    pub style: Option<u64>,
    pub label: Option<String>,
    pub emoji: Option<Emoji>,
    pub url: Option<String>,
    pub options: Vec<SelectOption>,
    pub placeholder: Option<String>,
    pub min_values: Option<u64>,
    pub max_values: Option<u64>,
    pub components: Option<Vec<Component>>,
}
#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct Application {
    pub id: Snowflake,
    pub name: String,
    pub icon: Option<String>,
    pub description: String,
    pub rpc_origins: Option<Vec<String>>,
    pub bot_public: bool,
    pub bot_require_code_grant: bool,
    pub terms_of_service_url: Option<String>,
    pub privacy_policy_url: Option<String>,
    pub owner: Option<User>,
    pub summary: String,
    pub verify_key: String,
    pub team: Option<Team>,
    pub guild_id: Option<Snowflake>,
    pub primary_sku_id: Option<Snowflake>,
    pub slug: Option<String>,
    pub cover_image: Option<String>,
    pub flags: u64,
}
#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct MessageInteraction {
    pub id: Snowflake,

    #[serde(rename = "type")]
    pub message_interaction_type: u64,
    pub name: String,
    pub user: User,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct MessageReference {
    pub message_id: Option<Snowflake>,
    pub channel_id: Option<Snowflake>,
    pub guild_id: Option<Snowflake>,
    pub fail_if_not_exists: Option<bool>,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct StickerItem {
    pub id: Snowflake,
    pub name: String,
    pub format_type: u64,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct Message {
    pub id: Snowflake,
    pub channel_id: Snowflake,
    pub guild_id: Option<Snowflake>,
    pub author: User,
    pub member: Option<GuildMember>,
    pub content: String,
    pub timestamp: String,
    pub edited_timestamp: Option<String>,
    pub tts: bool,
    pub mention_everyone: bool,
    pub mentions: Vec<User>,
    pub mention_roles: Vec<Snowflake>,
    pub mention_channels: Option<Vec<ChannelMention>>,
    pub attachments: Vec<Attachment>,
    pub embeds: Vec<Embed>,
    pub reactions: Option<Vec<Reaction>>,
    pub nonce: Option<Nonce>,
    pub pinned: bool,
    pub webhook_id: Option<Snowflake>,

    #[serde(rename = "type")]
    pub message_type: u64,
    pub activity: Option<MessageActivity>,
    pub application: Option<Application>,
    pub application_id: Option<Snowflake>,

    pub message_reference: Option<MessageReference>,
    pub flags: Option<u64>,
    pub reference_message: Option<Box<Message>>,
    pub interaction: Option<MessageInteraction>,
    pub thread: Option<Channel>,
    pub components: Option<Vec<Component>>,
    pub sticker_items: Option<Vec<StickerItem>>,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct RoleTag {
    pub bot_id: Option<Snowflake>,
    pub integration_id: Option<Snowflake>,
    pub premium_subscriber: Option<Option<bool>>,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct Role {
    pub id: Snowflake,
    pub name: String,

    #[serde(rename = "color")]
    pub colour: u64,
    pub hoist: bool,
    pub icon: Option<String>,
    pub unicode_emoji: Option<String>,
    pub position: u64,
    pub permissions: String,
    pub managed: bool,
    pub mentionable: bool,
    pub tags: Option<RoleTag>,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct ActivityTimestamp {
    pub start: Option<u64>,
    pub end: Option<u64>,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct ActivityParty {
    pub id: Option<String>,
    pub size: Option<Vec<u64>>,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct ActivityAsset {
    pub large_image: Option<String>,
    pub large_text: Option<String>,
    pub small_image: Option<String>,
    pub small_text: Option<String>,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct ActivitySecret {
    pub join: Option<String>,
    pub spectate: Option<String>,

    #[serde(rename="match")]
    pub activity_secret_match: Option<String>,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct ActivityEmoji {
    pub name: String,
    pub id: Option<Snowflake>,
    pub animated: Option<bool>,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct ActivityButton {
    pub label: String,
    pub url: String
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct Activity {
    pub name: String,

    #[serde(rename = "type")]
    pub activity_type: u64,

    pub url: Option<String>,
    pub created_at: u64,
    pub timestamps: Option<Vec<ActivityTimestamp>>,
    pub application_id: Option<Snowflake>,
    pub details: Option<String>,
    pub state: Option<String>,
    pub emoji: Option<ActivityEmoji>,
    pub party: Option<ActivityParty>,
    pub assets: Option<ActivityAsset>,
    pub secrets: Option<ActivitySecret>,
    pub instance: Option<bool>,
    pub flags: Option<u64>,
    pub buttons: Option<Vec<ActivityButton>>,

}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct ClientStatus {
    pub desktop: Option<String>,
    pub mobile: Option<String>,
    pub web: Option<String>,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct PresenceUpdate {
    pub user: User,
    pub guild_id: Snowflake,
    pub status: String,
    pub activities: Vec<Activity>,
    pub client_status: ClientStatus,

}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct VoiceState {
    pub guild_id: Option<Snowflake>,
    pub channel_id: Option<Snowflake>,
    pub user_id: Snowflake,
    pub member: Option<GuildMember>,
    pub session_id: String,
    pub deaf: bool,
    pub mute: bool,
    pub self_deaf: bool,
    pub self_mute: bool,
    pub self_stream: Option<bool>,
    pub self_video: bool,
    pub suppress: bool,
    pub request_to_speak_timestamp: Option<String>,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct StageInstance {
    pub id: Snowflake,
    pub guild_id: Snowflake,
    pub channel_id: Snowflake,
    pub topic: String,
    pub privacy_level: u64,
    pub discoverable_disabled: bool,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct WelcomeScreenChannel {
    pub channel_id: Snowflake,
    pub description: String,
    pub emoji_id: Option<Snowflake>,
    pub emoji_name: Option<String>,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct WelcomeScreen {
    pub description: Option<String>,
    pub welcome_channels: Vec<WelcomeScreenChannel>,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct Sticker {
    pub id: Snowflake,
    pub pack_id: Option<Snowflake>,
    pub name: String,
    pub description: Option<String>,
    pub tags: String,
    pub asset: String,

    #[serde(rename="type")]
    pub sticker_type: u64,
    pub format_type: u64,
    pub available: Option<bool>,
    pub guild_id: Option<Snowflake>,
    pub user: Option<User>,
    pub sort_value: Option<u64>,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct Guild {
    pub id: Snowflake,
    pub name: String,
    pub icon: Option<String>,
    pub icon_hash: Option<String>,
    pub splash: Option<String>,
    pub discovery_splash: Option<String>,
    pub owner: Option<bool>,
    pub owner_id: Snowflake,
    pub permissions: Option<String>,
    pub region: Option<String>,
    pub afk_channel_id: Option<Snowflake>,
    pub afk_timeout: u64,
    pub widget_enabled: Option<bool>,
    pub widget_channel_id: Option<Snowflake>,
    pub verification_level: u64,
    pub default_message_notifications: u64,
    pub explicit_content_filter: u64,
    pub roles: Vec<Role>,
    pub emojis: Vec<Emoji>,
    pub features: Vec<String>,
    pub mfa_level: u64,
    pub application_id: Option<Snowflake>,
    pub system_channel_id: Option<Snowflake>,
    pub system_channel_flags: u64,
    pub rules_channel_id: Option<Snowflake>,
    pub joined_at: Option<String>, // Timestamp
    pub large: Option<bool>,
    pub unavailable: Option<bool>,
    pub member_count: Option<u64>,
    pub voice_states: Option<Vec<VoiceState>>,
    pub members: Option<Vec<GuildMember>>,
    pub channels: Option<Vec<Channel>>,
    pub threads: Option<Vec<Channel>>,
    pub presences: Option<Vec<PresenceUpdate>>,
    pub max_presences: Option<u64>,
    pub max_members: Option<u64>,
    pub vanity_url_code: Option<String>,
    pub description: Option<String>,
    pub premium_tier: u64,
    pub premium_subscriber_count: Option<u64>,
    pub preferred_locale: String,
    pub public_updates_channel_id: Option<Snowflake>,
    pub max_video_channel_users: Option<u64>,
    pub approximate_member_count: Option<u64>,
    pub approximate_presence_count: Option<u64>,
    pub welcome_screen: Option<WelcomeScreen>,
    pub nsfw_level: u64,
    pub stage_instances: Option<Vec<StageInstance>>,
    pub stickers: Option<Vec<Sticker>>,

}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct UnavailableGuild {
    pub id: Snowflake,
    pub unavailable: bool,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct ReadyEventApplication {
    // partial application for the ready event,
    pub id : Snowflake,
    pub flags: Option<u64>,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct Hello {
    pub heartbeat_interval: u64,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct ReadyEvent {
    pub v : u64,
    pub user : User,
    pub guilds: Vec<UnavailableGuild>,
    pub session_id: String,
    pub shard: Option<Vec<u64>>,
    pub application: ReadyEventApplication,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct ChannelPinUpdateEvent {
    pub guild_id: Option<Snowflake>,
    pub channel_id: Snowflake,
    pub last_pin_timestamp: Option<String>,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct ReactionAddEvent {
    pub user_id: Option<Snowflake>,
    pub channel_id: Option<Snowflake>,
    pub message_id: Option<Snowflake>,
    pub guild_id: Option<Snowflake>,
    pub member: Option<GuildMember>,
    pub emoji: Option<Emoji>,
}

#[derive(Debug, PartialEq, Eq, Hash, Deserialize, Serialize, Clone)]
#[serde(rename_all(serialize = "SCREAMING_SNAKE_CASE", deserialize = "SCREAMING_SNAKE_CASE"))]
pub enum GatewayEventBinding {
    HeartbeatOk,
    Heartbeat,
    Hello,
    Ready,
    Resumed,
    Reconnect,
    InvalidSession,
    ChannelCreate,
    ChannelUpdate,
    ChannelDelete,
    ChannelPinsUpdate,
    ThreadCreate,
    ThreadUpdate,
    ThreadDelete,
    ThreadListSync,
    ThreadMemberUpdate,
    ThreadMembersUpdate,
    GuildCreate,
    GuildUpdate,
    GuildDelete,
    GuildBanAdd,
    GuildBanRemove,
    GuildEmojisUpdate,
    GuildStickersUpdate,
    GuildIntegrationsUpdate,
    GuildMemberAdd,
    GuildMemberRemove,
    GuildMemberUpdate,
    GuildMembersChunk,
    GuildRoleCreate,
    GuildRoleUpdate,
    GuildRoleDelete,
    IntegrationCreate,
    IntegrationUpdate,
    IntegrationDelete,
    InteractionCreate,
    InviteCreate,
    InviteDelete,
    MessageCreate,
    MessageUpdate,
    MessageDelete,
    MessageDeleteBulk,
    MessageReactionAdd,
    MessageReactionRemove,
    MessageReactionRemoveAll,
    MessageReactionRemoveEmoji,
    PresenceUpdate,
    StageInstanceCreate,
    StageInstanceDelete,
    StageInstanceUpdate,
    TypingStart,
    UserUpdate,
    VoiceStateUpdate,
    VoiceServerUpdate,
    WebhooksUpdate,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct Reply {
    pub message_id: Option<Snowflake>,
    pub channel_id: Option<Snowflake>,
    pub guild_id: Option<Snowflake>,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct ReplyMessage {
    pub content: Option<String>,
    pub tts: bool,
    pub embeds: Option<Vec<Embed>>,
    pub message_reference: Option<Reply>,
    pub sticker_ids: Option<Vec<Snowflake>>,
}

impl Reply {
    fn new(message_id: Option<Snowflake>, channel_id: Option<Snowflake>, guild_id: Option<Snowflake>) -> Self {
        Self {
            message_id,
            channel_id,
            guild_id,
        }
    }
}

impl ReplyMessage {
    pub fn new(tts: bool) -> Self {
        Self {
            content: None,
            tts,
            embeds: None,
            message_reference: None,
            sticker_ids : None,
        }
    }

    pub fn content_str(mut self, content: &str) -> ReplyMessage {
        self.content = Some(String::from(content));
        self
    }

    pub fn content_string(mut self, content: String) -> ReplyMessage {
        self.content = Some(content);
        self
    }

    pub fn add_embed(mut self, embed: Embed) -> ReplyMessage {

        if self.embeds.is_some() {
            let mut current = self.embeds.clone().unwrap();
            current.push(embed);

            self.embeds = Some(current)

        } else {
            self.embeds = Some(vec![embed])
        }

        self

    }

    pub fn reply_message(mut self, message: Message) -> ReplyMessage {

        let reply = Reply::new(Some(message.id), None, None);
        self.message_reference = Some(reply);

        self
    }

    pub async fn send(&self, channel_id: Snowflake, client: bot::BotClient) -> Message {

        let message = serde_json::json!({
            "content": self.content,
            "tts": self.tts,
            "embeds": self.embeds,
            "message_reference": self.message_reference,
            "sticker_ids": self.sticker_ids
        });

        let channel_id = match channel_id {
            Snowflake::String(value) => value,
            Snowflake::Integer(value) => value.to_string()
        };

        let extension = format!("/channels/{}/messages", channel_id);
        let payload = discord::HttpRequest::string_new(extension, client).await;

        let response = payload.post(message).await;
        let response_message : Message;

        response_message = response
            .expect("Failed to send message!")
            .json().await.expect("Failed to turn response into message! ");

        response_message
    }

}

impl Embed {
    pub fn new(title : &str, description: &str, colour : u64) -> Self {
        Self {
            title: Some(String::from(title)),
            embed_type: Some(String::from("rich")),
            description: Some(String::from(description)),
            url: None,
            timestamp: None,
            colour: Some(colour),
            footer: None,
            thumbnail: None,
            image: None,
            video: None,
            provider: None,
            author: None,
            fields: None,
        }
    }
    
    pub fn video(mut self, url: &str) -> Embed {
        self.video = Some(
            EmbedAttachment::new(
                url.to_string()
            )
        );

        self
    }
    
    pub fn image(mut self, url: &str) -> Embed {
        self.image = Some(
            EmbedAttachment::new(
                url.to_string()
            )
        );

        self
    }
    
    pub fn thumbnail(mut self, url: &str) -> Embed{
        self.thumbnail = Some(
            EmbedAttachment::new(
                url.to_string()
            )
        );

        self
    }

    pub fn url(mut self, url: &str) -> Embed {
        self.url = Some(String::from(url));
        self
    }

    pub fn timestamp(mut self, timestamp : &str) -> Embed {
        self.timestamp = Some(timestamp.to_string());
        self
    }

    pub fn change_colour(mut self, new_colour: u64) -> Embed {
        self.colour = Some(new_colour);
        self
    }

    pub fn footer(mut self, text: &str, icon_url: Option<&str>) -> Embed {

        let icon_url : Option<String> = match icon_url {
            Some(value) => Some(value.to_string()),
            None => None
        };

        self.footer = Some(
            EmbedFooter::new(
                text.to_string(),
                icon_url,
            )
        );

        self
    }

    pub fn author(mut self, name: &str, icon_url: Option<&str>, url: Option<&str>) -> Embed {

        let icon_url : Option<String> = match icon_url {
            Some(value) => Some(value.to_string()),
            None => None
        };

        let url : Option<String> = match url {
            Some(value) => Some(value.to_string()),
            None => None
        };

        self.author = Some(
            EmbedAuthor::new(
                name.to_string(),
                url,
                icon_url,
            )
        );

        self
    }

    pub fn add_field(mut self, name: &str, value: &str, inline: bool) -> Embed {

        let field = EmbedField::new(
            name.to_string(),
            value.to_string(),
            inline,
        );

        if self.fields.is_some() {
            let mut current = self.fields.clone().unwrap();

            current.push(field);

            self.fields = Some(current)

        } else {
            self.fields = Some(vec![field])
        }

        self
    }

}

impl EmbedField {
    pub fn new(name: String, value: String, inline: bool) -> Self {
        Self {
            name,
            value,
            inline : Some(inline),
        }
    }
}

impl EmbedAuthor {
    pub fn new(name: String, url: Option<String>, icon_url: Option<String>) -> Self {
        Self {
            name,
            url,
            icon_url,
            proxy_icon_url: None,
        }
    }
}

impl EmbedFooter {
    pub fn new(text: String, icon_url : Option<String>) -> Self {
        Self {
            text,
            icon_url,
            proxy_icon_url : None,
        }
    }
}

impl EmbedAttachment {

    pub fn new(url : String) -> Self {
        Self {
            url,
            proxy_url: None,
            height: None,
            width: None,
        }
    }
}

impl Message {
    pub fn is_bot(&self) -> bool {
        match self.author.bot {
            Some(value) => value,
            None => false,
        }
    }
}