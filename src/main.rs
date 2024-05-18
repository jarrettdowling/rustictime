use chrono::{Local, Utc};
use rusqlite::{params, Connection, Result};

#[derive(Debug)]
struct Task {
    id: i64,
    title: String,
}

fn create_db_record(conn: &Connection, task_to_add: &Task) {
    println!("Creating new record...");
    println!("Inserting Task: {:?}", task_to_add);
    let id = task_to_add.id;
    let title = task_to_add.title.clone();

    conn.execute("INSERT INTO tasks (id, title) values (?1, ?2)", &[&id.to_string(), &title]);
}

fn main() -> Result<()> {
    // for use with the date part later
    // let now = Local::now();
    // let time_stamp = Utc::now().timestamp_millis();

    // establishing the connection to the database
    let conn = Connection::open("rustic.db")?;

    // create table if none exist
    conn.execute(
        "create table if not exists tasks (
            id integer primary key,
            title text not null)"
        , (),)?;

    // Inserting test task
    let task1 = Task{id: 10, title: "number1".to_string()};

    create_db_record(&conn, &task1);

    // Selecting test task
    let mut statmnt = conn.prepare("SELECT id, title from tasks",)?;

    let tasks = statmnt.query_map(() , |row| {
    Ok(Task {id: row.get(0)?, title: row.get(1)?,})
    })?;

    for task in tasks {
        println!("Found Task: {:?}", task);
    }

    Ok(())
}
