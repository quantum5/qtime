use chrono::{DateTime, SecondsFormat, Utc};
use std::time::SystemTime;

fn chrono_now() -> DateTime<Utc> {
    SystemTime::now().into()
}

pub async fn iso8601() -> String {
    format!("{}\n", chrono_now().to_rfc3339_opts(SecondsFormat::Secs, true))
}

pub async fn iso8601_ms() -> String {
    format!("{}\n", chrono_now().to_rfc3339_opts(SecondsFormat::Millis, true))
}
