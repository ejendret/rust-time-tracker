use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Session {
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    duration: i64,
}

impl Session {
    pub fn new(start: DateTime<Utc>) -> Session {
        Session {
            start_time: start,
            end_time: start,
            duration: 0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Task {
    pub title: String,
    pub sessions: Vec<Session>,
    pub total_duration: i64,
}

impl Task {
    pub fn new(title: String) -> Task {
        Task {
            title: title,
            sessions: Vec::new(),
            total_duration: 0,
        }
    }

    pub fn start_session(&mut self) {
        // Proceed only if there are no sessions or the last session is complete
        let length = self.sessions.len();
        if length == 0 || self.sessions[length - 1].duration != 0 {
            let new_session = Session::new(Utc::now());
            self.sessions.push(new_session);
        } else {
            println!("Cannot start a session while another is ongoing.");
        }
    }

    pub fn end_session(&mut self, end: DateTime<Utc>) {
        let length = self.sessions.len();
        if length > 0 && self.sessions[length - 1].duration == 0 {
            self.sessions[length - 1].end_time = end;
            self.sessions[length - 1].duration =
                (end - self.sessions[length - 1].start_time).num_seconds();
        } else {
            println!("No ongoing session to end.");
        }
    }
}
