use chrono::{Utc};
use std::fmt;
use std::fmt::{Formatter, Display};

// Task Struct and methods ----------------------------------------------------

#[derive(Debug)]
pub struct Task {
    pub id: i64,
    pub title: String,
    pub priority: u8,
}

impl Task {
    pub fn new(title: String, priority: u8) -> Self {
        let timestamp = Utc::now().timestamp_millis();

        Self {
            id: timestamp,
            title: title,
            priority: priority,
        }
    }
}

impl Display for Task {
    fn fmt (&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "({}: {}, Priority: {})", self.title, self.id, self.priority)
    }
}

// Task ordering logic --------------------------------------------------------

// logic to find hours until date

// let a = DateTime::parse_from_str("31.5.2024 8:00 am +0000",
            // "%d.%m.%Y %H:%M %P %z").unwrap();
// let b: DateTime<FixedOffset> = Utc::now().into();
// let diff = a - b;
// println!("The difference between {} and {} is: {} hours",
//     a, b, diff.num_hours());

