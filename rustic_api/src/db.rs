use rusqlite::{Connection, Result};
use crate::task::Task;

// Database functions ---------------------------------------------------------

pub fn create_db_record(conn: &mut Connection, task_to_add: &Task) -> Result<()> {
    
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

pub fn fetch_priority_n_records(conn: &Connection, priority: u8) -> Result<Vec<Task>> {

    let mut task_list: Vec<Task> = Vec::new();
    
    // prepare sql statement
    let mut stmt = conn.prepare(
        "SELECT * from tasks WHERE priority=(?1)")?;

    // send the command
    let results = stmt.query_map([priority.to_string()], |row| {
        Ok(Task {
            id: row.get(0)?,
            title: row.get(1)?,
            priority: row.get(2)?,
        })
    })?;

    // unpack the results into a vector of tasks
    for task in results {
        task_list.push(task.unwrap());
    }
    
    Ok(task_list)
}

pub fn fetch_record_by_id(conn: &Connection, id: i64) -> Vec<Task> {

    //prepare sql statement
    let mut stmt = conn.prepare(
        "SELECT from tasks WHERE id=(?1)")?;
    // send the sql command
    let results = stmt.query_map(([id.to_string()]), |row| {
        Ok(Task {
            id: row.get(0)?,
            title: row.get(1)?,
            priority: row.get(2)?,
        })
    })?;

    // unpack the results into vector of tasks
    for task in results {
        task_list.push(task.unwrap());
    }

    Ok(task_list)
    
}

pub fn fetch_all_records(conn: &Connection) -> Result<Vec<Task>> {

    let mut task_list: Vec<Task> = Vec::new();
    
    // prepare sql statement
    let mut stmt = conn.prepare(
        "SELECT * from tasks")?;

    // send the sql command
    let results = stmt.query_map((), |row| {
        Ok(Task {
            id: row.get(0)?,
            title: row.get(1)?,
            priority: row.get(2)?,
        })
    })?;

    // unpack the results into vector of tasks
    for task in results {
        task_list.push(task.unwrap());
    }

    Ok(task_list)
}

