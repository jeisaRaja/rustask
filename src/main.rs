use chrono::{DateTime, Local, Utc};
use clap::Parser;
use core::fmt;
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
impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let status = match self {
            Self::Done => "[✔]",
            Self::Progress => "[~]",
            Self::Todo => "[ ]",
        };
        write!(f, "{}", status)
    }
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

impl Tasks {
    fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
        write_to_json_store(&self).unwrap();
    }

    fn id(&self) -> i8 {
        let len_tasks = self.tasks.len();
        if len_tasks == 0 {
            return 1;
        }
        return self.tasks[len_tasks - 1].id + 1;
    }

    fn mark_progress(&mut self, id: i8) {
        for task in self.tasks.iter_mut() {
            if task.id == id {
                task.status = TaskStatus::Progress;
            }
        }
        write_to_json_store(&self).unwrap();
    }

    fn list(&self) {
        for task in self.tasks.iter() {
            println!("{}", task);
        }
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let desc_width = 20;
        write!(f, " {}. {:<desc_width$} {}", self.id, self.name, self.status)
    }
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

fn write_to_json_store(contents: &Tasks) -> io::Result<()> {
    let path = Path::new("tasks.json");
    let str_contents = serde_json::to_string_pretty(contents)?;
    fs::write(path, str_contents)?;
    Ok(())
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

    let mut tasks = match initiate_json_store() {
        Ok(tasks) => tasks,
        Err(e) => {
            println!("Error while initiating tasks: {}", e);
            return;
        }
    };

    match args.cmd.as_deref() {
        Some("add") => {
            if args.val == None {
                println!("Provide the task!");
                return;
            }
            let id = tasks.id();
            let task = Task {
                name: args.val.expect("Should be a string"),
                id,
                created_at: local_time(),
                status: TaskStatus::Todo,
            };
            tasks.add_task(task);
        }
        Some("progress") => {
            if args.val == None {
                println!("Provide the task!");
                return;
            }
            if let Ok(id) = option_string_to_i8(args.val) {
                tasks.mark_progress(id);
            }
        }
        Some("list") => {
            tasks.list();
        }
        _ => {
            println!("unknown command")
        }
    }
}

fn option_string_to_i8(opt: Option<String>) -> Result<i8, std::num::ParseIntError> {
    opt.ok_or_else(|| "No value".parse::<i8>().unwrap_err())?
        .parse::<i8>()
}
