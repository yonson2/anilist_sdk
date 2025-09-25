use crate::client::AniListClient;
use crate::error::AniListError;
use crate::models::social::Notification;
use crate::queries;
use serde_json::json;
use std::collections::HashMap;

pub struct NotificationEndpoint {
    client: AniListClient,
}

impl NotificationEndpoint {
    pub(crate) fn new(client: AniListClient) -> Self {
        Self { client }
    }

    /// Get user notifications (requires authentication)
    pub async fn get_notifications(
        &self,
        page: i32,
        per_page: i32,
    ) -> Result<Vec<Notification>, AniListError> {
        let query = queries::notification::GET_NOTIFICATIONS;

        let mut variables = HashMap::new();
        variables.insert("page".to_string(), json!(page));
        variables.insert("perPage".to_string(), json!(per_page));

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["Page"]["notifications"].clone();
        let notifications: Vec<Notification> = serde_json::from_value(data)?;
        Ok(notifications)
    }

    /// Get unread notification count (requires authentication)
    pub async fn get_unread_count(&self) -> Result<i32, AniListError> {
        let query = queries::notification::GET_UNREAD_COUNT;

        let response = self.client.query(query, None).await?;
        let count = response["data"]["Viewer"]["unreadNotificationCount"]
            .as_i64()
            .unwrap_or(0) as i32;
        Ok(count)
    }

    /// Get notifications by type (requires authentication)
    pub async fn get_notifications_by_type(
        &self,
        notification_type: &str,
        page: i32,
        per_page: i32,
    ) -> Result<Vec<Notification>, AniListError> {
        let query = queries::notification::GET_NOTIFICATIONS_BY_TYPE;

        let mut variables = HashMap::new();
        variables.insert("type".to_string(), json!([notification_type]));
        variables.insert("page".to_string(), json!(page));
        variables.insert("perPage".to_string(), json!(per_page));

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["Page"]["notifications"].clone();
        let notifications: Vec<Notification> = serde_json::from_value(data)?;
        Ok(notifications)
    }

    /// Mark notifications as read (requires authentication)
    pub async fn mark_notifications_as_read(
        &self,
        notification_ids: Vec<i32>,
    ) -> Result<bool, AniListError> {
        let query = queries::notification::MARK_NOTIFICATIONS_AS_READ;

        let mut variables = HashMap::new();
        variables.insert("notificationIds".to_string(), json!(notification_ids));

        let response = self.client.query(query, Some(variables)).await?;
        // If we get a response without errors, consider it successful
        Ok(response["data"]["SaveNotificationSettings"].is_object())
    }
}
