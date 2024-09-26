use chrono::{DateTime, Local, Utc};
use clap::Parser;
use std::{fs, io, path::Path};

mod task {

    use crate::{local_time, write_to_json_store};
    use chrono::{DateTime, Local};
    use core::fmt;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, Serialize)]
    pub enum TaskStatus {
        Done,
        Progress,
        Todo,
    }

    impl core::fmt::Display for TaskStatus {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let status = match self {
                Self::Done => "[✔]",
                Self::Progress => "[~]",
                Self::Todo => "[ ]",
            };
            write!(f, "{}", status)
        }
    }

    #[derive(Debug, serde::Deserialize, serde::Serialize)]
    pub struct Task {
        id: i8,
        name: String,
        status: TaskStatus,
        created_at: DateTime<Local>,
    }

    impl Task {
        pub fn new(name: String, tasks: &Tasks) -> Task {
            return Task {
                id: tasks.id(),
                name,
                created_at: local_time(),
                status: TaskStatus::Todo,
            };
        }
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Tasks {
        tasks: Vec<Task>,
    }

    impl Tasks {
        pub fn new() -> Tasks {
            return Tasks { tasks: vec![] };
        }
        pub fn add_task(&mut self, task: Task) {
            self.tasks.push(task);
            write_to_json_store(&self).unwrap();
        }

        pub fn delete_task(&mut self, id: i8) {
            println!("deleting task {}", id);
            self.tasks.retain(|item| item.id != id);
            write_to_json_store(&self).unwrap();
        }

        pub fn delete_all_tasks(&mut self) {
            println!("deleting all tasks");
            self.tasks.clear();
            write_to_json_store(&self).unwrap();
        }

        pub fn id(&self) -> i8 {
            let len_tasks = self.tasks.len();
            if len_tasks == 0 {
                return 1;
            }
            return self.tasks[len_tasks - 1].id + 1;
        }

        pub fn mark_progress(&mut self, id: i8) {
            for task in self.tasks.iter_mut() {
                if task.id == id {
                    task.status = TaskStatus::Progress;
                }
            }
            write_to_json_store(&self).unwrap();
        }

        pub fn mark_done(&mut self, id: i8) {
            for task in self.tasks.iter_mut() {
                if task.id == id {
                    task.status = TaskStatus::Done;
                }
            }
            write_to_json_store(&self).unwrap();
        }

        pub fn list(&self) {
            for task in self.tasks.iter() {
                println!("{}", task);
            }
        }

        pub fn list_done(&self) {
            for task in self.tasks.iter() {
                if let TaskStatus::Done = task.status {
                    println!("{}", task)
                }
            }
        }
        pub fn list_progress(&self) {
            for task in self.tasks.iter() {
                if let TaskStatus::Progress = task.status {
                    println!("{}", task)
                }
            }
        }
        pub fn list_todo(&self) {
            for task in self.tasks.iter() {
                if let TaskStatus::Todo = task.status {
                    println!("{}", task)
                }
            }
        }
        pub fn update_task(&mut self, id: i8, name: String) {
            if id < 1 {
                println!("ID cannot be less than 0");
            }
            if let Some(task) = self.tasks.get_mut((id - 1) as usize) {
                task.name = name;
                println!("{}", task);
                write_to_json_store(self).unwrap();
            } else {
                println!("Task not found");
            }
        }
    }

    impl fmt::Display for Task {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let desc_width = 20;
            write!(
                f,
                " {}. {:<desc_width$} {}",
                self.id, self.name, self.status
            )
        }
    }
}

fn initiate_json_store() -> io::Result<task::Tasks> {
    let path = Path::new("tasks.json");
    if path.exists() {
        let json_file = fs::read_to_string(path)?;
        let tasks: task::Tasks = serde_json::from_str(&json_file)?;

        Ok(tasks)
    } else {
        println!("tasks.json does not exist");
        let empty_tasks = task::Tasks::new();
        let json_data = serde_json::to_string_pretty(&empty_tasks)?;
        fs::write(path, json_data)?;

        Ok(empty_tasks)
    }
}

fn write_to_json_store(contents: &task::Tasks) -> io::Result<()> {
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
    val2: Option<String>,
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
                println!("Provide the task id!");
                return;
            }
            let task_add = task::Task::new(args.val.expect("a string"), &tasks);
            tasks.add_task(task_add);
        }
        Some("delete") => {
            if args.val == None {
                println!("Provide the task id!");
                return;
            }
            if args.val.as_deref() == Some("all") {
                tasks.delete_all_tasks();
                return;
            }
            let id = option_string_to_i8(args.val).unwrap();
            tasks.delete_task(id)
        }
        Some("progress") => {
            if args.val == None {
                println!("Provide the task id!");
                return;
            }
            let id = option_string_to_i8(args.val).unwrap();
            tasks.mark_progress(id);
        }
        Some("done") => {
            if args.val == None {
                println!("Provide the task id!")
            }
            let id = option_string_to_i8(args.val).unwrap();
            tasks.mark_done(id);
        }
        Some("list") => {
            if args.val == Some("done".to_string()) {
                tasks.list_done();
            } else if args.val == Some("progress".to_string()) {
                tasks.list_progress();
            } else if args.val == Some("todo".to_string()) {
                tasks.list_todo();
            } else {
                tasks.list();
            }
        }
        Some("update") => {
            let id = option_string_to_i8(args.val).unwrap();
            if let Some(name) = args.val2 {
                tasks.update_task(id, name)
            }
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
