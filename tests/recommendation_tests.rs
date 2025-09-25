use anilist_sdk::client::AniListClient;
mod test_utils;

#[tokio::test]
async fn test_get_recent_recommendations() {
    let client = AniListClient::new();
    let result = crate::recommendation_api_call!(client, get_recent_recommendations, 1, 5);

    let recommendations = result.expect("Failed to get recent recommendations");
    // Note: This might be empty if there are no recent recommendations

    for recommendation in &recommendations {
        assert!(recommendation.id > 0);
        assert!(recommendation.media.is_some());
        assert!(recommendation.media_recommendation.is_some());
    }
}

#[tokio::test]
async fn test_get_recommendations_for_media() {
    let client = AniListClient::new();
    // Using Attack on Titan's ID (16498)
    let result =
        crate::recommendation_api_call!(client, get_recommendations_for_media, 16498, 1, 5);

    let recommendations = result.expect("Failed to get recommendations for media");
    // Note: This might be empty if the media has no recommendations

    for recommendation in &recommendations {
        assert!(recommendation.id > 0);
        if let Some(media) = &recommendation.media {
            assert_eq!(media.id, 16498);
        }
    }
}

#[tokio::test]
async fn test_get_top_rated_recommendations() {
    let client = AniListClient::new();
    let result = crate::recommendation_api_call!(client, get_top_rated_recommendations, 1, 5);

    let recommendations = result.expect("Failed to get top rated recommendations");
    // Note: This might be empty if there are no recommendations

    // Check that recommendations are ordered by rating (descending)
    let mut prev_rating = i32::MAX;
    for recommendation in &recommendations {
        assert!(recommendation.id > 0);
        if let Some(rating) = recommendation.rating {
            assert!(rating <= prev_rating);
            prev_rating = rating;
        }
    }
}

#[tokio::test]
async fn test_get_recommendation_by_id() {
    let client = AniListClient::new();
    // This test might fail if the specific recommendation doesn't exist
    let result = crate::recommendation_api_call!(client, get_recommendation_by_id, 1);

    // We just check that the call doesn't panic
    match result {
        Ok(recommendation) => {
            assert_eq!(recommendation.id, 1);
            assert!(recommendation.media.is_some());
            assert!(recommendation.media_recommendation.is_some());
        }
        Err(_) => {
            // Recommendation might not exist, which is acceptable for this test
        }
    }
}
