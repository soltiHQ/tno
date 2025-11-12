use taskvisor::JitterPolicy;
use tno_model::JitterStrategy;

pub fn to_jitter_policy(s: JitterStrategy) -> JitterPolicy {
    match s {
        JitterStrategy::Decorrelated => JitterPolicy::Decorrelated,
        JitterStrategy::Equal => JitterPolicy::Equal,
        JitterStrategy::Full => JitterPolicy::Full,
        JitterStrategy::None => JitterPolicy::None,
    }
}
