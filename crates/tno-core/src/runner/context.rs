use std::fmt;

use tno_model::Env;

/// Shared build context passed to all runners.
#[derive(Debug, Clone)]
pub struct BuildContext {
    env: Env,
}

impl BuildContext {
    /// Create a new build context with the given params.
    pub fn new(env: Env) -> Self {
        Self { env }
    }

    /// Get a reference to the shared environment.
    pub fn env(&self) -> &Env {
        &self.env
    }

    /// Replace the environment and return updated context.
    pub fn with_env(mut self, env: Env) -> Self {
        self.env = env;
        self
    }
}

impl Default for BuildContext {
    fn default() -> Self {
        Self { env: Env::default() }
    }
}

impl fmt::Display for BuildContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BuildContext(env_len={})", self.env.len())
    }
}
