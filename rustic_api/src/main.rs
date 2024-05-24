mod db;
mod task;

use rusqlite::{Connection, Result};

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
    // let task1 = task::Task::new("Test Task 0".to_string(), 0);
    // db::create_db_record(&mut conn, &task1)?;

    // Selecting test task
    let priority = 1;
    let records_1 = db::fetch_priority_n_records(&conn, priority)?;

    println!("Fetching all priority 1 records: ...");

    for task in records_1 {
        println!("Found Task: {}", task);
    }
 
    let records = db::fetch_records(&conn).unwrap();

    println!("Fetching all records: ...");

    for task in records {
        println!("Found Task: {}", task);
    }
 
    Ok(())
}
