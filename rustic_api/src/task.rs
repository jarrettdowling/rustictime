use chrono::{Utc};
use std::fmt;
use std::fmt::{Formatter, Display};

// Task Struct and methods ----------------------------------------------------

#[derive(Debug)]
pub struct Task {
    pub id: i64,
    pub title: String,
    pub priority: u8,
    pub parts: u8,
    pub duedate: String,
    pub description: String,
}

impl Task {

    /// Creates a new task struct as defined above with the given inputs
    /// Requires: duedate follows the RFC2822 standard. I.e. the duedate
    ///            string has the form "Fri, 14 Jul 2017 02:40:00 +0000"
    pub fn new(title: String, priority: u8, parts: u8, duedate: String, description: String) -> Self {
        let timestamp = Utc::now().timestamp_millis();

        Self {
            id: timestamp,
            title: title,
            priority: priority,
            parts: parts,
            duedate: duedate,
            description: description,
        }
    }

    pub fn details(&self) {
        println!("{}", self);
        println!("{}", self.description);
    }
}

impl Display for Task {
    fn fmt (&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "({}: {}, Priority: {} with {} parts, Due: {})",
                self.title, self.id, self.priority, self.parts, self.duedate)
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

/// Takes an unordered vector of tasks and orders them by priority and
/// then by duedate.
fn list_tasks_by_order(unordered: Vec<Task>) -> Vec<Task> {
    println!("Ordering...");
    unordered
}


