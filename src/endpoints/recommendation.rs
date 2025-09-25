use crate::client::AniListClient;
use crate::error::AniListError;
use crate::models::social::Recommendation;
use crate::queries;
use serde_json::json;
use std::collections::HashMap;

pub struct RecommendationEndpoint {
    client: AniListClient,
}

impl RecommendationEndpoint {
    pub(crate) fn new(client: AniListClient) -> Self {
        Self { client }
    }

    /// Get recent recommendations
    pub async fn get_recent_recommendations(
        &self,
        page: i32,
        per_page: i32,
    ) -> Result<Vec<Recommendation>, AniListError> {
        let query = queries::recommendation::GET_RECENT_RECOMMENDATIONS;

        let mut variables = HashMap::new();
        variables.insert("page".to_string(), json!(page));
        variables.insert("perPage".to_string(), json!(per_page));

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["Page"]["recommendations"].clone();
        let recommendations: Vec<Recommendation> = serde_json::from_value(data)?;
        Ok(recommendations)
    }

    /// Get recommendations for a specific media
    pub async fn get_recommendations_for_media(
        &self,
        media_id: i32,
        page: i32,
        per_page: i32,
    ) -> Result<Vec<Recommendation>, AniListError> {
        let query = queries::recommendation::GET_RECOMMENDATIONS_FOR_MEDIA;

        let mut variables = HashMap::new();
        variables.insert("mediaId".to_string(), json!(media_id));
        variables.insert("page".to_string(), json!(page));
        variables.insert("perPage".to_string(), json!(per_page));

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["Page"]["recommendations"].clone();
        let recommendations: Vec<Recommendation> = serde_json::from_value(data)?;
        Ok(recommendations)
    }

    /// Get top rated recommendations
    pub async fn get_top_rated_recommendations(
        &self,
        page: i32,
        per_page: i32,
    ) -> Result<Vec<Recommendation>, AniListError> {
        let query = queries::recommendation::GET_TOP_RATED_RECOMMENDATIONS;

        let mut variables = HashMap::new();
        variables.insert("page".to_string(), json!(page));
        variables.insert("perPage".to_string(), json!(per_page));

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["Page"]["recommendations"].clone();
        let recommendations: Vec<Recommendation> = serde_json::from_value(data)?;
        Ok(recommendations)
    }

    /// Get recommendation by ID
    pub async fn get_recommendation_by_id(&self, id: i32) -> Result<Recommendation, AniListError> {
        let query = queries::recommendation::GET_RECOMMENDATION_BY_ID;

        let mut variables = HashMap::new();
        variables.insert("id".to_string(), json!(id));

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["Recommendation"].clone();
        let recommendation: Recommendation = serde_json::from_value(data)?;
        Ok(recommendation)
    }

    /// Create a recommendation (requires authentication)
    pub async fn save_recommendation(
        &self,
        media_id: i32,
        media_recommendation_id: i32,
        rating: Option<i32>,
    ) -> Result<Recommendation, AniListError> {
        let query = queries::recommendation::SAVE_RECOMMENDATION;

        let mut variables = HashMap::new();
        variables.insert("mediaId".to_string(), json!(media_id));
        variables.insert(
            "mediaRecommendationId".to_string(),
            json!(media_recommendation_id),
        );
        if let Some(r) = rating {
            let rating_str = match r {
                1 => "RATE_UP",
                -1 => "RATE_DOWN",
                _ => "NO_RATING",
            };
            variables.insert("rating".to_string(), json!(rating_str));
        }

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["SaveRecommendation"].clone();
        let recommendation: Recommendation = serde_json::from_value(data)?;
        Ok(recommendation)
    }

    /// Rate a recommendation (requires authentication)
    pub async fn rate_recommendation(
        &self,
        recommendation_id: i32,
        rating: i32,
    ) -> Result<Recommendation, AniListError> {
        let rating_str = match rating {
            1 => "RATE_UP",
            -1 => "RATE_DOWN",
            _ => "NO_RATING",
        };

        let query = queries::recommendation::RATE_RECOMMENDATION;

        let mut variables = HashMap::new();
        variables.insert("recommendationId".to_string(), json!(recommendation_id));
        variables.insert("rating".to_string(), json!(rating_str));

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["SaveRecommendation"].clone();
        let recommendation: Recommendation = serde_json::from_value(data)?;
        Ok(recommendation)
    }
}
