mod build;
mod error;

pub use build::BuildContext;
pub use error::RunnerError;

use taskvisor::TaskRef;
use tno_model::CreateSpec;

pub trait Runner: Send + Sync {
    fn name(&self) -> &'static str;

    fn supports(&self, spec: &CreateSpec) -> bool;

    fn build_task(&self, spec: &CreateSpec, ctx: &BuildContext) -> Result<TaskRef, RunnerError>;
}
