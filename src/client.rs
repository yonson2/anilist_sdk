use crate::endpoints::{
    AnimeEndpoint, CharacterEndpoint, MangaEndpoint, StaffEndpoint, UserEndpoint,
    StudioEndpoint, ForumEndpoint, ActivityEndpoint, ReviewEndpoint, 
    RecommendationEndpoint, AiringEndpoint, NotificationEndpoint,
};
use crate::error::AniListError;
use reqwest::Client;
use serde_json::Value;
use std::collections::HashMap;

const ANILIST_API_URL: &str = "https://graphql.anilist.co";

#[derive(Clone)]
pub struct AniListClient {
    client: Client,
    token: Option<String>,
}

impl AniListClient {
    /// Create a new unauthenticated client
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            token: None,
        }
    }

    /// Create a new authenticated client with an access token
    pub fn with_token(token: String) -> Self {
        Self {
            client: Client::new(),
            token: Some(token),
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

    pub fn studio(&self) -> StudioEndpoint {
        StudioEndpoint::new(self.clone())
    }

    pub fn forum(&self) -> ForumEndpoint {
        ForumEndpoint::new(self.clone())
    }

    pub fn activity(&self) -> ActivityEndpoint {
        ActivityEndpoint::new(self.clone())
    }

    pub fn review(&self) -> ReviewEndpoint {
        ReviewEndpoint::new(self.clone())
    }

    pub fn recommendation(&self) -> RecommendationEndpoint {
        RecommendationEndpoint::new(self.clone())
    }

    pub fn airing(&self) -> AiringEndpoint {
        AiringEndpoint::new(self.clone())
    }

    pub fn notification(&self) -> NotificationEndpoint {
        NotificationEndpoint::new(self.clone())
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

        let mut request = self
            .client
            .post(ANILIST_API_URL)
            .header("Content-Type", "application/json");

        // Add authorization header if token is present
        if let Some(token) = &self.token {
            request = request.header("Authorization", format!("Bearer {}", token));
        }

        let response = request
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
