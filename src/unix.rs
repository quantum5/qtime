use std::time::SystemTime;

pub async fn unix() -> String {
    format!("{}\n", SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs())
}

pub async fn unix_dec() -> String {
    format!("{}\n", SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs_f64())
}

pub async fn unix_ms() -> String {
    format!("{}\n", SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis())
}