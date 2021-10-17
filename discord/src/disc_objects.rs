use crate::bot;
use crate::discord;
use crate::interactions;
use serde::{Deserialize, Serialize};
use std::fmt::Formatter;
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Deserialize, Debug, Serialize, Clone, Hash, Eq, PartialEq)]
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

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Hash, Eq, PartialEq)]
#[repr(u64)]
pub enum PermissionOverwriteType {
    Role = 0,
    Member = 1,
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
    pub permission_overwrite_type: PermissionOverwriteType,

    pub allow: String,
    pub deny: String,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct ThreadListSyncEvent {
    pub guild_id: Option<Snowflake>,
    pub channel_ids: Option<Vec<Snowflake>>,
    pub threads: Vec<Channel>,
    pub members: Vec<ThreadMember>,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct GuildBanEvent {
    pub guild_id: Option<Snowflake>,
    pub user: User,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct GuildEmojisUpdateEvent {
    pub guild_id: Option<Snowflake>,
    pub emojis: Vec<Emoji>,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct GuildStickersUpdateEvent {
    pub guild_id: Option<Snowflake>,
    pub emojis: Vec<Emoji>,
}
#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct GuildMembersChunkEvent {
    pub guild_id: Option<Snowflake>,
    pub members: Vec<GuildMember>,
    pub chunk_index: u64,
    pub chunk_count: u64,
    pub not_found: Option<Vec<String>>,
    pub presences: Option<Vec<PresenceUpdate>>,
    pub nonce: Nonce,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct GuildRoleEvent {
    pub guild_id: Option<Snowflake>,
    pub role: Role,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct GuildRoleDeleteEvent {
    pub guild_id: Option<Snowflake>,
    pub role_id: Snowflake,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct IntegrationAccount {
    pub id: String,
    pub name: String,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct IntegrationApplication {
    pub id: Snowflake,
    pub name: String,
    pub icon: Option<String>,
    pub description: String,
    pub summary: String,
    pub bot: Option<User>,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct Integration {
    pub id: Snowflake,
    pub name: String,

    #[serde(rename = "type")]
    pub integration_type: String,
    pub enabled: bool,
    pub syncing: Option<bool>,
    pub role_id: Option<Snowflake>,
    pub enable_emoticons: Option<bool>,
    pub expire_behaviour: u64,
    pub expire_grace_period: u64,
    pub user: Option<User>,
    pub account: IntegrationAccount,
    pub synced_at: Option<String>, //timestamp
    pub subscriber_count: Option<u64>,
    pub revoked: Option<bool>,
    pub application: IntegrationApplication,
    pub guild_id: Option<Snowflake> // present in integration create && update event
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct IntegrationDeleteEvent {
    pub id: Snowflake,
    pub guild_id: Option<Snowflake>,
    pub application_id: Option<Snowflake>,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct InviteStageInstance {
    pub members: Vec<GuildMember>,
    pub participant_count: u64,
    pub speaker_count: u64,
    pub topic: String
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct ResolvedData {
    pub users: Option<std::collections::HashMap<Snowflake, User>>,
    pub members: Option<std::collections::HashMap<Snowflake, GuildMember>>,
    pub roles: Option<std::collections::HashMap<Snowflake, Role>>,
    pub channels: Option<std::collections::HashMap<Snowflake, Channel>>,
    pub messages: Option<std::collections::HashMap<Snowflake, Message>>,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Hash, Eq, PartialEq)]
#[repr(u64)]
pub enum ApplicationCommandType {
    ChatInput = 1,
    User = 2,
    Message = 3,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Hash, Eq, PartialEq)]
#[repr(u64)]
pub enum AppCommandOptionType {
    SubCommand = 1,
    SubCommandGroup = 2,
    String = 3,
    Integer = 4,
    Boolean = 5,
    User = 6,
    Channel = 7,
    Role = 8,
    Mentionable = 9,
    Number = 10,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
#[serde(untagged)]
pub enum AppCommandValue {
    String(String),
    Integer(i64),
    Double(f64),
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct AppMessageInteractionDataOption {
    pub name: String,

    // long boi
    #[serde(rename = "type")]
    pub app_message_interaction_data_type: AppCommandOptionType,
    pub value: Option<AppCommandValue>,
    pub options: Option<Vec<AppMessageInteractionDataOption>>,

}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct InteractionData {
    pub id: Snowflake,
    pub name: String,

    #[serde(rename = "type")]
    pub interaction_data_type: ApplicationCommandType,
    pub resolved: Option<ResolvedData>,
    pub options: Option<Vec<AppMessageInteractionDataOption>>,
    pub custom_id: Option<String>,
    pub component_type: Option<u64>,
    pub values: Option<Vec<SelectOption>>,
    pub target_id: Option<Snowflake>,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Hash, Eq, PartialEq)]
#[repr(u64)]
pub enum InteractionType {
    Ping  = 1,
    ApplicationCommand = 2,
    MessageComponent = 3,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct Interaction {
    pub id: Snowflake,
    pub application_id: Snowflake,

    #[serde(rename = "type")]
    pub interaction_type: InteractionType,
    pub data: Option<InteractionData>,
    pub guild_id: Option<Snowflake>,
    pub channel_id: Option<Snowflake>,
    pub member: Option<GuildMember>, //sent in guilds
    pub user: Option<User>, //sent in dms
    pub token: String,
    pub version: u64,
    pub message: Option<Message>
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct InviteCreateEvent {
    pub channel_id: Snowflake,
    pub code: String,
    pub created_at: String, //timestamp
    pub guild_id: Option<Snowflake>,
    pub inviter: Option<User>,
    pub max_age: u64,
    pub max_uses: u64,
    pub target_type: Option<u64>,
    pub target_user: Option<User>,
    pub target_application: Option<Application>,
    pub temporary: bool,
    pub uses: u64
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct InviteDeleteEvent {
    pub channel_id: Snowflake,
    pub guild_id: Option<Snowflake>,
    pub code: String,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct GuildIdEvent {
    pub guild_id: Option<Snowflake>,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct GuildMemberRemoveEvent {
    pub guild_id: Option<Snowflake>,
    pub user: User,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct GuildMemberUpdateEvent {
    pub guild_id: Option<Snowflake>,
    pub roles: Vec<Snowflake>,
    pub user: User,
    pub nick: Option<String>,
    pub avatar: Option<String>,
    pub joined_at: Option<String>,
    pub premium_since: Option<String>,
    pub deaf: Option<bool>,
    pub mute: Option<bool>,
    pub pending: Option<bool>,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct ThreadMembersUpdateEvent {
    pub id: Snowflake,
    pub guild_id: Option<Snowflake>,
    pub member_count: u64,
    pub added_members: Option<Vec<ThreadMember>>,
    pub removed_member_ids: Option<Vec<Snowflake>>,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct ThreadMetadata {
    pub archived: bool,
    pub auto_archive_duration: u64,
    pub archive_timestamp: String,
    pub locked: bool,
    pub invitable: Option<bool>,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Hash, Eq, PartialEq)]
#[repr(u64)]
pub enum ChannelType {
    GuildText = 0,
    Dm = 1,
    GuildVoice = 2,
    GroupDm = 3,
    GuildCategory = 4,
    GuildNews = 5,
    GuildStore= 6,
    GuildNewsThread = 7,
    GuildPublicThread = 8,
    GuildPrivateThread = 9,
    GuildStageVoice = 10
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct Channel {
    pub id: Snowflake,

    #[serde(rename = "type")]
    pub channel_type: Option<ChannelType>,
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

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Hash, Eq, PartialEq)]
#[repr(u64)]
pub enum NitroType {
    None = 0,
    NitroClassic = 1,
    Nitro = 2,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct User {
    pub id: Snowflake,
    pub username:  Option<String>,
    pub discriminator:  Option<String>,
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
    pub premium_type: Option<NitroType>,
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
    pub deaf: Option<bool>,
    pub mute: Option<bool>,
    pub pending: Option<bool>,
    pub permissions: Option<String>,

    pub guild_id: Option<Snowflake>, // Present in guild member add event!
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

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Hash, Eq, PartialEq)]
#[repr(u64)]
pub enum MessageActivityType {
    Join = 1,
    Spectate = 2,
    Listen = 3,
    JoinRequest = 5,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct MessageActivity {
    #[serde(rename = "type")]
    pub message_activity_type: MessageActivityType,
    pub party_id: Option<String>,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct ChannelMention {
    pub id: Snowflake,
    pub guild_id: Option<Snowflake>,

    #[serde(rename = "type")]
    pub channel_type: ChannelType,
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
    pub owner_user_id: Snowflake,
    pub members: Vec<TeamMember>,
    pub name: String,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct SelectOption {
    pub label: String,
    pub value: String,
    pub description: Option<String>,
    pub emoji: Option<Emoji>,
    pub default: Option<bool>,
}

// May be quickly broken as discord adds new component types,
// use https://discord.com/developers/docs/interactions/message-components#component-object-component-types to update
#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Hash, Eq, PartialEq)]
#[repr(u64)]
pub enum ComponentType {
    ActionRow = 1,
    Button = 2,
    SelectMenu = 3,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct Component {
    #[serde(rename = "type")]
    pub component_type: ComponentType,
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
    pub message_interaction_type: InteractionType,
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

// https://discord.com/developers/docs/resources/channel#message-object-message-types
#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Hash, Eq, PartialEq)]
#[repr(u64)]
pub enum MessageType {
    Default = 0,
    RecipientAdd = 1,
    RecipientRemove = 2,
    Call = 3,
    ChannelNameChange = 4,
    ChannelIconChange = 5,
    ChannelPinnedMessage = 6,
    GuildMemberJoin = 7,
    UserPremiumGuildSubscription = 8,
    UserPremiumGuildSubscriptionT1 = 9,
    UserPremiumGuildSubscriptionT2 = 10,
    UserPremiumGuildSubscriptionT3 = 11,
    ChannelFollowAdd = 12,
    GuildDiscoveryDisqualified = 14,
    GuildDiscoveryRequalified = 15,
    GuildDiscoveryGracePeriodInitialWarning = 16,
    GuildDiscoveryGracePeriodFinalWarning = 17,
    ThreadCreated = 18,
    Reply = 19,
    ChatInputCommand = 20,
    ThreadStarterMessage = 21,
    GuildInviteReminder = 22,
    ContextMenuCommand = 23,
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
    pub message_type: MessageType,
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
    pub permissions: u64,
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

    #[serde(rename = "match")]
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
    pub url: String,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Hash, Eq, PartialEq)]
#[repr(u64)]
pub enum ActivityType {
    Game = 0,
    Streaming = 1,
    Listening = 2,
    Watching = 3,
    Custom = 4,
    Competing = 5,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct Activity {
    pub name: String,

    #[serde(rename = "type")]
    pub activity_type: ActivityType,

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
    pub guild_id: Option<Snowflake>,
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
    pub guild_id: Option<Snowflake>,
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


#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Hash, Eq, PartialEq)]
#[repr(u64)]
pub enum StickerType {
    Standard = 1,
    Guild = 2,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Hash, Eq, PartialEq)]
#[repr(u64)]
pub enum StickerFormatType {
    PNG = 1,
    APNG = 2,
    Lottie = 3,
}


#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct Sticker {
    pub id: Snowflake,
    pub pack_id: Option<Snowflake>,
    pub name: String,
    pub description: Option<String>,
    pub tags: String,
    pub asset: String,

    #[serde(rename = "type")]
    pub sticker_type: StickerType,
    pub format_type: StickerFormatType,
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
    pub id: Snowflake,
    pub flags: Option<u64>,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct Hello {
    pub heartbeat_interval: u64,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct ReadyEvent {
    pub v: u64,
    pub user: User,
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
pub struct MessageReactionAddEvent {
    pub user_id: Option<Snowflake>,
    pub channel_id: Option<Snowflake>,
    pub message_id: Option<Snowflake>,
    pub guild_id: Option<Snowflake>,
    pub member: Option<GuildMember>,
    pub emoji: Option<Emoji>,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct MessageDeleteEvent {
    pub id: Snowflake,
    pub channel_id: Snowflake,
    pub guild_id: Option<Snowflake>,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct MessageBulkDeleteEvent {
    pub ids: Vec<Snowflake>,
    pub channel_id: Snowflake,
    pub guild_id: Option<Snowflake>,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct MessageReactionRemoveEvent {
    pub user_id: Option<Snowflake>,
    pub channel_id: Option<Snowflake>,
    pub message_id: Option<Snowflake>,
    pub guild_id: Option<Snowflake>,
    pub emoji: Option<Emoji>,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct MessageRemoveAllReactionEvent {
    pub channel_id: Snowflake,
    pub message_id: Snowflake,
    pub guild_id: Option<Snowflake>,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct MessageReactionRemoveEmojiEvent {
    pub channel_id: Snowflake,
    pub message_id: Option<Snowflake>,
    pub guild_id: Option<Snowflake>,
    pub emoji: Emoji,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct TypingStartEvent {
    pub channel_id: Snowflake,
    pub guild_id: Option<Snowflake>,
    pub user_id: Snowflake,
    pub timestamp: u64,
    pub member: GuildMember
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct VoiceServerUpdateEvent {
    pub token : String,
    pub guild_id: Option<Snowflake>,
    pub endpoint: Option<String>
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct WebhookUpdateEvent{
    pub guild_id: Option<Snowflake>,
    pub channel_id: Snowflake,
}

#[derive(Debug, PartialEq, Eq, Hash, Deserialize, Serialize, Clone)]
#[serde(rename_all(
    serialize = "SCREAMING_SNAKE_CASE",
    deserialize = "SCREAMING_SNAKE_CASE"
))]
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
    pub ephemeral: bool,
}

impl Reply {
    fn new(
        message_id: Option<Snowflake>,
        channel_id: Option<Snowflake>,
        guild_id: Option<Snowflake>,
    ) -> Self {
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
            sticker_ids: None,
            ephemeral: false,
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

    /// Toggles on/off the ephemeral flag for interaction based responses.
    pub fn ephemeral(mut self) -> ReplyMessage {
        self.ephemeral = !self.ephemeral;
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

    pub async fn send_with_id(&self, channel_id: Snowflake, client: bot::BotClient) -> Message {
        let message = serde_json::json!({
            "content": self.content,
            "tts": self.tts,
            "embeds": self.embeds,
            "message_reference": self.message_reference,
            "sticker_ids": self.sticker_ids
        });

        let channel_id = match channel_id {
            Snowflake::String(value) => value,
            Snowflake::Integer(value) => value.to_string(),
        };

        let extension = format!("/channels/{}/messages", channel_id);
        let payload = discord::HttpRequest::string_new(extension, client).await;

        let response = payload.post(message).await;
        let response_message: Message;

        response_message = response
            .expect("Failed to send message!")
            .json()
            .await
            .expect("Failed to turn response into message! ");

        response_message
    }

    pub async fn callback_with_interaction(&self, interaction: Interaction, client: bot::BotClient) {

        let flags = if self.ephemeral == true {
            Some(1 << 6)
        } else {
            None
        };

        let message = serde_json::json!({
            "type": 4,
            "data": {
                    "content": self.content,
                    "tts": self.tts,
                    "embeds": self.embeds,
                    "message_reference": self.message_reference,
                    "sticker_ids": self.sticker_ids,
                    "flags": flags
                }
        });

        let extension = format!("/interactions/{}/{}/callback", interaction.id, interaction.token);
        let payload = discord::HttpRequest::string_new(extension, client).await;

        payload.post(message).await;
    }

    pub async fn send_with_interaction(&self, interaction: Interaction, client: bot::BotClient) {

        let flags = if self.ephemeral == true {
            Some(1 << 6)
        } else {
            None
        };

        let message = serde_json::json!({
            "content": self.content,
            "tts": self.tts,
            "embeds": self.embeds,
            "message_reference": self.message_reference,
            "sticker_ids": self.sticker_ids,
            "flags": flags
        });

        let extension = format!("/webhooks/{}/{}", client.lock().await.application_id.clone(), interaction.token);
        let payload = discord::HttpRequest::string_new(extension, client).await;

        payload.post(message).await;
    }
}

impl Embed {
    pub fn new(title: &str, description: &str, colour: u64) -> Self {
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
        self.video = Some(EmbedAttachment::new(url.to_string()));

        self
    }

    pub fn image(mut self, url: &str) -> Embed {
        self.image = Some(EmbedAttachment::new(url.to_string()));

        self
    }

    pub fn thumbnail(mut self, url: &str) -> Embed {
        self.thumbnail = Some(EmbedAttachment::new(url.to_string()));

        self
    }

    pub fn url(mut self, url: &str) -> Embed {
        self.url = Some(String::from(url));
        self
    }

    pub fn timestamp(mut self, timestamp: &str) -> Embed {
        self.timestamp = Some(timestamp.to_string());
        self
    }

    pub fn change_colour(mut self, new_colour: u64) -> Embed {
        self.colour = Some(new_colour);
        self
    }

    pub fn footer(mut self, text: &str, icon_url: Option<&str>) -> Embed {
        let icon_url: Option<String> = match icon_url {
            Some(value) => Some(value.to_string()),
            None => None,
        };

        self.footer = Some(EmbedFooter::new(text.to_string(), icon_url));

        self
    }

    pub fn author(mut self, name: &str, icon_url: Option<&str>, url: Option<&str>) -> Embed {
        let icon_url: Option<String> = match icon_url {
            Some(value) => Some(value.to_string()),
            None => None,
        };

        let url: Option<String> = match url {
            Some(value) => Some(value.to_string()),
            None => None,
        };

        self.author = Some(EmbedAuthor::new(name.to_string(), url, icon_url));

        self
    }

    pub fn add_field(mut self, name: &str, value: &str, inline: bool) -> Embed {
        let field = EmbedField::new(name.to_string(), value.to_string(), inline);

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
            inline: Some(inline),
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
    pub fn new(text: String, icon_url: Option<String>) -> Self {
        Self {
            text,
            icon_url,
            proxy_icon_url: None,
        }
    }
}

impl EmbedAttachment {
    pub fn new(url: String) -> Self {
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
