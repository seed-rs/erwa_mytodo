#[macro_use]
extern crate serde;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub done: bool,
}

impl From<Task> for ResourceObject<Task> {
    fn from(task: Task) -> Self {
        ResourceObject {
            type_: "tasks".to_owned(),
            id: task.id.to_string(),
            attributes: task
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ResourceObject<T> {
    #[serde(rename="type")]
    pub type_: String,
    pub id: String,
    pub attributes: T
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct JsonApiResponse {
    pub data: Vec<ResourceObject<Task>>,
}

