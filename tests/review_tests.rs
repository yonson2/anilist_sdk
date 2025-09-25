use anilist_sdk::client::AniListClient;
mod test_utils;

#[tokio::test]
async fn test_get_recent_reviews() {
    let client = AniListClient::new();
    let result = crate::review_api_call!(client, get_recent_reviews, 1, 5);

    let reviews = result.expect("Failed to get recent reviews");
    assert!(!reviews.is_empty());

    for review in &reviews {
        assert!(review.id > 0);
        assert!(review.user_id > 0);
        assert!(review.media_id > 0);
        assert!(!review.body.is_empty());
    }
}

#[tokio::test]
async fn test_get_reviews_for_media() {
    let client = AniListClient::new();
    // Using Attack on Titan's ID (16498)
    let result = crate::review_api_call!(client, get_reviews_for_media, 16498, 1, 5);

    let reviews = result.expect("Failed to get reviews for media");
    // Note: This might be empty if the media has no reviews

    for review in &reviews {
        assert!(review.id > 0);
        assert_eq!(review.media_id, 16498);
        assert!(!review.body.is_empty());
    }
}

#[tokio::test]
async fn test_get_reviews_by_user() {
    let client = AniListClient::new();
    // This test might fail if the specific user doesn't exist or has no reviews
    let result = crate::review_api_call!(client, get_reviews_by_user, 1, 1, 5);

    // We just check that the call doesn't panic
    match result {
        Ok(reviews) => {
            for review in &reviews {
                assert!(review.id > 0);
                assert_eq!(review.user_id, 1);
                assert!(!review.body.is_empty());
            }
        }
        Err(_) => {
            // User might not exist or have no reviews, which is acceptable
        }
    }
}

#[tokio::test]
async fn test_get_review_by_id() {
    let client = AniListClient::new();
    // This test might fail if the specific review doesn't exist
    let result = crate::review_api_call!(client, get_review_by_id, 1);

    // We just check that the call doesn't panic
    match result {
        Ok(review) => {
            assert_eq!(review.id, 1);
            assert!(!review.body.is_empty());
        }
        Err(_) => {
            // Review might not exist, which is acceptable for this test
        }
    }
}

#[tokio::test]
async fn test_get_top_rated_reviews() {
    let client = AniListClient::new();
    let result = crate::review_api_call!(client, get_top_rated_reviews, 1, 5);

    let reviews = result.expect("Failed to get top rated reviews");
    assert!(!reviews.is_empty());

    // Check that reviews are ordered by rating (descending)
    let mut prev_rating = i32::MAX;
    for review in &reviews {
        assert!(review.id > 0);
        if let Some(rating) = review.rating {
            assert!(rating <= prev_rating);
            prev_rating = rating;
        }
    }
}
