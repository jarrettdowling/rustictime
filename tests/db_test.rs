use rustictime::{db, task};
use rusqlite::{Connection, Result};

#[test]
fn basic_db_rw_test() -> Result<()> {
    let mut conn = Connection::open_in_memory()?;

    conn.execute(
        "create table if not exists tasks (
            id integer primary key,
            title text not null,
            priority integer not null)"
        , (),)?;

    let test_task = task::Task::new("Test Task 1".to_string(), 0);

    db::create_db_record(&mut conn, &test_task)?;

    let records = db::fetch_records(&conn).unwrap();

    assert_eq!(test_task.id, records.get(0).unwrap().id);
    assert_eq!(test_task.title, records.get(0).unwrap().title);
    assert_eq!(test_task.priority, records.get(0).unwrap().priority);

    Ok(())
}

#[test]
fn db_read_test() -> Result<()> {
    let conn = Connection::open("./tests/read_records_test.db")?;

    let test_task_1 = task::Task::new("Test Task 1".to_string(), 0);
    let test_task_2 = task::Task::new("Test Task 2".to_string(), 1);
    let test_task_3 = task::Task::new("Test Task 3".to_string(), 2);
    let test_task_4 = task::Task::new("Test Task 4".to_string(), 1);
    let test_task_5 = task::Task::new("Test Task 5".to_string(), 0);

    let records = db::fetch_records(&conn).unwrap();
    
    assert_eq!(test_task_1.title, records.get(0).unwrap().title);
    assert_eq!(test_task_1.priority, records.get(0).unwrap().priority);
    assert_eq!(test_task_2.title, records.get(1).unwrap().title);
    assert_eq!(test_task_2.priority, records.get(1).unwrap().priority);
    assert_eq!(test_task_3.title, records.get(2).unwrap().title);
    assert_eq!(test_task_3.priority, records.get(2).unwrap().priority);
    assert_eq!(test_task_4.title, records.get(3).unwrap().title);
    assert_eq!(test_task_4.priority, records.get(3).unwrap().priority);
    assert_eq!(test_task_5.title, records.get(4).unwrap().title);
    assert_eq!(test_task_5.priority, records.get(4).unwrap().priority);

    Ok(())
}

#[test]
fn db_priority_read_test() -> Result<()> {
    let conn = Connection::open("./tests/read_records_test.db")?;

    let test_task_1 = task::Task::new("Test Task 1".to_string(), 0);
    let test_task_2 = task::Task::new("Test Task 2".to_string(), 1);
    let test_task_3 = task::Task::new("Test Task 3".to_string(), 2);
    let test_task_4 = task::Task::new("Test Task 4".to_string(), 1);
    let test_task_5 = task::Task::new("Test Task 5".to_string(), 0);

    let records = db::fetch_priority_n_records(&conn, 0).unwrap();
    
    assert_eq!(test_task_1.title, records.get(0).unwrap().title);
    assert_eq!(test_task_1.priority, records.get(0).unwrap().priority);
    assert_eq!(test_task_5.title, records.get(1).unwrap().title);
    assert_eq!(test_task_5.priority, records.get(1).unwrap().priority);

    let records = db::fetch_priority_n_records(&conn, 1).unwrap();
    
    assert_eq!(test_task_2.title, records.get(0).unwrap().title);
    assert_eq!(test_task_2.priority, records.get(0).unwrap().priority);
    assert_eq!(test_task_4.title, records.get(1).unwrap().title);
    assert_eq!(test_task_4.priority, records.get(1).unwrap().priority);

    let records = db::fetch_priority_n_records(&conn, 2).unwrap();
    
    assert_eq!(test_task_3.title, records.get(0).unwrap().title);
    assert_eq!(test_task_3.priority, records.get(0).unwrap().priority);

    Ok(())
}
