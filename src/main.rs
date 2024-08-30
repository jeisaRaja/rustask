use chrono::{DateTime, Local, Utc};
use clap::Parser;

#[derive(Debug)]
enum TaskStatus {
    Done,
    Progress,
    Todo,
}

fn local_time() -> DateTime<Local> {
    let utc_time: DateTime<Utc> = Utc::now();
    let local_time: DateTime<Local> = utc_time.with_timezone(&Local);

    local_time
}

#[derive(Debug)]
struct Task {
    id: i8,
    name: String,
    status: TaskStatus,
    created_at: DateTime<Local>,
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
}
