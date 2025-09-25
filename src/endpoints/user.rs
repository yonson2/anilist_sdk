use crate::client::AniListClient;
use crate::error::AniListError;
use crate::models::FuzzyDate;
use crate::models::media_list::{MediaList, MediaListStatus};
use crate::models::user::User;
use crate::queries;
use serde_json::json;
use std::collections::HashMap;

pub struct UserEndpoint {
    client: AniListClient,
}

impl UserEndpoint {
    pub(crate) fn new(client: AniListClient) -> Self {
        Self { client }
    }

    /// Get the currently authenticated user (requires token)
    pub async fn get_current_user(&self) -> Result<User, AniListError> {
        let query = queries::user::GET_CURRENT_USER;

        let response = self.client.query(query, None).await?;
        let data = response["data"]["Viewer"].clone();
        let user: User = serde_json::from_value(data)?;
        Ok(user)
    }

    /// Get the current user's anime list (requires token)
    pub async fn get_current_user_anime_list(
        &self,
        status: Option<&str>,
    ) -> Result<Vec<MediaList>, AniListError> {
        let query = queries::user::GET_CURRENT_USER_ANIME_LIST;

        let mut variables = HashMap::new();
        variables.insert("type".to_string(), json!("ANIME"));
        variables.insert(
            "userId".to_string(),
            json!(self.client.user().get_current_user().await?.id),
        );

        if let Some(status) = status {
            variables.insert("status".to_string(), json!(status.to_uppercase()));
        }

        let response = self.client.query(query, Some(variables)).await?;

        // Extract entries from all lists
        let mut all_entries = Vec::new();
        if let Some(lists) = response["data"]["MediaListCollection"]["lists"].as_array() {
            for list in lists {
                if let Some(entries) = list["entries"].as_array() {
                    for entry in entries {
                        if let Ok(media_list) = serde_json::from_value::<MediaList>(entry.clone()) {
                            all_entries.push(media_list);
                        }
                    }
                }
            }
        }

        Ok(all_entries)
    }

    /// Get user by ID
    pub async fn get_by_id(&self, id: i32) -> Result<User, AniListError> {
        let query = queries::user::GET_BY_ID;

        let mut variables = HashMap::new();
        variables.insert("id".to_string(), json!(id));

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["User"].clone();
        let user: User = serde_json::from_value(data)?;
        Ok(user)
    }

    /// Get user by name
    pub async fn get_by_name(&self, name: &str) -> Result<User, AniListError> {
        let query = queries::user::GET_BY_NAME;

        let mut variables = HashMap::new();
        variables.insert("name".to_string(), json!(name));

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["User"].clone();
        let user: User = serde_json::from_value(data)?;
        Ok(user)
    }

    /// Search users by name
    pub async fn search(
        &self,
        search: &str,
        page: i32,
        per_page: i32,
    ) -> Result<Vec<User>, AniListError> {
        let query = queries::user::SEARCH;

        let mut variables = HashMap::new();
        variables.insert("search".to_string(), json!(search));
        variables.insert("page".to_string(), json!(page));
        variables.insert("perPage".to_string(), json!(per_page));

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["Page"]["users"].clone();
        let users: Vec<User> = serde_json::from_value(data)?;
        Ok(users)
    }

    /// Get users with most anime watched
    pub async fn get_most_anime_watched(
        &self,
        page: i32,
        per_page: i32,
    ) -> Result<Vec<User>, AniListError> {
        let query = queries::user::GET_MOST_ANIME_WATCHED;

        let mut variables = HashMap::new();
        variables.insert("page".to_string(), json!(page));
        variables.insert("perPage".to_string(), json!(per_page));

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["Page"]["users"].clone();
        let users: Vec<User> = serde_json::from_value(data)?;
        Ok(users)
    }

    /// Get users with most manga read
    pub async fn get_most_manga_read(
        &self,
        page: i32,
        per_page: i32,
    ) -> Result<Vec<User>, AniListError> {
        let query = queries::user::GET_MOST_MANGA_READ;

        let mut variables = HashMap::new();
        variables.insert("page".to_string(), json!(page));
        variables.insert("perPage".to_string(), json!(per_page));

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["Page"]["users"].clone();
        let users: Vec<User> = serde_json::from_value(data)?;
        Ok(users)
    }

    /// Toggle follow/unfollow a user (requires authentication)
    ///
    /// # Arguments
    /// * `user_id` - The ID of the user to follow/unfollow
    ///
    /// # Returns
    /// Returns the updated User object with follow status
    ///
    /// # Errors
    /// * `AniListError::Unauthorized` - If no authentication token is provided
    /// * `AniListError::Network` - If there's a network connectivity issue
    /// * `AniListError::ApiError` - If the AniList API returns an error
    ///
    /// # Example
    /// ```rust
    /// let user = client.user().toggle_follow(123456).await?;
    /// println!("User {} follow status: {}", user.name, user.is_following.unwrap_or(false));
    /// ```
    pub async fn toggle_follow(&self, user_id: i32) -> Result<User, AniListError> {
        let query = queries::user::TOGGLE_FOLLOW;

        let mut variables = HashMap::new();
        variables.insert("userId".to_string(), json!(user_id));

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["ToggleFollow"].clone();
        let user: User = serde_json::from_value(data)?;
        Ok(user)
    }

    /// Toggle favorite anime/manga for the authenticated user
    ///
    /// # Arguments
    /// * `anime_id` - The ID of the anime to favorite/unfavorite (optional)
    /// * `manga_id` - The ID of the manga to favorite/unfavorite (optional)
    ///
    /// # Returns
    /// Returns a simple boolean indicating success
    ///
    /// # Errors
    /// * `AniListError::Unauthorized` - If no authentication token is provided
    /// * `AniListError::InvalidInput` - If neither anime_id nor manga_id is provided
    /// * `AniListError::Network` - If there's a network connectivity issue
    /// * `AniListError::ApiError` - If the AniList API returns an error
    ///
    /// # Example
    /// ```rust
    /// // Favorite an anime
    /// let success = client.user().toggle_favorite(Some(21), None).await?;
    ///
    /// // Favorite a manga
    /// let success = client.user().toggle_favorite(None, Some(30013)).await?;
    /// ```
    pub async fn toggle_favorite(
        &self,
        anime_id: Option<i32>,
        manga_id: Option<i32>,
    ) -> Result<bool, AniListError> {
        if anime_id.is_none() && manga_id.is_none() {
            return Err(AniListError::BadRequest {
                message: "Either anime_id or manga_id must be provided".to_string(),
            });
        }

        let query = queries::user::TOGGLE_FAVORITE;

        let mut variables = HashMap::new();
        if let Some(id) = anime_id {
            variables.insert("animeId".to_string(), json!(id));
        }
        if let Some(id) = manga_id {
            variables.insert("mangaId".to_string(), json!(id));
        }

        let response = self.client.query(query, Some(variables)).await?;
        // The mutation returns the updated favourites object, but we'll just return success
        Ok(response["data"]["ToggleFavourite"].is_object())
    }

    /// Update the progress of a media list entry (requires authentication)
    ///
    /// # Arguments
    /// * `media_list_entry_id` - The ID of the media list entry to update
    /// * `progress` - The new progress value (episodes watched)
    ///
    /// # Returns
    /// Returns `()` on successful update
    ///
    /// # Errors
    /// * `AniListError::AuthenticationRequired` - If no authentication token is provided
    /// * `AniListError::Network` - If there's a network connectivity issue
    /// * `AniListError::GraphQL` - If the AniList API returns an error
    ///
    /// # Example
    /// ```rust
    /// client.user().update_media_list_progress(123456, 12).await?;
    /// println!("Progress updated successfully!");
    /// ```
    pub async fn update_media_list_progress(
        &self,
        media_list_entry_id: i32,
        progress: i32,
    ) -> Result<(), AniListError> {
        let query = queries::user::UPDATE_MEDIA_LIST_PROGRESS;

        let mut variables = HashMap::new();
        variables.insert(
            "saveMediaListEntryId".to_string(),
            json!(media_list_entry_id),
        );
        variables.insert("progress".to_string(), json!(progress));

        self.client.query(query, Some(variables)).await?;
        Ok(())
    }

    /// Update the status of a media list entry (requires authentication)
    ///
    /// # Arguments
    /// * `media_list_entry_id` - The ID of the media list entry to update
    /// * `status` - The new status (Current, Completed, Dropped, etc.)
    /// * `completed_at` - Optional completion date (when status is set to Completed)
    ///
    /// # Returns
    /// Returns `()` on successful update
    ///
    /// # Errors
    /// * `AniListError::AuthenticationRequired` - If no authentication token is provided
    /// * `AniListError::Network` - If there's a network connectivity issue
    /// * `AniListError::GraphQL` - If the AniList API returns an error
    ///
    /// # Example
    /// ```rust
    /// use crate::models::media_list::MediaListStatus;
    /// use crate::models::FuzzyDate;
    ///
    /// // Mark as completed with completion date
    /// let completion_date = FuzzyDate {
    ///     year: Some(2024),
    ///     month: Some(3),
    ///     day: Some(15),
    /// };
    /// client.user().update_media_list_status(123456, MediaListStatus::Completed, Some(completion_date)).await?;
    ///
    /// // Just change status without completion date
    /// client.user().update_media_list_status(123456, MediaListStatus::Dropped, None).await?;
    /// ```
    pub async fn update_media_list_status(
        &self,
        media_list_entry_id: i32,
        status: MediaListStatus,
        completed_at: Option<FuzzyDate>,
    ) -> Result<(), AniListError> {
        let query = queries::user::UPDATE_MEDIA_LIST_STATUS;

        let mut variables = HashMap::new();
        variables.insert(
            "saveMediaListEntryId".to_string(),
            json!(media_list_entry_id),
        );
        variables.insert("status".to_string(), json!(status));

        if let Some(completed_at) = completed_at {
            variables.insert("completedAt".to_string(), json!(completed_at));
        }

        self.client.query(query, Some(variables)).await?;
        Ok(())
    }
}
