use chrono::{DateTime, Local, Utc};
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::{
    fs::{self},
    io,
    path::Path,
};

#[derive(Debug, Deserialize, Serialize)]
enum TaskStatus {
    Done,
    Progress,
    Todo,
}

#[derive(Debug, Deserialize, Serialize)]
struct Task {
    id: i8,
    name: String,
    status: TaskStatus,
    created_at: DateTime<Local>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Tasks {
    tasks: Vec<Task>,
}

fn initiate_json_store() -> io::Result<Tasks> {
    let path = Path::new("tasks.json");
    if path.exists() {
        println!("tasks.json exists!");
        let json_file = fs::read_to_string(path)?;
        let tasks: Tasks = serde_json::from_str(&json_file)?;

        Ok(tasks)
    } else {
        println!("tasks.json does not exist");
        let empty_tasks = Tasks { tasks: vec![] };
        let json_data = serde_json::to_string_pretty(&empty_tasks)?;
        fs::write(path, json_data)?;

        Ok(empty_tasks)
    }
}

fn local_time() -> DateTime<Local> {
    let utc_time: DateTime<Utc> = Utc::now();
    let local_time: DateTime<Local> = utc_time.with_timezone(&Local);
    local_time
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    cmd: Option<String>,
    val: Option<String>,
}

fn main() {
    let args = Cli::parse();

    match args.cmd.as_deref() {
        Some("add") => {
            if args.val == None {
                println!("Provide the task!")
            }
            let task = Task {
                name: args.val.expect("Should be a string"),
                id: 2,
                created_at: local_time(),
                status: TaskStatus::Todo,
            };

            println!("{:?}", task)
        }
        _ => {
            println!("unknown command")
        }
    }
    local_time();
    let _tasks = initiate_json_store();
}
