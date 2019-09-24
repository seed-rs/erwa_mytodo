use std::env;
use mytodo::db::{self, establish_connection, TaskState};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        help();
        return;
    }

    let subcommand = &args[1];
    match subcommand.as_ref() {
        "new" => new_task(&args[2..]),
        "show" => show_tasks(&args[2..]),
        "set_state" => set_task_state(&args[2..]),
        "delete" => delete_task(&args[2..]),
        _ => help(),
    }
}

fn help() {
    println!("\nsubcommands:");
    [
        ("new <title>", "create a new task"),
        ("show", "show tasks"),
        ("set_state <id> <state [pending | done]>", "set task state"),
        ("delete <id>", "delete task"),
    ].iter().for_each(|(command, description)| {
        println!("    {:<40}- {}", command, description)
    });
}

fn new_task(args: &[String]) {
    if args.len() < 1 {
        println!("new: missing <title>");
        help();
        return;
    }

    let conn = establish_connection();
    db::create_task(&conn, &args[0]);
}

fn show_tasks(args: &[String]) {
    if args.len() > 0 {
        println!("show: unexpected argument");
        help();
        return;
    }

    let conn = establish_connection();
    println!("\nTASKS\n-----");
    println!("{:<5} | {:<6} | {}", "ID", "Done", "Title");
    println!("----------------------------------");
    for task in db::query_task(&conn) {
        println!("{:<5} | {:<6} | {}", task.id, task.done, task.title);
    }
}

fn set_task_state(args: &[String]) {
    if args.len() < 2 {
        println!("set_state: missing <id> or <state>");
        help();
        return;
    }

    let id: i32 = match args[0].parse() {
        Ok(id) => id,
        _ => {
            println!("set_state: <id> has to be an integer");
            help();
            return;
        }
    };

    let state: TaskState = match args[1].as_str() {
        "pending" => TaskState::Pending,
        "done" => TaskState::Done,
        _ => {
            println!("set_state: <state> has to be either 'pending' or 'done'");
            help();
            return;
        }
    };

    let conn = establish_connection();
    db::set_task_state(&conn, id, state);
}

fn delete_task(args: &[String]) {
    if args.len() < 1 {
        println!("delete: missing <id>");
        help();
        return;
    }

    let id: i32 = match args[0].parse() {
        Ok(id) => id,
        _ => {
            println!("delete: <id> has to be an integer");
            help();
            return;
        }
    };

    let conn = establish_connection();
    db::delete_task(&conn, id);
}


