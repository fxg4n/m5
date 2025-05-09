pub mod config;
pub mod bootstrap;
pub mod common;
pub mod application;
pub mod features;
pub mod api;
pub mod infrastructure;

pub use bootstrap::{AppState, init, shutdown_signal};
pub use config::Config;
pub use common::constants;

pub mod prelude {
    pub use crate::common::errors::{AppError, Result};
    pub use crate::common::logging;
    pub use crate::common::validation::ValidateExt;
    pub use crate::common::constants;
}
