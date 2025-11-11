pub mod logger;

#[cfg(feature = "subscriber")]
pub mod subscriber;

pub mod prelude {
    pub use crate::logger::{logger_init, LoggerConfig, LoggerError, LoggerFormat};

    #[cfg(feature = "subscriber")]
    pub use crate::subscriber::Journal;
}