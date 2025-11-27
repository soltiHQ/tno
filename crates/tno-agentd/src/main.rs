use std::{sync::Arc, time::Duration};

use tracing::info;

// tno-core: high-level API вокруг taskvisor
use tno_core::{RunnerRouter, SupervisorApi, TaskPolicy};

// tno-exec: раннер для Subprocess
use tno_exec::subprocess::SubprocessRunner;

// tno-observe: логгер + internal timezone-задача
use tno_observe::{init_logger, LoggerConfig, LoggerLevel, Subscriber, timezone_sync};

use taskvisor::{ControllerConfig, SupervisorConfig, Subscribe};

use tno_model::{
    AdmissionStrategy, BackoffStrategy, CreateSpec, Env, Flag, JitterStrategy,
    RestartStrategy, TaskKind,
};

#[tokio::main(flavor = "multi_thread")]
async fn main() -> anyhow::Result<()> {
    // 1) Логгер
    let mut cfg = LoggerConfig::default();
    cfg.level = LoggerLevel::new("debug")?;
    init_logger(&cfg)?;
    info!("logger initialized");

    // 2) Подписчики на события
    let subscribers: Vec<Arc<dyn Subscribe>> = vec![Arc::new(Subscriber::default())];

    // 3) Роутер + регистрация subprocess-runner
    let mut router = RunnerRouter::new();
    router.register(Arc::new(SubprocessRunner::new()));

    // 4) Поднимаем SupervisorApi
    let api = SupervisorApi::new(
        SupervisorConfig::default(),
        ControllerConfig::default(),
        subscribers,
        router,
    )
        .await?;

    // 5) Internal timezone-sync задача через submit_with_task
    let (tz_task, tz_spec) = timezone_sync();
    let tz_policy = TaskPolicy::from_spec(&tz_spec);
    api.submit_with_task(tz_task, &tz_policy).await?;

    // 6) Обычная задача: `ls /tmp` через CreateSpec + TaskKind::Subprocess
    let ls_spec = CreateSpec {
        slot: "demo-ls-tmp".to_string(),
        kind: TaskKind::Subprocess {
            command: "ls".into(),
            args: vec!["/tmp".into()],
            env: Env::default(),
            cwd: None,
            fail_on_non_zero: Flag::enabled(),
        },
        timeout_ms: 5_000,
        restart: RestartStrategy::Never,
        backoff: BackoffStrategy {
            jitter: JitterStrategy::None,
            first_ms: 0,
            max_ms: 0,
            factor: 1.0,
        },
        admission: AdmissionStrategy::DropIfRunning,
    };

    api.submit(&ls_spec).await?;

    // Небольшая пауза, чтобы увидеть вывод/логи
    tokio::time::sleep(Duration::from_secs(5)).await;

    Ok(())
}