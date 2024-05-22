use chrono::{Local, Utc};
use rusqlite::{params, Connection, Result};


// Task Struct and methods ----------------------------------------------------

#[derive(Debug)]
struct Task {
    id: i64,
    title: String,
    priority: u8,
}

impl Task {
    fn new(title: String, priority: u8) -> Self {
        let timestamp = Utc::now().timestamp_millis();

        Self {
            id: timestamp,
            title: title,
            priority: priority,
        }
    }
}

// Database functions ---------------------------------------------------------

fn create_db_record(conn: &mut Connection, task_to_add: &Task) -> Result<()> {
    
    // make a transaction from the db connection
    let tx = conn.transaction()?;

    // collect the fields from the task struct
    let id = task_to_add.id;
    let title = task_to_add.title.clone();
    let priority = task_to_add.priority;

    // send the sql command to insert the record
    tx.execute("INSERT INTO tasks (id, title, priority) values (?1, ?2, ?3)",
    &[&id.to_string(), &title, &priority.to_string()])?;

    // commit the change
    tx.commit()
}

fn fetch_priority_n_records(conn: &Connection, priority: u8) -> Result<()> {
    
    // prepare sql statement
    let mut stmt = conn.prepare(
        "SELECT * from tasks WHERE priority=(?1)", priority.to_string())?;
    
    let tasks_iter = stmt.query_map((), |row| {
        Ok(Task {
            id: row.get(0)?,
            title: row.get(1)?,
            priority: row.get(2)?,
        })
    })?;

    for task in tasks {
        println!("Found Task: {:?} where priority is 0", task.unwrap().id);
    }
    
    Ok(())
}

// Task ordering logic --------------------------------------------------------

// logic to find hours until date

// let a = DateTime::parse_from_str("31.5.2024 8:00 am +0000",
            // "%d.%m.%Y %H:%M %P %z").unwrap();
// let b: DateTime<FixedOffset> = Utc::now().into();
// let diff = a - b;
// println!("The difference between {} and {} is: {} hours",
//     a, b, diff.num_hours());

fn main() -> Result<()> {

    // establishing the connection to the database
    let mut conn = Connection::open("rustic.db")?;

    // create table if none exist
    conn.execute(
        "create table if not exists tasks (
            id integer primary key,
            title text not null,
            priority integer not null)"
        , (),)?;

    // Inserting test task
    // let task1 = Task::new("Test Title 3".to_string(), 0);
    // create_db_record(&mut conn, &task1)?;

    // Selecting test task
    let priority = 1;
    fetch_priority_n_records(&conn, priority)?;

    Ok(())
}
