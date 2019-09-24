#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde;

use backend::db::query_task;
use rocket_contrib::{json::Json, databases::diesel};
use rocket_cors::{AllowedHeaders, AllowedOrigins, Error};
use mytodo::{self, JsonApiResponse};

#[get("/tasks")]
fn tasks_get(conn: TestDbConn) -> Json<JsonApiResponse> {
    let mut response = JsonApiResponse { data: vec![] };

    for db_task in query_task(&conn) {
        let api_task = mytodo::Task {
            id: db_task.id,
            title: db_task.title,
            done: db_task.done,
        };
        response.data.push(api_task.into());
    }

    Json(response)
}

#[database("testdb")]
struct TestDbConn(diesel::SqliteConnection);

fn main() -> Result<(), Error> {
    let allowed_origins = AllowedOrigins::all();

    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    }
        .to_cors()?;

    rocket::ignite()
        .attach(TestDbConn::fairing())
        .attach(cors)
        .mount("/", routes![tasks_get])
        .launch();

    Ok(())
}

