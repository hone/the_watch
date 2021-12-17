use crate::data::payload::Payload;
use chrono::{offset::Utc, DateTime};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Event {
    pub id: String,
    pub actor: Actor,
    pub repo: Repo,
    pub public: bool,
    pub created_at: DateTime<Utc>,
    #[serde(flatten)]
    pub payload: Payload,
}

#[derive(Debug, Deserialize)]
enum EventType {
    IssueCommentEvent,
    PushEvent,
}

#[derive(Debug, Deserialize)]
pub struct Actor {
    pub id: u64,
    pub login: String,
    pub display_login: String,
    pub gravatar_id: String,
    pub url: String,
    pub avatar_url: String,
}

#[derive(Debug, Deserialize)]
pub struct Repo {
    pub id: u64,
    pub name: String,
    pub url: String,
}
