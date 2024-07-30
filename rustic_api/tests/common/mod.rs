use lib::{db, task};
use rusqlite::{Connection, Result};
use std::{thread, time};

// sets up common db to test read and write on.
// sleep is used to ensure unique ids for the tasks.
// used in: db_tests.rs
pub fn setup_test_db(task_list: &mut Vec<task::Task>) -> Result<Connection> {
    
    let ten_millis = time::Duration::from_millis(10);
    let mut conn = Connection::open_in_memory()?;
    
    // create db in memory
    conn.execute(
        "create table if not exists tasks (
            id integer primary key,
            title text not null,
            priority integer not null,
            parts integer not null,
            duedate text not null,
            description text not null)"
        , (),)?;

    // create test tasks
    let test_task_1 = task::Task::new("Test Task 1".to_string(), 0, 1,
                                        "Fri, 14, Jul 2017 02:40:00 +0000".to_string(),
                                        "Test Task 1 Description".to_string());
    thread::sleep(ten_millis);
    let test_task_2 = task::Task::new("Test Task 2".to_string(), 1, 2,
                                        "Fri, 14, Jul 2017 02:50:00 +0000".to_string(),
                                        "Test Task 2 Description".to_string());
    thread::sleep(ten_millis);
    let test_task_3 = task::Task::new("Test Task 3".to_string(), 2, 3,
                                        "Fri, 14, Jul 2017 03:00:00 +0000".to_string(),
                                        "Test Task 3 Description".to_string());
    thread::sleep(ten_millis);
    let test_task_4 = task::Task::new("Test Task 4".to_string(), 1, 4,
                                        "Fri, 14, Jul 2017 03:10:00 +0000".to_string(),
                                        "Test Task 4 Description".to_string());
    thread::sleep(ten_millis);
    let test_task_5 = task::Task::new("Test Task 5".to_string(), 0, 5,
                                        "Fri, 14, Jul 2017 03:20:00 +0000".to_string(),
                                        "Test Task 5 Description".to_string());

    // input into db
    db::create_db_record(&mut conn, &test_task_1)?;
    db::create_db_record(&mut conn, &test_task_2)?;
    db::create_db_record(&mut conn, &test_task_3)?;
    db::create_db_record(&mut conn, &test_task_4)?;
    db::create_db_record(&mut conn, &test_task_5)?;
    
    // tasks added to vector
    task_list.push(test_task_1);
    task_list.push(test_task_2);
    task_list.push(test_task_3);
    task_list.push(test_task_4);
    task_list.push(test_task_5);

    Ok(conn)
}
