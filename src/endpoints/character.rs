use crate::client::AniListClient;
use crate::error::AniListError;
use crate::models::character::Character;
use crate::queries;
use serde_json::json;
use std::collections::HashMap;

pub struct CharacterEndpoint {
    client: AniListClient,
}

impl CharacterEndpoint {
    pub(crate) fn new(client: AniListClient) -> Self {
        Self { client }
    }

    /// Get popular characters
    pub async fn get_popular(
        &self,
        page: i32,
        per_page: i32,
    ) -> Result<Vec<Character>, AniListError> {
        let query = queries::character::GET_POPULAR;

        let mut variables = HashMap::new();
        variables.insert("page".to_string(), json!(page));
        variables.insert("perPage".to_string(), json!(per_page));

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["Page"]["characters"].clone();
        let characters: Vec<Character> = serde_json::from_value(data)?;
        Ok(characters)
    }

    /// Get character by ID
    pub async fn get_by_id(&self, id: i32) -> Result<Character, AniListError> {
        let query = queries::character::GET_BY_ID;

        let mut variables = HashMap::new();
        variables.insert("id".to_string(), json!(id));

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["Character"].clone();
        let character: Character = serde_json::from_value(data)?;
        Ok(character)
    }

    /// Search characters by name
    pub async fn search(
        &self,
        search: &str,
        page: i32,
        per_page: i32,
    ) -> Result<Vec<Character>, AniListError> {
        let query = queries::character::SEARCH;

        let mut variables = HashMap::new();
        variables.insert("search".to_string(), json!(search));
        variables.insert("page".to_string(), json!(page));
        variables.insert("perPage".to_string(), json!(per_page));

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["Page"]["characters"].clone();
        let characters: Vec<Character> = serde_json::from_value(data)?;
        Ok(characters)
    }

    /// Get characters who have birthday today
    pub async fn get_today_birthday(
        &self,
        page: i32,
        per_page: i32,
    ) -> Result<Vec<Character>, AniListError> {
        let query = queries::character::GET_TODAY_BIRTHDAY;

        let mut variables = HashMap::new();
        variables.insert("page".to_string(), json!(page));
        variables.insert("perPage".to_string(), json!(per_page));

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["Page"]["characters"].clone();
        let characters: Vec<Character> = serde_json::from_value(data)?;

        Ok(characters)
    }

    /// Get most favorited characters
    pub async fn get_most_favorited(
        &self,
        page: i32,
        per_page: i32,
    ) -> Result<Vec<Character>, AniListError> {
        let query = queries::character::GET_MOST_FAVORITED;

        let mut variables = HashMap::new();
        variables.insert("page".to_string(), json!(page));
        variables.insert("perPage".to_string(), json!(per_page));

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["Page"]["characters"].clone();
        let characters: Vec<Character> = serde_json::from_value(data)?;
        Ok(characters)
    }
}
