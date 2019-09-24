#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde;

use mytodo::db::{query_task, models::Task};
use rocket_contrib::{json::Json, databases::diesel};

#[derive(Serialize)]
struct JsonApiResponse {
    data: Vec<ResourceObject<Task>>,
}

#[derive(Serialize)]
struct ResourceObject<T> {
    #[serde(rename="type")]
    type_: &'static str,
    id: String,
    attributes: T
}

impl From<Task> for ResourceObject<Task> {
    fn from(task: Task) -> Self {
        ResourceObject {
            type_: "tasks",
            id: task.id.to_string(),
            attributes: task
        }
    }
}

#[get("/tasks")]
fn tasks_get(conn: TestDbConn) -> Json<JsonApiResponse> {
    let mut response = JsonApiResponse { data: vec![] };

    for task in query_task(&conn) {
        response.data.push(task.into());
    }

    Json(response)
}

#[database("testdb")]
struct TestDbConn(diesel::SqliteConnection);

fn main() {
    rocket::ignite()
        .attach(TestDbConn::fairing())
        .mount("/", routes![tasks_get])
        .launch();
}

