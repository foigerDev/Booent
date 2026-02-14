use std::time::{SystemTime, UNIX_EPOCH};

pub fn current_unix_time() -> usize {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() as usize
}
