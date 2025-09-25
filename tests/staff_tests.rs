use anilist_sdk::client::AniListClient;
use chrono::prelude::*;
mod test_utils;

#[tokio::test]
async fn test_get_popular_staff() {
    let client = AniListClient::new();
    let result = crate::staff_api_call!(client, get_popular, 1, 5);

    let staff_list = result.expect("Failed to get popular staff");
    assert!(!staff_list.is_empty());
    assert!(staff_list.len() <= 5);

    // Check that all staff have required fields
    for staff in &staff_list {
        assert!(staff.id > 0);
        assert!(staff.name.is_some());
    }
}

#[tokio::test]
async fn test_get_staff_by_id() {
    let client = AniListClient::new();
    // Using Ikue Ootani's ID (95128)
    let result = crate::staff_api_call!(client, get_by_id, 95128);

    let staff = result.expect("Failed to get staff by ID");
    assert_eq!(staff.id, 95128);
    assert!(staff.name.is_some());
}

#[tokio::test]
async fn test_search_staff() {
    let client = AniListClient::new();
    let result = crate::staff_api_call!(client, search, "Miyazaki", 1, 5);

    let staff_list = result.expect("Failed to search staff");
    assert!(!staff_list.is_empty());

    // Check that results contain "Miyazaki" in some form
    let has_miyazaki = staff_list.iter().any(|staff| {
        if let Some(name) = &staff.name {
            name.full
                .as_ref()
                .is_some_and(|n| n.to_lowercase().contains("miyazaki"))
                || name
                    .last
                    .as_ref()
                    .is_some_and(|n| n.to_lowercase().contains("miyazaki"))
        } else {
            false
        }
    });
    assert!(has_miyazaki);
}

#[tokio::test]
async fn test_get_staff_today_birthday() {
    let client = AniListClient::new();
    let today = Local::now().date_naive();
    let day = today.day() as i32;
    let month = today.month() as i32;
    let result = crate::staff_api_call!(client, get_today_birthday, 1, 10);

    let staff_list = result.expect("Failed to get staff with today's birthday");
    // Note: This might be empty if no staff have this birthday

    for staff in &staff_list {
        assert!(staff.id > 0);
        if let Some(birth_date) = &staff.date_of_birth {
            assert_eq!(birth_date.month, Some(month));
            assert_eq!(birth_date.day, Some(day));
        }
    }
}

#[tokio::test]
async fn test_get_most_favorited_staff() {
    let client = AniListClient::new();
    let result = crate::staff_api_call!(client, get_most_favorited, 1, 5);

    let staff_list = result.expect("Failed to get most favorited staff");
    assert!(!staff_list.is_empty());

    // Check that staff are ordered by favorites (descending)
    let mut prev_favorites = i32::MAX;
    for staff in &staff_list {
        assert!(staff.id > 0);
        if let Some(favourites) = staff.favourites {
            assert!(favourites <= prev_favorites);
            prev_favorites = favourites;
        }
    }
}
