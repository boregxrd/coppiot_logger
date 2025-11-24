#[cfg(test)]
mod tests {
    use coppiot_rust_logger::{init_logging, LogFormat};
    use std::sync::Once;
    use tracing::{debug, error, info, warn};

    static INIT: Once = Once::new();

    fn setup() {
        INIT.call_once(|| {
            init_logging(LogFormat::Human);
        });
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_logging_output() {
        setup();

        println!("\n=== Testing Human Format ===\n");

        // Basic log
        info!(
            service = "service_name",
            function = "function_name",
            message = "Starting service creation"
        );

        // With variables
        let customer_id = "789021";

        info!(
            service = "service_name",
            function = "function_name",
            message = format!("Creating service for customer with id {}", customer_id)
        );

        // Success message (should be green)
        info!(
            service = "service_name",
            function = "function_name",
            message = "Service created successfully"
        );

        // Warning
        warn!(
            service = "service_name",
            function = "function_name",
            message = "Connection pool running low"
        );

        // Error
        error!(
            service = "service_name",
            function = "function_name",
            message = "Failed to authenticate user"
        );

        // Debug (won't show unless RUST_LOG=debug)
        debug!(
            service = "service_name",
            function = "function_name",
            message = "Cache miss for key user:123"
        );

        // Test with missing fields
        info!(message = "Log with only message field");

        println!("\n=== Tests Complete ===\n");
    }
}
