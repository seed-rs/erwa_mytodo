use diesel::{prelude::*, sqlite::SqliteConnection};

pub mod models;
pub mod schema;

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum TaskState {
    Pending,
    Done,
}

pub fn establish_connection() -> SqliteConnection {
    let db = "./testdb.sqlite3";
    SqliteConnection::establish(db)
        .unwrap_or_else(|_| panic!("Error connecting to {}", db))
}

pub fn create_task(connection: &SqliteConnection, title: &str) {
    let task = models::NewTask { title };

    diesel::insert_into(schema::task::table)
        .values(&task)
        .execute(connection)
        .expect("Error inserting new task");
}

pub fn query_task(connection: &SqliteConnection) -> Vec<models::Task> {
    schema::task::table
        .load::<models::Task>(connection)
        .expect("Error loading tasks")
}

pub fn set_task_state(connection: &SqliteConnection, id: i32, state: TaskState) {
    use schema::task::dsl::{task, done};

    diesel::update(task.find(id))
        .set(done.eq(state == TaskState::Done))
        .execute(connection)
        .expect("Error updating task state");
}

pub fn delete_task(connection: &SqliteConnection, id: i32) {
    use schema::task::dsl::task;

    diesel::delete(task.find(id))
        .execute(connection)
        .expect("Error deleting task");
}


