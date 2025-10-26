Week Task CLI

A simple Command-Line Interface (CLI) tool in Rust to manage tasks for each day of the week. Keep track of tasks, mark them done, update, or clear them quickly from the terminal.

Features

Add tasks for any day of the week

List all tasks in a neat table

Mark tasks as done

Update tasks

Delete individual tasks

View today’s and tomorrow’s tasks

Clear all tasks for the week

Installation

Make sure you have Rust
 installed.

Clone the repository or download the files.

Build the project:

cargo build --release


Run the CLI:

cargo run -- <command>

Commands

Add a task:

cargo run -- add <day> "<task description>"


List all tasks:

cargo run -- list


Mark a task as done:

cargo run -- done <day> <task_number>


Update a task:

cargo run -- update <day> <task_number> "<new_task_description>"


Delete a task:

cargo run -- delete <day> <task_number>


View today’s tasks:

cargo run -- today


View tomorrow’s tasks:

cargo run -- tomorrow


Clear all tasks for the week:

cargo run -- clear

Example
cargo run -- add Monday "Buy groceries"
cargo run -- add Tuesday "Gym workout"
cargo run -- list
cargo run -- done Monday 1
cargo run -- update Tuesday 1 "Evening gym session"
cargo run -- delete Monday 1
cargo run -- today
cargo run -- tomorrow
cargo run -- clear

Notes

Task numbers start from 1 in each day.

The week.json file stores your tasks automatically.

License

This project is open-source under the MIT License.
