pub mod issue_comment_event;
pub mod push_event;

use issue_comment_event::IssueCommentEvent;
use push_event::PushEvent;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum Payload {
    PushEvent(PushEvent),
    IssueCommentEvent(IssueCommentEvent),
}

impl Payload {
    pub fn as_push_event(&self) -> Option<&PushEvent> {
        match *self {
            Payload::PushEvent(ref i) => Some(i),
            _ => None,
        }
    }

    pub fn as_issue_comment_event(&self) -> Option<&IssueCommentEvent> {
        match *self {
            Payload::IssueCommentEvent(ref i) => Some(i),
            _ => None,
        }
    }
}
