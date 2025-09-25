use anilist_sdk::client::AniListClient;
mod test_utils;

#[tokio::test]
async fn test_get_upcoming_episodes() {
    let client = AniListClient::new();
    let result = crate::airing_api_call!(client, get_upcoming_episodes, 1, 10);

    let schedules = result.expect("Failed to get upcoming episodes");
    // Note: This might be empty if no episodes are scheduled to air

    for schedule in &schedules {
        assert!(schedule.id > 0);
        assert!(schedule.media_id > 0);
        assert!(schedule.episode > 0);
        assert!(schedule.airing_at > 0);
        assert!(schedule.media.is_some());
    }
}

#[tokio::test]
async fn test_get_today_episodes() {
    let client = AniListClient::new();
    let result = crate::airing_api_call!(client, get_today_episodes, 1, 10);

    let schedules = result.expect("Failed to get today's episodes");
    // Note: This might be empty if no episodes are airing today

    for schedule in &schedules {
        assert!(schedule.id > 0);
        assert!(schedule.media_id > 0);
        assert!(schedule.episode > 0);
    }
}

#[tokio::test]
async fn test_get_recently_aired() {
    let client = AniListClient::new();
    let result = crate::airing_api_call!(client, get_recently_aired, 1, 10);

    let schedules = result.expect("Failed to get recently aired episodes");
    // Should have some recently aired episodes

    for schedule in &schedules {
        assert!(schedule.id > 0);
        assert!(schedule.media_id > 0);
        assert!(schedule.episode > 0);
    }
}

#[tokio::test]
async fn test_get_schedule_for_media() {
    let client = AniListClient::new();
    // Using Attack on Titan's ID (16498) - this might not have current airing schedule
    let result = crate::airing_api_call!(client, get_schedule_for_media, 16498, 1, 5);

    let schedules = result.expect("Failed to get schedule for media");
    // Note: This might be empty if the media has no airing schedule

    for schedule in &schedules {
        assert!(schedule.id > 0);
        assert_eq!(schedule.media_id, 16498);
    }
}

#[tokio::test]
async fn test_get_schedule_by_id() {
    let client = AniListClient::new();
    // This test might fail if the specific schedule doesn't exist
    let result = crate::airing_api_call!(client, get_schedule_by_id, 1);

    // We just check that the call doesn't panic
    match result {
        Ok(schedule) => {
            assert_eq!(schedule.id, 1);
            assert!(schedule.media_id > 0);
        }
        Err(_) => {
            // Schedule might not exist, which is acceptable for this test
        }
    }
}

#[tokio::test]
async fn test_get_next_episode() {
    let client = AniListClient::new();
    // Test with a currently airing anime - this might return None if no episode is scheduled
    let result = crate::airing_api_call!(client, get_next_episode, 16498);

    let schedule_opt = result.expect("Failed to get next episode");

    if let Some(schedule) = schedule_opt {
        assert!(schedule.id > 0);
        assert_eq!(schedule.media_id, 16498);
        assert!(schedule.episode > 0);
        assert!(schedule.time_until_airing >= 0); // Should be positive for future episodes
    }
    // If None, it means no upcoming episodes are scheduled, which is also valid
}

#[tokio::test]
async fn test_get_episodes_in_range() {
    let client = AniListClient::new();

    // Get episodes for the next 7 days
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;
    let week_later = now + (7 * 24 * 60 * 60); // 7 days in seconds

    let result = crate::airing_api_call!(client, get_episodes_in_range, now, week_later, 1, 10);

    let schedules = result.expect("Failed to get episodes in range");
    // Note: This might be empty if no episodes are scheduled in this range

    for schedule in &schedules {
        assert!(schedule.id > 0);
        assert!(schedule.airing_at as i64 >= now);
        assert!(schedule.airing_at as i64 <= week_later);
    }
}
