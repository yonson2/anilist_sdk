use crate::client::AniListClient;
use crate::error::AniListError;
use crate::models::Manga;
use crate::queries;
use serde_json::json;
use std::collections::HashMap;

pub struct MangaEndpoint {
    client: AniListClient,
}

impl MangaEndpoint {
    pub(crate) fn new(client: AniListClient) -> Self {
        Self { client }
    }

    /// Get popular manga
    pub async fn get_popular(&self, page: i32, per_page: i32) -> Result<Vec<Manga>, AniListError> {
        let query = queries::manga::GET_POPULAR;

        let mut variables = HashMap::new();
        variables.insert("page".to_string(), json!(page));
        variables.insert("perPage".to_string(), json!(per_page));

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["Page"]["media"].clone();
        let manga_list: Vec<Manga> = serde_json::from_value(data)?;
        Ok(manga_list)
    }

    /// Get trending manga
    pub async fn get_trending(&self, page: i32, per_page: i32) -> Result<Vec<Manga>, AniListError> {
        let query = queries::manga::GET_TRENDING;

        let mut variables = HashMap::new();
        variables.insert("page".to_string(), json!(page));
        variables.insert("perPage".to_string(), json!(per_page));

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["Page"]["media"].clone();
        let manga_list: Vec<Manga> = serde_json::from_value(data)?;
        Ok(manga_list)
    }

    /// Get manga by ID
    pub async fn get_by_id(&self, id: i32) -> Result<Manga, AniListError> {
        let query = queries::manga::GET_BY_ID;

        let mut variables = HashMap::new();
        variables.insert("id".to_string(), json!(id));

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["Media"].clone();
        let manga: Manga = serde_json::from_value(data)?;
        Ok(manga)
    }

    /// Search manga by title
    pub async fn search(
        &self,
        search: &str,
        page: i32,
        per_page: i32,
    ) -> Result<Vec<Manga>, AniListError> {
        let query = queries::manga::SEARCH;

        let mut variables = HashMap::new();
        variables.insert("search".to_string(), json!(search));
        variables.insert("page".to_string(), json!(page));
        variables.insert("perPage".to_string(), json!(per_page));

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["Page"]["media"].clone();
        let manga_list: Vec<Manga> = serde_json::from_value(data)?;
        Ok(manga_list)
    }

    /// Get top rated manga
    pub async fn get_top_rated(
        &self,
        page: i32,
        per_page: i32,
    ) -> Result<Vec<Manga>, AniListError> {
        let query = queries::manga::GET_TOP_RATED;

        let mut variables = HashMap::new();
        variables.insert("page".to_string(), json!(page));
        variables.insert("perPage".to_string(), json!(per_page));

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["Page"]["media"].clone();
        let manga_list: Vec<Manga> = serde_json::from_value(data)?;
        Ok(manga_list)
    }

    /// Get currently releasing manga
    pub async fn get_releasing(
        &self,
        page: i32,
        per_page: i32,
    ) -> Result<Vec<Manga>, AniListError> {
        let query = queries::manga::GET_RELEASING;

        let mut variables = HashMap::new();
        variables.insert("page".to_string(), json!(page));
        variables.insert("perPage".to_string(), json!(per_page));

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["Page"]["media"].clone();
        let manga_list: Vec<Manga> = serde_json::from_value(data)?;
        Ok(manga_list)
    }

    /// Get completed manga
    pub async fn get_completed(
        &self,
        page: i32,
        per_page: i32,
    ) -> Result<Vec<Manga>, AniListError> {
        let query = queries::manga::GET_COMPLETED;

        let mut variables = HashMap::new();
        variables.insert("page".to_string(), json!(page));
        variables.insert("perPage".to_string(), json!(per_page));

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["Page"]["media"].clone();
        let manga_list: Vec<Manga> = serde_json::from_value(data)?;
        Ok(manga_list)
    }
}
