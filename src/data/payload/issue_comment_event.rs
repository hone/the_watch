use chrono::{offset::Utc, DateTime};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct IssueCommentEvent {
    action: String,
    issue: Issue,
    comment: Comment,
}

#[derive(Debug, Deserialize)]
pub struct Issue {
    url: String,
    repository_url: String,
    labels_url: String,
    comments_url: String,
    html_url: String,
    id: u64,
    node_id: String,
    number: u64,
    title: String,
    user: User,
    // example was empty, should double check contents
    labels: Vec<String>,
    // probably should be an enum
    state: String,
    locked: bool,
    // example was null, should double check contents
    assignee: Option<String>,
    // example was empty, should double check contents
    assignees: Vec<String>,
    // example was null, should double check contents
    milestone: Option<String>,
    comments: u64,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    // example was null, should double check contents
    closed_at: Option<DateTime<Utc>>,
    // probably should be an enum
    author_association: String,
    // example was null, should double check contents
    active_lock_reason: Option<String>,
    body: String,
    // example was null, should double check contents
    performed_via_github_app: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Comment {
    url: String,
    html_url: String,
    issue_url: String,
}

#[derive(Debug, Deserialize)]
pub struct User {}
