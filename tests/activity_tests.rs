use anilist_sdk::client::AniListClient;
mod test_utils;

#[tokio::test]
async fn test_get_recent_activities() {
    let client = AniListClient::new();
    let result = crate::activity_api_call!(client, get_recent_activities, 1, 5);

    let activities = result.expect("Failed to get recent activities");
    // Note: This might be empty based on privacy settings

    for activity in &activities {
        assert!(activity.id > 0);
    }
}

#[tokio::test]
async fn test_get_text_activities() {
    let client = AniListClient::new();
    let result = crate::activity_api_call!(client, get_text_activities, 1, 5);

    let activities = result.expect("Failed to get text activities");
    // Note: This might be empty based on privacy settings

    for activity in &activities {
        assert!(activity.id > 0);
    }
}

#[tokio::test]
async fn test_get_user_activities() {
    let client = AniListClient::new();
    // Test with a known user ID (this might fail if the user doesn't exist or has private activities)
    let result = crate::activity_api_call!(client, get_user_activities, 1, 1, 5);

    // We just check that the call doesn't panic
    match result {
        Ok(activities) => {
            for activity in &activities {
                assert!(activity.id > 0);
            }
        }
        Err(_) => {
            // User might not exist or have private activities, which is acceptable
        }
    }
}

#[tokio::test]
async fn test_get_activity_by_id() {
    let client = AniListClient::new();
    // This test might fail if the specific activity doesn't exist
    let result = crate::activity_api_call!(client, get_activity_by_id, 1);

    // We just check that the call doesn't panic
    match result {
        Ok(activity) => {
            assert_eq!(activity.id, 1);
        }
        Err(_) => {
            // Activity might not exist, which is acceptable for this test
        }
    }
}

#[tokio::test]
async fn test_get_activity_replies() {
    let client = AniListClient::new();
    // This test might fail if the specific activity doesn't exist or has no replies
    let result = crate::activity_api_call!(client, get_activity_replies, 1, 1, 5);

    // We just check that the call doesn't panic
    match result {
        Ok(replies) => {
            for reply in &replies {
                assert!(reply.id > 0);
            }
        }
        Err(_) => {
            // Activity might not exist or have no replies, which is acceptable
        }
    }
}
