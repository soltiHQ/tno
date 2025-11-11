use tno_model::RestartStrategy;
use taskvisor::RestartPolicy;

pub fn to_restart_policy(s: RestartStrategy) -> RestartPolicy {
    match s {
        RestartStrategy::OnFailure => RestartPolicy::OnFailure,
        RestartStrategy::Always => RestartPolicy::Always,
        RestartStrategy::Never => RestartPolicy::Never,
    }
}