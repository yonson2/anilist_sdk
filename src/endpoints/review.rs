use crate::client::AniListClient;
use crate::error::AniListError;
use crate::models::social::Review;
use crate::queries;
use serde_json::json;
use std::collections::HashMap;

pub struct ReviewEndpoint {
    client: AniListClient,
}

impl ReviewEndpoint {
    pub(crate) fn new(client: AniListClient) -> Self {
        Self { client }
    }

    /// Get recent reviews
    pub async fn get_recent_reviews(
        &self,
        page: i32,
        per_page: i32,
    ) -> Result<Vec<Review>, AniListError> {
        let query = queries::review::GET_RECENT_REVIEWS;

        let mut variables = HashMap::new();
        variables.insert("page".to_string(), json!(page));
        variables.insert("perPage".to_string(), json!(per_page));

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["Page"]["reviews"].clone();
        let reviews: Vec<Review> = serde_json::from_value(data)?;
        Ok(reviews)
    }

    /// Get reviews by media ID
    pub async fn get_reviews_for_media(
        &self,
        media_id: i32,
        page: i32,
        per_page: i32,
    ) -> Result<Vec<Review>, AniListError> {
        let query = queries::review::GET_REVIEWS_FOR_MEDIA;

        let mut variables = HashMap::new();
        variables.insert("mediaId".to_string(), json!(media_id));
        variables.insert("page".to_string(), json!(page));
        variables.insert("perPage".to_string(), json!(per_page));

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["Page"]["reviews"].clone();
        let reviews: Vec<Review> = serde_json::from_value(data)?;
        Ok(reviews)
    }

    /// Get reviews by user ID
    pub async fn get_reviews_by_user(
        &self,
        user_id: i32,
        page: i32,
        per_page: i32,
    ) -> Result<Vec<Review>, AniListError> {
        let query = queries::review::GET_REVIEWS_BY_USER;

        let mut variables = HashMap::new();
        variables.insert("userId".to_string(), json!(user_id));
        variables.insert("page".to_string(), json!(page));
        variables.insert("perPage".to_string(), json!(per_page));

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["Page"]["reviews"].clone();
        let reviews: Vec<Review> = serde_json::from_value(data)?;
        Ok(reviews)
    }

    /// Get review by ID
    pub async fn get_review_by_id(&self, id: i32) -> Result<Review, AniListError> {
        let query = queries::review::GET_REVIEW_BY_ID;

        let mut variables = HashMap::new();
        variables.insert("id".to_string(), json!(id));

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["Review"].clone();
        let review: Review = serde_json::from_value(data)?;
        Ok(review)
    }

    /// Create or update a review (requires authentication)
    pub async fn save_review(
        &self,
        media_id: i32,
        body: &str,
        summary: Option<&str>,
        score: Option<i32>,
        private: Option<bool>,
    ) -> Result<Review, AniListError> {
        let query = queries::review::SAVE_REVIEW;

        let mut variables = HashMap::new();
        variables.insert("mediaId".to_string(), json!(media_id));
        variables.insert("body".to_string(), json!(body));
        if let Some(s) = summary {
            variables.insert("summary".to_string(), json!(s));
        }
        if let Some(sc) = score {
            variables.insert("score".to_string(), json!(sc));
        }
        if let Some(p) = private {
            variables.insert("private".to_string(), json!(p));
        }

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["SaveReview"].clone();
        let review: Review = serde_json::from_value(data)?;
        Ok(review)
    }

    /// Rate a review (requires authentication)
    pub async fn rate_review(&self, review_id: i32, rating: &str) -> Result<Review, AniListError> {
        let query = queries::review::RATE_REVIEW;

        let mut variables = HashMap::new();
        variables.insert("reviewId".to_string(), json!(review_id));
        variables.insert("rating".to_string(), json!(rating));

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["RateReview"].clone();
        let review: Review = serde_json::from_value(data)?;
        Ok(review)
    }

    /// Delete a review (requires authentication and ownership)
    pub async fn delete_review(&self, id: i32) -> Result<bool, AniListError> {
        let query = queries::review::DELETE_REVIEW;

        let mut variables = HashMap::new();
        variables.insert("id".to_string(), json!(id));

        let response = self.client.query(query, Some(variables)).await?;
        let deleted = response["data"]["DeleteReview"]["deleted"]
            .as_bool()
            .unwrap_or(false);
        Ok(deleted)
    }

    /// Get top rated reviews
    pub async fn get_top_rated_reviews(
        &self,
        page: i32,
        per_page: i32,
    ) -> Result<Vec<Review>, AniListError> {
        let query = queries::review::GET_TOP_RATED_REVIEWS;

        let mut variables = HashMap::new();
        variables.insert("page".to_string(), json!(page));
        variables.insert("perPage".to_string(), json!(per_page));

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["Page"]["reviews"].clone();
        let reviews: Vec<Review> = serde_json::from_value(data)?;
        Ok(reviews)
    }
}
