use std::sync::Arc;

use taskvisor::{Config as SupervisorConfig, ControllerConfig, Supervisor};
use tracing::{debug, info, instrument};

use crate::{error::CoreError, map::to_controller_spec, router::RunnerRouter};

pub struct SupervisorApi {
    sup: Arc<Supervisor>,
    router: RunnerRouter,
}

impl SupervisorApi {
    #[instrument(level = "info", skip(router))]
    pub async fn new_default(router: RunnerRouter) -> Result<Self, CoreError> {
        let sup = Supervisor::builder(SupervisorConfig::default())
            .with_controller(ControllerConfig::default())
            .build();

        sup.wait_ready().await;
        info!("supervisor is ready");
        Ok(Self { sup, router })
    }

    pub fn supervisor(&self) -> Arc<Supervisor> {
        Arc::clone(&self.sup)
    }

    #[instrument(level = "debug", skip(self, spec), fields(slot = %spec.slot, kind = ?spec.kind))]
    pub async fn submit(&self, spec: &tno_model::CreateSpec) -> Result<(), CoreError> {
        let task  = self.router.build(spec)?;

        debug!("submitting controller spec");
        self.sup
            .submit(to_controller_spec(task, spec))
            .await
            .map_err(|e| CoreError::Supervisor(e.to_string()))
    }
}