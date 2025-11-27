//! Runner abstraction used by `tno-core` to build taskvisor tasks from `CreateSpec`.
//!
//! Concrete runners implement this trait and are plugged into the router.
mod error;
pub use error::RunnerError;

mod context;
pub use context::BuildContext;

use taskvisor::TaskRef;
use tno_model::CreateSpec;

/// Generic task runner used by the core layer.
///
/// A runner is responsible for:
/// - deciding whether it can handle a given [`CreateSpec`] (`supports`)
/// - building a concrete [`TaskRef`] that the supervisor can execute (`build_task`)
pub trait Runner: Send + Sync {
    /// Runner name used in logs and diagnostics.
    fn name(&self) -> &'static str;

    /// Returns `true` if this runner can handle the given spec.
    fn supports(&self, spec: &CreateSpec) -> bool;

    /// Build a concrete [`TaskRef`] for the given spec.
    ///
    /// The provided [`BuildContext`] carries shared dependencies injected at router setup time.
    fn build_task(&self, spec: &CreateSpec, ctx: &BuildContext) -> Result<TaskRef, RunnerError>;
}
