use crate::error::AniListError;
use std::time::Duration;
use tokio::time::sleep;

/// Retry configuration for API calls
#[derive(Debug, Clone)]
pub struct RetryConfig {
    /// Maximum number of retry attempts
    pub max_retries: u32,
    /// Base delay between retries in milliseconds
    pub base_delay_ms: u64,
    /// Whether to use exponential backoff
    pub exponential_backoff: bool,
    /// Maximum delay between retries in milliseconds
    pub max_delay_ms: u64,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            base_delay_ms: 1000,
            exponential_backoff: true,
            max_delay_ms: 30000,
        }
    }
}

/// Retry a future with exponential backoff for rate limit errors
pub async fn retry_with_backoff<F, Fut, T>(
    mut operation: F,
    config: RetryConfig,
) -> Result<T, AniListError>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Result<T, AniListError>>,
{
    let mut attempts = 0;
    let mut delay = config.base_delay_ms;

    loop {
        match operation().await {
            Ok(result) => return Ok(result),
            Err(AniListError::RateLimit { retry_after, .. }) => {
                if attempts >= config.max_retries {
                    return Err(AniListError::RateLimit {
                        limit: 90,
                        remaining: 0,
                        reset_at: 0,
                        retry_after,
                    });
                }

                // Use the Retry-After header if available, otherwise use exponential backoff
                let sleep_duration = if retry_after > 0 {
                    Duration::from_secs(retry_after as u64)
                } else {
                    Duration::from_millis(delay.min(config.max_delay_ms))
                };

                println!(
                    "Rate limited. Retrying in {} seconds... (attempt {}/{})",
                    sleep_duration.as_secs(),
                    attempts + 1,
                    config.max_retries
                );

                sleep(sleep_duration).await;

                attempts += 1;
                if config.exponential_backoff {
                    delay = (delay * 2).min(config.max_delay_ms);
                }
            }
            Err(AniListError::RateLimitSimple) => {
                if attempts >= config.max_retries {
                    return Err(AniListError::RateLimitSimple);
                }

                let sleep_duration = Duration::from_millis(delay.min(config.max_delay_ms));
                println!(
                    "Rate limited. Retrying in {} seconds... (attempt {}/{})",
                    sleep_duration.as_secs(),
                    attempts + 1,
                    config.max_retries
                );

                sleep(sleep_duration).await;

                attempts += 1;
                if config.exponential_backoff {
                    delay = (delay * 2).min(config.max_delay_ms);
                }
            }
            Err(AniListError::BurstLimit) => {
                if attempts >= config.max_retries {
                    return Err(AniListError::BurstLimit);
                }

                // For burst limits, wait a bit longer
                let sleep_duration = Duration::from_millis((delay * 2).min(config.max_delay_ms));
                println!(
                    "Burst limit exceeded. Retrying in {} seconds... (attempt {}/{})",
                    sleep_duration.as_secs(),
                    attempts + 1,
                    config.max_retries
                );

                sleep(sleep_duration).await;

                attempts += 1;
                delay = (delay * 2).min(config.max_delay_ms);
            }
            Err(other_error) => return Err(other_error),
        }
    }
}

/// Helper to add delay between requests to avoid rate limiting
pub async fn rate_limit_delay(delay_ms: u64) {
    sleep(Duration::from_millis(delay_ms)).await;
}

/// Calculate appropriate delay based on remaining rate limit
pub fn calculate_delay(remaining: u32, reset_in_seconds: u64) -> Duration {
    if remaining == 0 {
        Duration::from_secs(reset_in_seconds)
    } else if remaining < 10 {
        Duration::from_millis(2000) // 2 seconds when getting low
    } else if remaining < 30 {
        Duration::from_millis(1000) // 1 second when moderate
    } else {
        Duration::from_millis(500) // 500ms when plenty remaining
    }
}
