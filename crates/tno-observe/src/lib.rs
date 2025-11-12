pub mod logger;

#[cfg(feature = "subscriber")]
pub mod subscriber;

pub mod prelude {
    pub use crate::logger::{LoggerConfig, LoggerError, LoggerFormat, logger_init};

    #[cfg(feature = "subscriber")]
    pub use crate::subscriber::Journal;
}
