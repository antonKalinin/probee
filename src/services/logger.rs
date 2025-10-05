use log::kv::{Key, Value, Visitor};
use log::{Level, Metadata, Record};
use tokio::sync::OnceCell;

use std::env;
use std::time::Duration;

static COUNTRY_CODE: OnceCell<String> = OnceCell::const_new();

pub struct Logger {
    min_level: Level,
}

impl Logger {
    pub fn new() -> anyhow::Result<Self> {
        let min_level = Level::Info;

        Ok(Logger { min_level })
    }

    pub fn with_level(mut self, level: Level) -> Self {
        self.min_level = level;
        self
    }
}

fn level_to_severity_number(level: Level) -> u32 {
    match level {
        Level::Error => 17, // SEVERITY_NUMBER_ERROR
        Level::Warn => 13,  // SEVERITY_NUMBER_WARN
        Level::Info => 9,   // SEVERITY_NUMBER_INFO
        Level::Debug => 5,  // SEVERITY_NUMBER_DEBUG
        Level::Trace => 1,  // SEVERITY_NUMBER_TRACE
    }
}

/// Visitor to collect key-value pairs from log records
struct KeyValueCollector {
    attributes: Vec<(String, String)>,
}

impl KeyValueCollector {
    fn new() -> Self {
        Self {
            attributes: Vec::new(),
        }
    }
}

impl<'kvs> Visitor<'kvs> for KeyValueCollector {
    fn visit_pair(&mut self, key: Key<'kvs>, value: Value<'kvs>) -> Result<(), log::kv::Error> {
        let key_str = key.as_str().to_string();
        let value_str = value.to_string();
        self.attributes.push((key_str, value_str));
        Ok(())
    }
}

impl log::Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.min_level
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        // Send to BetterStack asynchronously if configured
        let record_clone = LogRecordData {
            level: record.level(),
            args: record.args().to_string(),
            target: record.target().to_string(),
            file: record.file().map(|s| s.to_string()),
            line: record.line(),
            module_path: record.module_path().map(|s| s.to_string()),
            key_values: {
                let mut kv_collector = KeyValueCollector::new();
                let _ = record.key_values().visit(&mut kv_collector);
                kv_collector.attributes
            },
        };

        // Spawn async task to send log
        tokio::spawn(async move {
            send_log_to_betterstack(record_clone).await;
        });
    }

    fn flush(&self) {
        // For async implementation, we would need to wait for pending sends
        // This is a simplified implementation
    }
}

#[derive(Clone)]
struct LogRecordData {
    level: Level,
    args: String,
    target: String,
    file: Option<String>,
    line: Option<u32>,
    module_path: Option<String>,
    key_values: Vec<(String, String)>,
}

async fn send_log_to_betterstack(record: LogRecordData) {
    let endpoint = match env!("BETTERSTACK_INGESTING_HOST") {
        host if !host.is_empty() => format!("https://{}/v1/logs", host),
        _ => return,
    };

    let token = match env!("BETTERSTACK_SOURCE_TOKEN") {
        t if !t.is_empty() => t,
        _ => return,
    };

    // Create OTLP log payload
    let mut attributes = vec![serde_json::json!({
        "key": "target",
        "value": { "stringValue": record.target }
    })];

    if let Some(file) = record.file {
        attributes.push(serde_json::json!({
            "key": "source.file",
            "value": { "stringValue": file }
        }));
    }

    if let Some(line) = record.line {
        attributes.push(serde_json::json!({
            "key": "source.line",
            "value": { "intValue": line }
        }));
    }

    if let Some(module_path) = record.module_path {
        attributes.push(serde_json::json!({
            "key": "source.module",
            "value": { "stringValue": module_path }
        }));
    }

    // Add key-value pairs from log record
    for (key, value) in record.key_values {
        attributes.push(serde_json::json!({
            "key": key,
            "value": { "stringValue": value }
        }));
    }

    let country_code = get_country_code().await;
    let macos_version = get_macos_version();

    let log_payload = serde_json::json!({
        "resourceLogs": [{
            "resource": {
                "attributes": [
                    {
                    "key": "app.name",
                    "value": { "stringValue": "probee" }
                    },
                    {
                    "key": "app.version",
                    "value": { "stringValue": env!("CARGO_PKG_VERSION") }
                    },
                    {
                        "key": "user.country_code",
                        "value": { "stringValue": country_code }
                    },
                    {
                        "key": "os.name",
                        "value": { "stringValue": "macOS" }
                    },
                    {
                        "key": "os.version",
                        "value": { "stringValue": macos_version.unwrap_or_default() }
                    }
                ]
            },
            "scopeLogs": [{
                "scope": {
                    "name": "probee"
                },
                "logRecords": [{
                    "timeUnixNano": format!("{}",
                        std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap()
                            .as_nanos()
                    ),
                    "severityNumber": level_to_severity_number(record.level),
                    "severityText": record.level.to_string(),
                    "body": {
                        "stringValue": record.args
                    },
                    "attributes": attributes
                }]
            }]
        }]
    });

    // Send to BetterStack
    let client = reqwest::Client::new();
    let result = client
        .post(&endpoint)
        .header("Authorization", format!("Bearer {}", token))
        .header("Content-Type", "application/json")
        .json(&log_payload)
        .send()
        .await;

    if let Err(e) = result {
        eprintln!("Failed to send log to BetterStack: {:?}", e);
    }
}

async fn fetch_country_code() -> Option<String> {
    let url = "https://ipapi.co/country/";
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(1))
        .build()
        .ok()?;

    if let Ok(response) = client.get(url).send().await {
        if let Ok(country) = response.text().await {
            let trimmed = country.trim();

            if trimmed.len() == 2 && trimmed.chars().all(|c| c.is_ascii_alphabetic()) {
                return Some(trimmed.to_uppercase());
            }
        }
    }

    None
}

async fn get_country_code() -> String {
    COUNTRY_CODE
        .get_or_init(|| async { fetch_country_code().await.unwrap_or_default() })
        .await
        .clone()
}

fn get_macos_version() -> Option<String> {
    use std::process::Command;

    Command::new("sw_vers")
        .arg("-productVersion")
        .output()
        .ok()
        .and_then(|output| {
            String::from_utf8(output.stdout)
                .ok()
                .map(|s| s.trim().to_string())
        })
}
