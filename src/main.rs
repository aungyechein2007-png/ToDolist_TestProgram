use clap::{Parser, Subcommand};
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Write};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Task {
    description: String,
    done: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Day {
    name: String,
    tasks: Vec<Task>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Week {
    days: Vec<Day>,
}

/// Weekly Task Manager CLI
#[derive(Parser)]
#[command(name = "Week Task CLI")]
#[command(about = "Manage tasks for each day of the week", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add { day: String, task: String },
    List,
    Done { day: String, index: usize },
    Update { day: String, index: usize, new_task: String },
    Delete { day: String, index: usize },
    Today,
    Tomorrow,
    Clear,
}

// Load week data from JSON or create new week
fn load_week() -> Week {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("week.json")
        .expect("Failed to open week.json");

    let reader = BufReader::new(file);
    match serde_json::from_reader(reader) {
        Ok(week) => week,
        Err(_) => {
            let days = vec![
                "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday",
            ]
            .iter()
            .map(|name| Day {
                name: name.to_string(),
                tasks: Vec::new(),
            })
            .collect();
            Week { days }
        }
    }
}

// Save week data to JSON
fn save_week(week: &Week) {
    let json = serde_json::to_string_pretty(week).expect("Failed to serialize week");
    let mut file = File::create("week.json").expect("Failed to create week.json");
    file.write_all(json.as_bytes())
        .expect("Failed to write to file");
}

// Helper: find day by name mutable
fn find_day_mut<'a>(week: &'a mut Week, day_name: &str) -> Option<&'a mut Day> {
    week.days
        .iter_mut()
        .find(|d| d.name.to_lowercase() == day_name.to_lowercase())
}

// Helper: find day by name immutable
fn find_day<'a>(week: &'a Week, day_name: &str) -> Option<&'a Day> {
    week.days
        .iter()
        .find(|d| d.name.to_lowercase() == day_name.to_lowercase())
}

// Display the week as a table with today/tomorrow columns (handles long tasks)
fn display_week_table(week: &Week) {
    let today_name = Local::now().format("%A").to_string();
    let tomorrow_name = Local::now()
        .checked_add_signed(chrono::Duration::days(1))
        .unwrap()
        .format("%A")
        .to_string();

    let mut headers: Vec<String> = week.days.iter().map(|d| d.name.clone()).collect();
    headers.push("Today".to_string());
    headers.push("Tomorrow".to_string());

    // Calculate max column width per column
    let mut col_widths: Vec<usize> = headers.iter().map(|h| h.len()).collect();

    for (i, d) in week.days.iter().enumerate() {
        for t in &d.tasks {
            let len = t.description.len() + 2; // extra for status
            if len > col_widths[i] {
                col_widths[i] = len;
            }
        }
    }

    // Create owned vectors for today and tomorrow tasks
    let today_tasks: Vec<Task> = match find_day(week, &today_name) {
        Some(d) => d.tasks.clone(),
        None => Vec::new(),
    };
    let tomorrow_tasks: Vec<Task> = match find_day(week, &tomorrow_name) {
        Some(d) => d.tasks.clone(),
        None => Vec::new(),
    };

    // Adjust column widths for today/tomorrow
    col_widths.push(today_tasks.iter().map(|t| t.description.len() + 2).max().unwrap_or(5));
    col_widths.push(tomorrow_tasks.iter().map(|t| t.description.len() + 2).max().unwrap_or(8));

    // Print header
    for (i, h) in headers.iter().enumerate() {
        print!("{:<width$}", h, width = col_widths[i] + 2);
    }
    println!();

    // Print separator
    for width in &col_widths {
        print!("{:-<width$}", "", width = width + 2);
    }
    println!();

    // Determine max number of tasks
    let max_tasks = week
        .days
        .iter()
        .map(|d| d.tasks.len())
        .chain(std::iter::once(today_tasks.len()))
        .chain(std::iter::once(tomorrow_tasks.len()))
        .max()
        .unwrap_or(0);

    for i in 0..max_tasks {
        // Days Monday-Sunday
        for (j, d) in week.days.iter().enumerate() {
            if let Some(task) = d.tasks.get(i) {
                let status = if task.done { "✅" } else { "🔘" };
                print!("{:<width$}", format!("{} {}", task.description, status), width = col_widths[j] + 2);
            } else {
                print!("{:<width$}", "", width = col_widths[j] + 2);
            }
        }

        // Today
        if let Some(task) = today_tasks.get(i) {
            let status = if task.done { "✅" } else { "🔘" };
            print!("{:<width$}", format!("{} {}", task.description, status), width = col_widths[7] + 2);
        } else {
            print!("{:<width$}", "", width = col_widths[7] + 2);
        }

        // Tomorrow
        if let Some(task) = tomorrow_tasks.get(i) {
            let status = if task.done { "✅" } else { "🔘" };
            print!("{:<width$}", format!("{} {}", task.description, status), width = col_widths[8] + 2);
        } else {
            print!("{:<width$}", "", width = col_widths[8] + 2);
        }

        println!();
    }
}

fn main() {
    let cli = Cli::parse();
    let mut week = load_week();

    match &cli.command {
        Commands::Add { day, task } => {
            {
                let d = match find_day_mut(&mut week, day) {
                    Some(day) => day,
                    None => {
                        println!("⚠️ Invalid day: {}", day);
                        return;
                    }
                };
                d.tasks.push(Task {
                    description: task.clone(),
                    done: false,
                });
                println!("✅ Added task for {}: {}", d.name, task);
            }
            save_week(&week);
        }
        Commands::List => {
            display_week_table(&week);
        }
        Commands::Done { day, index } => {
            let day_name = day.clone();
            {
                let d = match find_day_mut(&mut week, &day_name) {
                    Some(day) => day,
                    None => {
                        println!("⚠️ Invalid day: {}", day_name);
                        return;
                    }
                };
                if *index == 0 || *index > d.tasks.len() {
                    println!("⚠️ Invalid task number!");
                    return;
                }
                d.tasks[*index - 1].done = true;
            }
            save_week(&week);
            println!("✅ Task {} for {} marked as done", index, day_name);
        }
        Commands::Update {
            day,
            index,
            new_task,
        } => {
            let day_name = day.clone();
            {
                let d = match find_day_mut(&mut week, &day_name) {
                    Some(day) => day,
                    None => {
                        println!("⚠️ Invalid day: {}", day_name);
                        return;
                    }
                };
                if *index == 0 || *index > d.tasks.len() {
                    println!("⚠️ Invalid task number!");
                    return;
                }
                d.tasks[*index - 1].description = new_task.clone();
            }
            save_week(&week);
            println!("✏️ Task {} for {} updated", index, day_name);
        }
        Commands::Delete { day, index } => {
            let day_name = day.clone();
            let removed_description;
            {
                let d = match find_day_mut(&mut week, &day_name) {
                    Some(day) => day,
                    None => {
                        println!("⚠️ Invalid day: {}", day_name);
                        return;
                    }
                };
                if *index == 0 || *index > d.tasks.len() {
                    println!("⚠️ Invalid task number!");
                    return;
                }
                removed_description = d.tasks.remove(*index - 1).description;
            }
            save_week(&week);
            println!("🗑️ Task '{}' for {} deleted", removed_description, day_name);
        }
        Commands::Today => {
            let today_name = Local::now().format("%A").to_string();
            println!("📋 Tasks for today ({})", today_name);
            if let Some(d) = find_day(&week, &today_name) {
                for (i, t) in d.tasks.iter().enumerate() {
                    let status = if t.done { "✅" } else { "🔘" };
                    println!("{}: {} {}", i + 1, t.description, status);
                }
            }
        }
        Commands::Tomorrow => {
            let tomorrow_name = Local::now()
                .checked_add_signed(chrono::Duration::days(1))
                .unwrap()
                .format("%A")
                .to_string();
            println!("📋 Tasks for tomorrow ({})", tomorrow_name);
            if let Some(d) = find_day(&week, &tomorrow_name) {
                for (i, t) in d.tasks.iter().enumerate() {
                    let status = if t.done { "✅" } else { "🔘" };
                    println!("{}: {} {}", i + 1, t.description, status);
                }
            }
        }
        Commands::Clear => {
            for d in &mut week.days {
                d.tasks.clear();
            }
            save_week(&week);
            println!("🗑️ All tasks cleared!");
        }
    }
}
