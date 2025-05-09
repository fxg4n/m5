use std::str::FromStr;
use tracing::{Level, Subscriber};
use tracing_subscriber::{
    fmt::{self, format::FmtSpan},
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter,
};

pub fn init(app_name: &str, log_level: &str) {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        EnvFilter::new(format!(
            "{}={},tower_http=debug,axum::rejection=trace",
            app_name, log_level
        ))
    });

    let formatting_layer = fmt::layer()
        .with_target(true)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_file(true)
        .with_line_number(true)
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
        .json();

    tracing_subscriber::registry()
        .with(env_filter)
        .with(formatting_layer)
        .init();

    tracing::info!("Logging initialized with level: {}", log_level);
}

pub fn parse_log_level(level: &str) -> Level {
    Level::from_str(level).unwrap_or(Level::INFO)
}

#[derive(Debug)]
pub struct RequestId(String);

impl RequestId {
    pub fn new() -> Self {
        Self(cuid::cuid2())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for RequestId {
    fn default() -> Self {
        Self::new()
    }
}

pub mod macros {
    #[macro_export]
    macro_rules! log_error {
        ($err:expr) => {{
            tracing::error!(
                error.type_name = std::any::type_name_of_val(&$err),
                error.display = %$err,
                error.debug = ?$err,
                "An error occurred"
            );
        }};
        ($err:expr, $message:expr) => {{
            tracing::error!(
                error.type_name = std::any::type_name_of_val(&$err),
                error.display = %$err,
                error.debug = ?$err,
                $message
            );
        }};
    }

    #[macro_export]
    macro_rules! log_request {
        ($req:expr) => {{
            let request_id = $crate::common::logging::RequestId::new();
            tracing::info!(
                request.id = %request_id.as_str(),
                request.method = %$req.method(),
                request.uri = %$req.uri(),
                request.version = ?$req.version(),
                "Incoming request"
            );
            request_id
        }};
    }
}