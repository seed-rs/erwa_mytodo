use super::schema::task;

#[derive(Insertable)]
#[table_name = "task"]
pub struct NewTask<'a> {
    pub title: &'a str,
}

#[derive(Queryable)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub done: bool,
}