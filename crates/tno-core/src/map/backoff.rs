use std::time::Duration;

use tno_model::BackoffStrategy;
use taskvisor::BackoffPolicy;

use super::to_jitter_policy;

pub fn to_backoff_policy(s: &BackoffStrategy) -> BackoffPolicy {
    BackoffPolicy {
        success_delay: s.delay_ms.map(Duration::from_millis),
        first: Duration::from_millis(s.first_ms),
        max: Duration::from_millis(s.max_ms),
        jitter: to_jitter_policy(s.jitter),
        factor: s.factor,
    }
}