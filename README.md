# Week Task CLI

A simple **Rust CLI tool** to manage tasks for each day of the week. Track tasks, mark them done, update them, or clear them—all from your terminal.

---

## Features

* Add tasks for any day of the week
* List tasks in a table
* Mark tasks as done ✅
* Update tasks ✏️
* Delete tasks 🗑️
* View **today**’s and **tomorrow**’s tasks
* Clear all tasks for the week

---

## Installation

1. Install [Rust](https://www.rust-lang.org/tools/install)
2. Clone the repository or download the project files
3. Build the project:

```bash
cargo build --release
```

4. Run the CLI:

```bash
cargo run -- <command>
```

---

## Commands

* **Add a task**

```bash
cargo run -- add <day> "<task description>"
```

* **List all tasks**

```bash
cargo run -- list
```

* **Mark a task as done**

```bash
cargo run -- done <day> <task_number>
```

* **Update a task**

```bash
cargo run -- update <day> <task_number> "<new_task_description>"
```

* **Delete a task**

```bash
cargo run -- delete <day> <task_number>
```

* **View today’s tasks**

```bash
cargo run -- today
```

* **View tomorrow’s tasks**

```bash
cargo run -- tomorrow
```

* **Clear all tasks**

```bash
cargo run -- clear
```

---

## Example Usage

```bash
cargo run -- add Monday "Buy groceries"
cargo run -- add Tuesday "Gym workout"
cargo run -- list
cargo run -- done Monday 1
cargo run -- update Tuesday 1 "Evening gym session"
cargo run -- delete Monday 1
cargo run -- today
cargo run -- tomorrow
cargo run -- clear
```

> Task numbers start from **1** for each day.
> Tasks are saved automatically in `week.json`.

---

## License

MIT License – free to use and modify
