# Task Tracker CLI
https://roadmap.sh/projects/task-tracker

A simple command-line interface (CLI) tool to track and manage tasks. This project helps practice programming skills, including working with the filesystem, handling user inputs, and building a functional CLI application. The tasks are stored in a JSON file in the current directory, which is created if it doesn't exist.

## Features

- Add, Update, and Delete tasks
- Mark tasks as "in-progress" or "done"
- List tasks by status (todo, in-progress, done)

## Installation

1. Clone or download this repository to your local machine.
2. Navigate to the project directory.
3. Compile with `cargo build --release`


## Usage

### 1. Add a new task

```bash
$ task-cli add "Task description"
```

- Adds a new task with a unique ID and stores it in the JSON file.

Example:

```bash
$ task-cli add "Buy groceries"
# Output: Task added successfully (ID: 1)
```

### 2. Update a task

```bash
$ task-cli update <task-id> "Updated task description"
```

- Updates the description of a task.

Example:

```bash
$ task-cli update 1 "Buy groceries and cook dinner"
```

### 3. Delete a task

```bash
$ task-cli delete <task-id>
```

- Deletes a task by its ID.
- Use `all` to delete all tasks at once.

Example:

```bash
$ task-cli delete 1
$ task-cli delete all
```

### 4. Mark a task as in progress

```bash
$ task-cli progress <task-id>
```

- Marks a task as "in-progress".

Example:

```bash
$ task-cli progress 1
```

### 5. Mark a task as done

```bash
$ task-cli done <task-id>
```

- Marks a task as "done".

Example:

```bash
$ task-cli done 1
```

### 6. List all tasks

```bash
$ task-cli list
```

- Lists all tasks regardless of their status.

### 7. List tasks by status

```bash
$ task-cli list <status>
```

- Lists tasks by their status: `todo`, `progress`, or `done`.

Example:

```bash
$ task-cli list done
$ task-cli list progress
$ task-cli list todo
```

## Task Properties

Each task will have the following properties:

- `id`: A unique identifier for the task.
- `description`: A brief description of the task.
- `status`: The current status of the task (todo, progress, done).
- `createdAt`: The date and time when the task was created.
- `updatedAt`: The date and time when the task was last updated.

## File Storage

Tasks are stored in a JSON file in the current directory (`tasks.json`). If this file doesn't exist, it will be automatically created.

The JSON structure looks like this:

```json
[
  {
    "id": 1,
    "name": "run in the park",
    "description": "Buy groceries",
    "status": "Todo",
    "created_at": "2024-10-02T22:51:43.161502892+07:00",
    "updated_at": "2024-10-02T22:51:43.161574262+07:00"
  }
]
```

## License

This project is open-source and available under the MIT License.
