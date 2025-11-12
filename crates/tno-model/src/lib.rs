mod api;
mod common;
mod domain;
mod error;
mod kind;
mod spec;
mod strategy;

pub use api::CreateRequest;
pub use error::{ModelError, ModelResult};
pub use kind::TaskKind;
pub use spec::CreateSpec;
pub use strategy::{AdmissionStrategy, BackoffStrategy, JitterStrategy, RestartStrategy};

#[cfg(feature = "schema")]
pub use schemars::{JsonSchema, schema_for};

pub mod prelude {
    pub use crate::{
        AdmissionStrategy, BackoffStrategy, CreateRequest, CreateSpec, JitterStrategy,
        RestartStrategy, TaskKind,
    };
    #[cfg(feature = "schema")]
    pub use schemars::{JsonSchema, schema_for};
}
