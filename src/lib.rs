use std::fmt;
use tracing::{Event, Subscriber};
use tracing_subscriber::fmt::{format::Writer, FmtContext, FormatEvent, FormatFields};
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::filter::{EnvFilter, LevelFilter};
use chrono::Local;

/// Logging format options
#[allow(dead_code)]
pub enum LogFormat {
    /// best for local development & easy to read in CloudWatch for humans
    Human,
    /// Rust's pretty format with full metadata (for debugging)
    Debug,
    /// JSON format (best for production and easy to read by machines or AI)
    Json,
}

/// Initialize the logging system with the specified format
pub fn init_logging(format: LogFormat) {
    let filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();

    match format {
        LogFormat::Human => {
            // Human-style format: [SERVICE][FUNCTION][TIMESTAMP] Message
            tracing_subscriber::fmt()
                .with_env_filter(filter)
                .event_format(HumanStyleFormatter)
                .init();
        }
        LogFormat::Debug => {
            // Pretty format with full metadata (for debugging)
            tracing_subscriber::fmt()
                .with_env_filter(filter)
                .with_ansi(true)
                .with_line_number(true)
                .with_file(true)
                .pretty()
                .init();
        }
        LogFormat::Json => {
            // JSON format (for production)
            tracing_subscriber::fmt()
                .with_env_filter(filter)
                .with_ansi(false)
                .json()
                .init();
        }
    }
}

/// Custom formatter for [SERVICE][FUNCTION][TIMESTAMP] Message
pub struct HumanStyleFormatter;

impl<S, N> FormatEvent<S, N> for HumanStyleFormatter
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        _ctx: &FmtContext<'_, S, N>,
        mut writer: Writer<'_>,
        event: &Event<'_>,
    ) -> fmt::Result {
        // Get the log level
        let level = *event.metadata().level();
        
        // Colors
        let error_color = "\x1b[31m"; // Red
        let warn_color = "\x1b[33m";  // Yellow
        let info_color = "\x1b[34m";  // Blue
        let success_color = "\x1b[32m"; // Green
        let reset = "\x1b[0m";
        let gray = "\x1b[37m"; // Gray for metadata

        // Extract service, function, and message from the event's fields
        let mut service = String::new();
        let mut function = String::new();
        let mut message = String::new();

        // Visit all fields to extract our custom fields
        let mut visitor = HumanStyleVisitor {
            service: &mut service,
            function: &mut function,
            message: &mut message,
        };

        event.record(&mut visitor);

        // Use defaults if fields are missing
        if service.is_empty() {
            service = "unknown".to_string();
        }
        if function.is_empty() {
            function = "unknown".to_string();
        }
        if message.is_empty() {
            message = "<no message>".to_string();
        }

        // Format timestamp
        let timestamp = Local::now().format("%b %d %H:%M:%S");

        // Determine message color based on content or level
        let msg_color = if message.to_lowercase().contains("succe") // for success or succeeded
            || message.to_lowercase().contains("created")
            || message.to_lowercase().contains("complete")
            || message.to_lowercase().contains("finished")
            || message.to_lowercase().contains("done")
            || message.to_lowercase().contains("ready")
            || message.to_lowercase().contains("initialized")
            || message.to_lowercase().contains("connected")
            || message.to_lowercase().contains("deployed")
            || message.to_lowercase().contains("started") {
            success_color
        } else {
            match level {
                tracing::Level::ERROR => error_color,
                tracing::Level::WARN => warn_color,
                _ => info_color,
            }
        };

        // Write in Human style format: [SERVICE][FUNCTION][TIMESTAMP] Message
        write!(
            writer,
            "{gray}[{service}][{function}][{timestamp}]{reset}{msg_color} {message}{reset}",
            gray = gray,
            service = service.to_uppercase(),
            function = function,
            timestamp = timestamp,
            reset = reset,
            msg_color = msg_color,
            message = message,
        )?;

        writeln!(writer)
    }
}

/// Visitor to extract custom fields from the event
struct HumanStyleVisitor<'a> {
    service: &'a mut String,
    function: &'a mut String,
    message: &'a mut String,
}

impl<'a> tracing::field::Visit for HumanStyleVisitor<'a> {
    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn fmt::Debug) {
        match field.name() {
            "service" => {
                *self.service = format!("{:?}", value).trim_matches('"').to_string();
            }
            "function" => {
                *self.function = format!("{:?}", value).trim_matches('"').to_string();
            }
            "message" => {
                // This is the actual log message - only use the first one
                if self.message.is_empty() {
                    *self.message = format!("{:?}", value).trim_matches('"').to_string();
                }
            }
            _ => {
                // Skip debug, environment, and other metadata fields
                // We only want service, function, and the message
            }
        }
    }

    fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
        match field.name() {
            "service" => *self.service = value.to_string(),
            "function" => *self.function = value.to_string(),
            "message" => {
                // This is the actual log message - only use the first one
                if self.message.is_empty() {
                    *self.message = value.to_string();
                }
            }
            _ => {
                // Skip other fields
            }
        }
    }
}
