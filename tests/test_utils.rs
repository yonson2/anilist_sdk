use anilist_sdk::error::AniListError;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::time::{Duration, sleep};

/// Global rate limiter to coordinate between all tests
static LAST_REQUEST_TIME: AtomicU64 = AtomicU64::new(0);

/// Standard rate limiting delay between API requests
const RATE_LIMIT_DELAY_MS: u64 = 1500;

/// Smart rate limiting for tests that prevents hitting AniList's rate limits
pub async fn rate_limit() {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;

    let last_request = LAST_REQUEST_TIME.load(Ordering::Relaxed);
    let time_since_last = now.saturating_sub(last_request);

    if time_since_last < RATE_LIMIT_DELAY_MS {
        let sleep_duration = RATE_LIMIT_DELAY_MS - time_since_last;
        sleep(Duration::from_millis(sleep_duration)).await;
    }

    LAST_REQUEST_TIME.store(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64,
        Ordering::Relaxed,
    );
}

/// Execute a request with automatic retry on rate limit errors
pub async fn with_retry<T>(
    operation: impl Fn() -> std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<T, AniListError>> + Send>,
    >,
) -> Result<T, AniListError> {
    const MAX_RETRIES: u32 = 3;
    let mut attempts = 0;

    loop {
        attempts += 1;

        match operation().await {
            Ok(result) => return Ok(result),
            Err(AniListError::RateLimit {
                retry_after,
                reset_at,
                ..
            }) => {
                if attempts >= MAX_RETRIES {
                    return Err(AniListError::RateLimit {
                        limit: 90,
                        remaining: 0,
                        reset_at,
                        retry_after,
                    });
                }

                println!(
                    "Rate limited (attempt {}/{}), waiting {} seconds before retry...",
                    attempts, MAX_RETRIES, retry_after
                );

                // Wait the specified retry_after duration plus a small buffer
                sleep(Duration::from_secs(retry_after as u64 + 1)).await;
            }
            Err(AniListError::RateLimitSimple) => {
                if attempts >= MAX_RETRIES {
                    return Err(AniListError::RateLimitSimple);
                }

                println!(
                    "Rate limited (attempt {}/{}), waiting 60 seconds before retry...",
                    attempts, MAX_RETRIES
                );

                // Conservative wait time when we don't have specific timing
                sleep(Duration::from_secs(60)).await;
            }
            Err(AniListError::BurstLimit) => {
                if attempts >= MAX_RETRIES {
                    return Err(AniListError::BurstLimit);
                }

                println!(
                    "Burst limit hit (attempt {}/{}), waiting 5 seconds before retry...",
                    attempts, MAX_RETRIES
                );

                // Shorter wait for burst limits
                sleep(Duration::from_secs(5)).await;
            }
            Err(other_error) => return Err(other_error),
        }
    }
}

/// Macro to simplify running API calls with rate limiting and retry logic
#[macro_export]
macro_rules! api_call {
    ($client:expr, $method:ident, $($args:expr),* $(,)?) => {{
        use $crate::test_utils::{rate_limit, with_retry};

        rate_limit().await;
        let result = with_retry(|| {
            let client = $client.clone();
            Box::pin(async move { client.$method($($args),*).await })
        }).await;
        rate_limit().await;
        result
    }};
}

/// Macro for character API calls
#[macro_export]
macro_rules! character_api_call {
    ($client:expr, $method:ident, $($args:expr),* $(,)?) => {{
        use $crate::test_utils::{rate_limit, with_retry};

        rate_limit().await;
        let result = with_retry(|| {
            let client = $client.clone();
            Box::pin(async move { client.character().$method($($args),*).await })
        }).await;
        rate_limit().await;
        result
    }};
}

/// Macro for anime API calls
#[macro_export]
macro_rules! anime_api_call {
    ($client:expr, $method:ident, $($args:expr),* $(,)?) => {{
        use $crate::test_utils::{rate_limit, with_retry};

        rate_limit().await;
        let result = with_retry(|| {
            let client = $client.clone();
            Box::pin(async move { client.anime().$method($($args),*).await })
        }).await;
        rate_limit().await;
        result
    }};
}

/// Macro for user API calls
#[macro_export]
macro_rules! user_api_call {
    ($client:expr, $method:ident) => {{
        use $crate::test_utils::{rate_limit, with_retry};

        rate_limit().await;
        let result = with_retry(|| {
            let client = $client.clone();
            Box::pin(async move { client.user().$method().await })
        }).await;
        rate_limit().await;
        result
    }};
    ($client:expr, $method:ident, $($args:expr),* $(,)?) => {{
        use $crate::test_utils::{rate_limit, with_retry};

        rate_limit().await;
        let result = with_retry(|| {
            let client = $client.clone();
            Box::pin(async move { client.user().$method($($args),*).await })
        }).await;
        rate_limit().await;
        result
    }};
}

/// Macro for airing API calls
#[macro_export]
macro_rules! airing_api_call {
    ($client:expr, $method:ident, $($args:expr),* $(,)?) => {{
        use $crate::test_utils::{rate_limit, with_retry};

        rate_limit().await;
        let result = with_retry(|| {
            let client = $client.clone();
            Box::pin(async move { client.airing().$method($($args),*).await })
        }).await;
        rate_limit().await;
        result
    }};
}

/// Macro for staff API calls
#[macro_export]
macro_rules! staff_api_call {
    ($client:expr, $method:ident, $($args:expr),* $(,)?) => {{
        use $crate::test_utils::{rate_limit, with_retry};

        rate_limit().await;
        let result = with_retry(|| {
            let client = $client.clone();
            Box::pin(async move { client.staff().$method($($args),*).await })
        }).await;
        rate_limit().await;
        result
    }};
}

/// Macro for studio API calls
#[macro_export]
macro_rules! studio_api_call {
    ($client:expr, $method:ident, $($args:expr),* $(,)?) => {{
        use $crate::test_utils::{rate_limit, with_retry};

        rate_limit().await;
        let result = with_retry(|| {
            let client = $client.clone();
            Box::pin(async move { client.studio().$method($($args),*).await })
        }).await;
        rate_limit().await;
        result
    }};
}

/// Macro for manga API calls
#[macro_export]
macro_rules! manga_api_call {
    ($client:expr, $method:ident, $($args:expr),* $(,)?) => {{
        use $crate::test_utils::{rate_limit, with_retry};

        rate_limit().await;
        let result = with_retry(|| {
            let client = $client.clone();
            Box::pin(async move { client.manga().$method($($args),*).await })
        }).await;
        rate_limit().await;
        result
    }};
}

/// Macro for activity API calls
#[macro_export]
macro_rules! activity_api_call {
    ($client:expr, $method:ident, $($args:expr),* $(,)?) => {{
        use $crate::test_utils::{rate_limit, with_retry};

        rate_limit().await;
        let result = with_retry(|| {
            let client = $client.clone();
            Box::pin(async move { client.activity().$method($($args),*).await })
        }).await;
        rate_limit().await;
        result
    }};
}

/// Macro for forum API calls
#[macro_export]
macro_rules! forum_api_call {
    ($client:expr, $method:ident, $($args:expr),* $(,)?) => {{
        use $crate::test_utils::{rate_limit, with_retry};

        rate_limit().await;
        let result = with_retry(|| {
            let client = $client.clone();
            Box::pin(async move { client.forum().$method($($args),*).await })
        }).await;
        rate_limit().await;
        result
    }};
}

/// Macro for review API calls
#[macro_export]
macro_rules! review_api_call {
    ($client:expr, $method:ident, $($args:expr),* $(,)?) => {{
        use $crate::test_utils::{rate_limit, with_retry};

        rate_limit().await;
        let result = with_retry(|| {
            let client = $client.clone();
            Box::pin(async move { client.review().$method($($args),*).await })
        }).await;
        rate_limit().await;
        result
    }};
}

/// Macro for recommendation API calls
#[macro_export]
macro_rules! recommendation_api_call {
    ($client:expr, $method:ident, $($args:expr),* $(,)?) => {{
        use $crate::test_utils::{rate_limit, with_retry};

        rate_limit().await;
        let result = with_retry(|| {
            let client = $client.clone();
            Box::pin(async move { client.recommendation().$method($($args),*).await })
        }).await;
        rate_limit().await;
        result
    }};
}

/// Macro for notification API calls
#[macro_export]
macro_rules! notification_api_call {
    ($client:expr, $method:ident) => {{
        use $crate::test_utils::{rate_limit, with_retry};

        rate_limit().await;
        let result = with_retry(|| {
            let client = $client.clone();
            Box::pin(async move { client.notification().$method().await })
        }).await;
        rate_limit().await;
        result
    }};
    ($client:expr, $method:ident, $($args:expr),* $(,)?) => {{
        use $crate::test_utils::{rate_limit, with_retry};

        rate_limit().await;
        let result = with_retry(|| {
            let client = $client.clone();
            Box::pin(async move { client.notification().$method($($args),*).await })
        }).await;
        rate_limit().await;
        result
    }};
}
