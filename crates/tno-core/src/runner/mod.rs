mod error;
mod build;

pub use build::BuildContext;
pub use error::RunnerError;

use tno_model::CreateSpec;
use taskvisor::TaskRef;

pub trait Runner: Send + Sync {
    fn name(&self) -> &'static str;

    fn supports(&self, spec: &CreateSpec) -> bool;

    fn build_task(&self, spec: &CreateSpec, ctx: &BuildContext) -> Result<TaskRef, RunnerError>;
}