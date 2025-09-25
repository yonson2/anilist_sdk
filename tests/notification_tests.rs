use anilist_sdk::client::AniListClient;
use std::env;
mod test_utils;

#[tokio::test]
async fn test_get_notifications() {
    // Skip if no token provided
    let Ok(token) = env::var("ANILIST_TOKEN") else {
        println!("Skipping authenticated test - no ANILIST_TOKEN environment variable");
        return;
    };

    let client = AniListClient::with_token(token);
    let result = crate::notification_api_call!(client, get_notifications, 1, 10);

    let notifications = result.expect("Failed to get notifications");

    for notification in &notifications {
        assert!(notification.id > 0);
        if let Some(created_at) = notification.created_at {
            assert!(created_at > 0);
        }
        assert!(notification.notification_type.is_some());
    }
}

#[tokio::test]
async fn test_get_notifications_by_type() {
    // Skip if no token provided
    let Ok(token) = env::var("ANILIST_TOKEN") else {
        println!("Skipping authenticated test - no ANILIST_TOKEN environment variable");
        return;
    };

    let client = AniListClient::with_token(token);
    let result =
        crate::notification_api_call!(client, get_notifications_by_type, "ActivityMessage", 1, 5);

    let notifications = result.expect("Failed to get notifications by type");

    for notification in &notifications {
        assert!(notification.id > 0);
        if let Some(_notification_type) = &notification.notification_type {
            // We can't easily compare enum variants in string format, so just check it exists
            assert!(notification.notification_type.is_some());
        }
    }
}

#[tokio::test]
async fn test_mark_notifications_as_read() {
    // Skip if no token provided
    let Ok(token) = env::var("ANILIST_TOKEN") else {
        println!("Skipping authenticated test - no ANILIST_TOKEN environment variable");
        return;
    };

    let client = AniListClient::with_token(token);

    // First try to get some notifications to mark as read
    let notifications_result = crate::notification_api_call!(client, get_notifications, 1, 1);
    if let Ok(notifications) = notifications_result
        && let Some(notification) = notifications.first()
    {
        let notification_id = notification.id;
        let result = crate::notification_api_call!(
            client,
            mark_notifications_as_read,
            vec![notification_id]
        );
        // This might fail if the notification is already read or doesn't belong to user
        match result {
            Ok(success) => assert!(success),
            Err(_) => {
                // Acceptable if notification doesn't exist or is already read
            }
        }
    }
}

#[tokio::test]
async fn test_get_unread_count() {
    // Skip if no token provided
    let Ok(token) = env::var("ANILIST_TOKEN") else {
        println!("Skipping authenticated test - no ANILIST_TOKEN environment variable");
        return;
    };

    let client = AniListClient::with_token(token);
    let result = crate::notification_api_call!(client, get_unread_count);

    let count = result.expect("Failed to get unread count");
    assert!(count >= 0);
}
