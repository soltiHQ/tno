use tno_model::AdmissionStrategy;
// TODO: change to 'AdmissionPolicy' after: https://github.com/soltiHQ/taskvisor/issues/48
use taskvisor::ControllerAdmission as AdmissionPolicy;

pub fn to_admission_policy(s: AdmissionStrategy) -> AdmissionPolicy {
    match s {
        AdmissionStrategy::DropIfRunning => AdmissionPolicy::DropIfRunning,
        AdmissionStrategy::Replace => AdmissionPolicy::Replace,
        AdmissionStrategy::Queue => AdmissionPolicy::Queue,
    }
}
