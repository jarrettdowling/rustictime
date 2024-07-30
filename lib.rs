pub mod db;
pub mod task;

fn order_tasks() {
    println!("Ordering...");
}

/// This is a function to make a new task and store it in the database
/// Requirements: duedate follows the RFC2822 standard. (I.e. the duedate
///                string has the form "Fri, 14 Jul 2017 02:40:00 +0000")
pub fn new_task(conn: &mut Connection) -> Result<()> {
    // ToDo:
    //      - Create struct
    //      - fill with incoming data
    //      - Call create_db_record
}
