use crate::endpoints::{
    AnimeEndpoint, CharacterEndpoint, MangaEndpoint, StaffEndpoint, UserEndpoint,
};
use crate::error::AniListError;
use reqwest::Client;
use serde_json::Value;
use std::collections::HashMap;

const ANILIST_API_URL: &str = "https://graphql.anilist.co";

#[derive(Clone)]
pub struct AniListClient {
    client: Client,
}

impl AniListClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub fn anime(&self) -> AnimeEndpoint {
        AnimeEndpoint::new(self.clone())
    }

    pub fn manga(&self) -> MangaEndpoint {
        MangaEndpoint::new(self.clone())
    }

    pub fn character(&self) -> CharacterEndpoint {
        CharacterEndpoint::new(self.clone())
    }

    pub fn staff(&self) -> StaffEndpoint {
        StaffEndpoint::new(self.clone())
    }

    pub fn user(&self) -> UserEndpoint {
        UserEndpoint::new(self.clone())
    }

    pub(crate) async fn query(
        &self,
        query: &str,
        variables: Option<HashMap<String, Value>>,
    ) -> Result<Value, AniListError> {
        let mut body = HashMap::new();
        body.insert("query", Value::String(query.to_string()));

        if let Some(vars) = variables {
            body.insert("variables", Value::Object(vars.into_iter().collect()));
        }

        let response = self
            .client
            .post(ANILIST_API_URL)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        let json: Value = response.json().await?;

        if let Some(errors) = json.get("errors") {
            return Err(AniListError::GraphQL {
                message: errors.to_string(),
            });
        }

        Ok(json)
    }
}

impl Default for AniListClient {
    fn default() -> Self {
        Self::new()
    }
}
