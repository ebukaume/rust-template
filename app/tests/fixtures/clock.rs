use app::util::Clock;
use chrono::{DateTime, Utc};

#[derive(Clone)]
pub struct MockClock {
    time: String,
}

impl MockClock {
    pub fn with_frozen_time(time: String) -> Self {
        MockClock { time }
    }
}

impl Clock for MockClock {
    fn now(&self) -> DateTime<Utc> {
        DateTime::parse_from_rfc3339(&self.time)
            .unwrap()
            .with_timezone(&Utc)
    }
}
