use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::path::Path;

struct LogMessage {
    error_type: String,
    error_message: String,
}

// Function to read lines from a file
fn read_lines<P>(filename: P) -> io::Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

// Function to parse a log line into a LogMessage
fn parse_log_message(line: &str) -> Option<LogMessage> {
    if line.starts_with("[ERROR]") {
        let parts: Vec<&str> = line.splitn(3, ' ').collect();
        if parts.len() < 3 {
            return None;
        }
        let error_type = parts[1].to_string();
        let error_message = parts[2..].join(" ").to_string();
        Some(LogMessage {
            error_type,
            error_message,
        })
    } else {
        None
    }
}

fn main() {
    // Get the filename from the command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("You must provide a log file as an argument.");
        return;
    }
    let filename = &args[1];

    // Map to hold count of each error type
    let mut error_counts: HashMap<String, u32> = HashMap::new();

    // Map to hold count of each error message
    let mut message_counts: HashMap<String, u32> = HashMap::new();

    // Read lines from the file
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(log) = line {
                if let Some(log_message) = parse_log_message(&log) {
                    // Increase the count for this error type
                    let count = error_counts
                        .entry(log_message.error_type.clone())
                        .or_insert(0);
                    *count += 1;

                    // Increase the count for this error message
                    let message_count = message_counts
                        .entry(log_message.error_message.clone())
                        .or_insert(0);
                    *message_count += 1;
                }
            }
        }
    }

    // Print the counts of each error type
    for (error_type, count) in &error_counts {
        println!("{}: {}", error_type, count);
    }

    // Print the 5 most common error messages
    let mut messages: Vec<(&String, &u32)> = message_counts.iter().collect();
    messages.sort_by(|a, b| b.1.cmp(a.1));
    for (message, count) in messages.iter().take(5) {
        println!("\"{}\" occurred {} time(s)", message, count);
    }
}
