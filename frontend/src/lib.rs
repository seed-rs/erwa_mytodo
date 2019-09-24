#[macro_use]
extern crate seed;
use futures::Future;
use mytodo::{JsonApiResponse, Task};
use seed::prelude::*;
use seed::{fetch, Request};

struct Model {
    tasks: Vec<Task>,
}

#[derive(Clone, Debug)]
enum Msg {
    FetchedTasks(fetch::ResponseDataResult<JsonApiResponse>),
}

fn update(msg: Msg, model: &mut Model, _orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::FetchedTasks(Ok(result)) => {
            model.tasks.clear();
            model.tasks.append(
                &mut result
                    .data
                    .into_iter()
                    .map(|resource_object| resource_object.attributes)
                    .collect(),
            );
        }
        Msg::FetchedTasks(Err(reason)) => {
            log!(format!("Error fetching: {:?}", reason));
        }
    }
}

fn view(model: &Model) -> impl View<Msg> {
    let tasks: Vec<Node<Msg>> = model
        .tasks
        .iter()
        .map(|t| li![t.title.clone(), if t.done { "[X]" } else { "[ ]" }])
        .collect();

    h1![{ "Tasks" }, ul![tasks,],]
}

fn fetch_drills() -> impl Future<Item = Msg, Error = Msg> {
    Request::new("http://localhost:8000/tasks/").fetch_json_data(Msg::FetchedTasks)
}

fn init(_url: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.perform_cmd(fetch_drills());
    Model { tasks: vec![] }
}

#[wasm_bindgen(start)]
pub fn render() {
    seed::App::build(init, update, view).finish().run();
}
