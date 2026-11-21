use serde::*;

#[derive(Debug, Clone, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    #[serde(default)]
    pub display_name: Option<String>,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub trust_level: Option<u8>,
    #[serde(default)]
    pub is_admin: bool,
    #[serde(default)]
    pub is_archived: bool,
    #[serde(default)]
    pub avatar_url: Option<String>,
    #[serde(default)]
    pub bio: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub last_active_at: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Channel {
    pub id: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub is_e2ee: bool,
    #[serde(default)]
    pub created_by: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub recipient_id: Option<String>,
    #[serde(default)]
    pub recipient_status: Option<String>,
    #[serde(default)]
    pub recipient_is_archived: Option<bool>,
    #[serde(default)]
    pub recipient_avatar: Option<String>,
    #[serde(default)]
    pub recipient_display_name: Option<String>,
    #[serde(default)]
    pub last_seen_at: Option<String>,
    #[serde(default)]
    pub last_message: Option<Message>,
    #[serde(default)]
    pub unread_count: Option<u32>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Message {
    pub id: String,
    pub channel_id: String,
    pub sender_id: String,
    #[serde(default)]
    pub sender_name: Option<String>,
    #[serde(default)]
    pub sender_display_name: Option<String>,
    #[serde(default)]
    pub sender_avatar: Option<String>,
    #[serde(default)]
    pub content: String,
    #[serde(default)]
    pub is_e2ee: bool,
    #[serde(default)]
    pub is_read: bool,
    #[serde(default)]
    pub reply_to: Option<String>,
    #[serde(default)]
    pub reactions: Option<String>,
    #[serde(default)]
    pub created_at: String,
}

// #[derive(Debug, Clone, Deserialize)]
// pub struct TotpSetup {
//     #[serde(default)]
//     pub totp_secret: Option<String>,
//     #[serde(default)]
//     pub totp_qr_data_uri: Option<String>,
//     #[serde(default)]
//     pub totp_auth_url: Option<String>,
//     #[serde(default)]
//     pub token: Option<String>,
// }

#[derive(Debug, Clone)]
pub enum LoginOutcome {
    Success { token: String, user: User },
    RequireTotp { message: Option<String> },
    // SetupTotp(TotpSetup),
}

impl User {
    pub fn display_name(&self) -> &str {
        self.display_name.as_deref().unwrap_or(&self.username)
    }
}

impl Channel {
    pub fn title(&self) -> &str {
        self.recipient_display_name.as_deref().unwrap_or(&self.name)
    }

    pub fn is_system(&self) -> bool {
        self.kind == "system"
    }

    pub fn is_saved(&self) -> bool {
        self.kind == "saved"
    }

    pub fn is_direct(&self) -> bool {
        self.kind == "direct"
    }

    pub fn is_archived_recipient(&self) -> bool {
        self.recipient_is_archived.unwrap_or(false)
            || self.recipient_status.as_deref().unwrap_or_default() == "ARCHIVED"
    }

    pub fn is_read_only(&self) -> bool {
        self.is_system() || self.is_archived_recipient()
    }
}

impl Message {
    pub fn body(&self) -> &str {
        &self.content
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_direct_channel() {
        let json = r#"{
                "id": "direct-b4ec62ea",
                "type": "direct",
                "name": "admin",
                "recipient_display_name": "shamper",
                "recipient_status": "ACTIVE"
            }"#;
        let channel: Channel = serde_json::from_str(json).unwrap();
        assert_eq!(channel.title(), "shamper");
        assert!(channel.is_direct());
        assert!(channel.last_message.is_none());
    }

    #[test]
    fn system_channel_with_partial_last_message() {
        let json = r#"{
            "id": "system-2deeeedf-26de-4d6c-8e9b-afcad25689c2",
            "type": "system",
            "name": "Система",
            "last_message": {
                "id": "msg-11111111-2222-3333-4444-555555555555",
                "channel_id": "system-2deeeedf-26de-4d6c-8e9b-afcad25689c2",
                "sender_id": "user-1",
                "content": "Пожалуйста, обновите свой клиент sharada!",
                "created_at": "2026-11-16T09:11:12Z"
            }
        }"#;

        let channel: Channel = serde_json::from_str(json).unwrap();

        assert!(channel.is_system());
        assert!(channel.is_read_only());

        let message = channel.last_message.as_ref().unwrap();
        assert_eq!(message.body(), "Пожалуйста, обновите свой клиент sharada!");
        assert!(message.sender_name.is_none());
        assert!(message.sender_display_name.is_none());
    }

    #[test]
    fn user_without_display_name() {
        let json = r#"{
            "id": "b4ec62ea",
            "username": "pansangg",
            "bio": "тест био"
        }"#;

        let user: User = serde_json::from_str(json).unwrap();

        assert_eq!(user.display_name(), "pansangg");
    }

    #[test]
    fn message_without_sender_display_name() {
        let json = r#"{
            "id": "111111",
            "channel_id": "222222",
            "sender_id": "333333",
            "content": "Hello World!"
        }"#;

        let msg: Message = serde_json::from_str(json).unwrap();

        assert_eq!(msg.body(), "Hello World!");
        assert!(msg.sender_display_name.is_none())
    }
}
