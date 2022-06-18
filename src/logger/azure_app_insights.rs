use std::time::Duration;
use appinsights::telemetry::SeverityLevel;
use appinsights::{TelemetryClient, TelemetryConfig};

use super::Logger;

pub struct AzureAppInsights {
    client: TelemetryClient,
}

impl AzureAppInsights {
    pub fn new(key: String) -> Self {
        let config = TelemetryConfig::builder()
            .i_key(key)
            .interval(Duration::from_secs(5))
            .build();

        let client = TelemetryClient::from_config(config);

        Self { client }
    }
}

impl Logger for AzureAppInsights {
    fn information(&self, message: String) {
        self.client.track_trace(message, SeverityLevel::Information);
    }

    fn warning(&self, message: String) {
        self.client.track_trace(message, SeverityLevel::Warning);
    }

    fn error(&self, message: String) {
        self.client.track_trace(message, SeverityLevel::Error);
    }
}
