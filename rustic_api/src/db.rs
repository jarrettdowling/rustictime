use rusqlite::{Connection, Result};
use crate::task::Task;

// Database functions ---------------------------------------------------------

// Create ---------------------------------------------------------------------

pub fn create_db_record(conn: &mut Connection, task_to_add: &Task) -> Result<()> {
    
    // make a transaction from the db connection
    let tx = conn.transaction()?;

    // collect the fields from the task struct
    let id = task_to_add.id;
    let title = task_to_add.title.clone();
    let priority = task_to_add.priority;
    let duedate = task_to_add.duedate.clone();

    // send the sql command to insert the record
    tx.execute("INSERT INTO tasks (id, title, priority, duedate) values (?1, ?2, ?3, ?4)",
    &[&id.to_string(), &title, &priority.to_string(), &duedate])?;

    // commit the change
    tx.commit()
}

// Read / Fetch ---------------------------------------------------------------

pub fn fetch_priority_n_records(conn: &Connection, priority: u8, ordered: bool) -> Result<Vec<Task>> {

    let mut task_list: Vec<Task> = Vec::new();
    let mut stmt;
    
    // prepare sql statement
    if !ordered {
        stmt = conn.prepare(
            "SELECT * from tasks WHERE priority=(?1)")?;
    }
    else {
        stmt = conn.prepare(
            "SELECT * from tasks WHERE priority=(?1) ORDER BY duedate")?;
    }

    // send the command
    let results = stmt.query_map([priority.to_string()], |row| {
        Ok(Task {
            id: row.get(0)?,
            title: row.get(1)?,
            priority: row.get(2)?,
            duedate: row.get(3)?,
        })
    })?;

    // unpack the results into a vector of tasks
    for task in results {
        task_list.push(task.unwrap());
    }
    
    Ok(task_list)
}

pub fn fetch_record_by_id(conn: &Connection, id: i64) -> Result<Task> {

    //prepare sql statement
    let mut stmt = conn.prepare(
        "SELECT * from tasks WHERE id=(?1)")?;
    // send the sql command
    let mut result = stmt.query_map([id.to_string()], |row| {
        Ok(Task {
            id: row.get(0)?,
            title: row.get(1)?,
            priority: row.get(2)?,
            duedate: row.get(3)?,
        })
    })?;

    let task: Task = result.next().unwrap()?;

    Ok(task)
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
            duedate: row.get(3)?,
        })
    })?;

    // unpack the results into vector of tasks
    for task in results {
        task_list.push(task.unwrap());
    }

    Ok(task_list)
}

// Update ---------------------------------------------------------------------

pub fn update_record(conn: &mut Connection, id: i64) -> Result<()> {
    let mut task: Task = fetch_record_by_id(conn, id).unwrap();
    println!("Updating Task ...");

    Ok(())
}
