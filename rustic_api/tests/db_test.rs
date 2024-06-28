use lib::{db, task};
use rusqlite::{Connection, Result};

mod common;

#[test]
fn basic_db_rw_test() -> Result<()> {
    let mut conn = Connection::open_in_memory()?;

    conn.execute(
        "create table if not exists tasks (
            id integer primary key,
            title text not null,
            priority integer not null,
            duedate text not null)"
        , (),)?;

    let test_task = task::Task::new("Test Task 1".to_string(), 0,
                                     "Fri, 14, Jul 2017 02:40:00 +0000".to_string());

    db::create_db_record(&mut conn, &test_task)?;

    let records = db::fetch_all_records(&conn).unwrap();

    assert_eq!(test_task.id, records.get(0).unwrap().id);
    assert_eq!(test_task.title, records.get(0).unwrap().title);
    assert_eq!(test_task.priority, records.get(0).unwrap().priority);
    assert_eq!(test_task.duedate, records.get(0).unwrap().duedate);

    Ok(())
}

#[test]
fn basic_id_read_test() -> Result<()> {
    let mut conn = Connection::open_in_memory()?;

    conn.execute(
        "create table if not exists tasks (
            id integer primary key,
            title text not null,
            priority integer not null,
            duedate text not null)"
        , (),)?;

    let test_task = task::Task::new("Test Task 1".to_string(), 0,
                                     "Fri, 14, Jul 2017 02:40:00 +0000".to_string());

    db::create_db_record(&mut conn, &test_task)?;

    let record = db::fetch_record_by_id(&conn, test_task.id).unwrap();

    assert_eq!(test_task.id, record.id);
    assert_eq!(test_task.title, record.title);
    assert_eq!(test_task.priority, record.priority);
    assert_eq!(test_task.duedate, record.duedate);


    Ok(())
}

#[test]
fn db_read_test() -> Result<()> {

    let mut task_list: Vec<task::Task> = Vec::new();
    let conn = common::setup_test_db(&mut task_list)?;

    let test_task_1 = task_list.get(0).unwrap();
    let test_task_2 = task_list.get(1).unwrap();
    let test_task_3 = task_list.get(2).unwrap();
    let test_task_4 = task_list.get(3).unwrap();
    let test_task_5 = task_list.get(4).unwrap();

    let records = db::fetch_all_records(&conn).unwrap();
    
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
    let mut task_list: Vec<task::Task> = Vec::new();
    let conn = common::setup_test_db(&mut task_list)?;

    let test_task_1 = task_list.get(0).unwrap();
    let test_task_2 = task_list.get(1).unwrap();
    let test_task_3 = task_list.get(2).unwrap();
    let test_task_4 = task_list.get(3).unwrap();
    let test_task_5 = task_list.get(4).unwrap();

    let records = db::fetch_priority_n_records(&conn, 0, false).unwrap();
    
    assert_eq!(test_task_1.title, records.get(0).unwrap().title);
    assert_eq!(test_task_1.priority, records.get(0).unwrap().priority);
    assert_eq!(test_task_5.title, records.get(1).unwrap().title);
    assert_eq!(test_task_5.priority, records.get(1).unwrap().priority);

    let records = db::fetch_priority_n_records(&conn, 1, false).unwrap();
    
    assert_eq!(test_task_2.title, records.get(0).unwrap().title);
    assert_eq!(test_task_2.priority, records.get(0).unwrap().priority);
    assert_eq!(test_task_4.title, records.get(1).unwrap().title);
    assert_eq!(test_task_4.priority, records.get(1).unwrap().priority);

    let records = db::fetch_priority_n_records(&conn, 2, false).unwrap();
    
    assert_eq!(test_task_3.title, records.get(0).unwrap().title);
    assert_eq!(test_task_3.priority, records.get(0).unwrap().priority);

    Ok(())
}

#[test]
fn db_ordered_read_test() -> Result<()> {
    let mut task_list: Vec<task::Task> = Vec::new();
    let conn = common::setup_test_db(&mut task_list)?;

    let test_task_1 = task_list.get(0).unwrap();
    let test_task_2 = task_list.get(1).unwrap();
    let test_task_3 = task_list.get(2).unwrap();
    let test_task_4 = task_list.get(3).unwrap();
    let test_task_5 = task_list.get(4).unwrap();

    let records = db::fetch_priority_n_records(&conn, 0, true).unwrap();
    
    assert_eq!(test_task_1.title, records.get(0).unwrap().title);
    assert_eq!(test_task_1.priority, records.get(0).unwrap().priority);
    assert_eq!(test_task_5.title, records.get(1).unwrap().title);
    assert_eq!(test_task_5.priority, records.get(1).unwrap().priority);

    let records = db::fetch_priority_n_records(&conn, 1, true).unwrap();
    
    assert_eq!(test_task_2.title, records.get(0).unwrap().title);
    assert_eq!(test_task_2.priority, records.get(0).unwrap().priority);
    assert_eq!(test_task_4.title, records.get(1).unwrap().title);
    assert_eq!(test_task_4.priority, records.get(1).unwrap().priority);

    let records = db::fetch_priority_n_records(&conn, 2, true).unwrap();
    
    assert_eq!(test_task_3.title, records.get(0).unwrap().title);
    assert_eq!(test_task_3.priority, records.get(0).unwrap().priority);

    Ok(())
}
