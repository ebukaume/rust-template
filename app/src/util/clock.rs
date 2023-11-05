use chrono::{DateTime, Utc};

pub trait Clock: Send + Sync + 'static {
    fn now(&self) -> DateTime<Utc>;
}

#[derive(Clone)]
pub struct SystemClock;

impl SystemClock {
    pub fn new() -> Self {
        SystemClock {}
    }
}

impl Clock for SystemClock {
    fn now(&self) -> DateTime<Utc> {
        Utc::now()
    }
}
