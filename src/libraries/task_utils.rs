use serde::{Deserialize, Serialize};
extern crate chrono;
use chrono::{DateTime, Duration, Utc};

struct Session {
    start_time: DateTime<Utc>,
    end_time: Option<DateTime<Utc>>,
    duration: Option<Duration>,
}

impl Session {
    fn new(start: DateTime<Utc>) -> Session {
        Session {
            start_time: start,
            end_time: None,
            duration: None,
        }
    }

    // Method to set the end time and calculate duration
    fn end_session(&mut self, end: DateTime<Utc>) -> Result<(), String> {
        if self.end_time.is_none() {
            self.end_time = Some(end);
            self.duration = Some(end.signed_duration_since(self.start_time));
            Ok(())
        } else {
            Err("End time has already been set".to_string())
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct Task {
    title: String,
}
