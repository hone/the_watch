use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PushEvent {
    pub push_id: u64,
    pub size: u64,
    pub distinct_size: u64,
    pub r#ref: String,
    pub head: String,
    pub before: String,
    pub commits: Vec<Commit>,
}

#[derive(Debug, Deserialize)]
pub struct Commit {
    pub sha: String,
    pub author: Author,
    pub message: String,
    pub distinct: bool,
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct Author {
    pub name: String,
    pub email: String,
}

#[cfg(test)]
mod tests {
    use crate::data::event::Event;

    #[test]
    fn it_deserializes() {
        let result: Result<Event, serde_json::Error> =
            serde_json::from_str(include_str!("../../../fixtures/push_event.json"));

        assert!(result.is_ok());
        let json = result.unwrap();
        assert_eq!(
            "refs/heads/main",
            json.payload.as_push_event().unwrap().r#ref
        );
    }
}
