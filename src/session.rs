use std::path::PathBuf;

pub struct Session {
    path: PathBuf,
    token: Option<String>,
    device_id: String,
    user_id: Option<String>,
    username: Option<String>,
}
#[derive(Debug, thiserror::Error)]
pub enum SessionError {
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("json: {0}")]
    Json(#[from] serde_json::Error),
}

impl Session {
    pub fn load() -> Self;
    pub fn load_from(path: PathBuf) -> Self;
    pub fn token(&self) -> Option<&str>;
    pub fn device_id(&self) -> &str;
    pub fn user_id(&self) -> Option<&str>;
    pub fn username(&self) -> Option<&str>;
    pub fn warning(&self) -> Option<&str>;
    pub fn save_token(&mut self, token: &str) -> Result<(), SessionError>;
    pub fn save_user(&mut self, user_id: &str, username: &str) -> Result<(), SessionError>;
    pub fn clear_token(&mut self) -> Result<(), SessionError>;
}
