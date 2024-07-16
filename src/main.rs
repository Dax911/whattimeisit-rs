use chrono::prelude::*;
use clap::{Arg, Command};

fn main() {
    let matches = Command::new("DateTime Formatter")
        .version("0.1.0")
        .author("Dax the Developer <dax@skill-issue.dev>")
        .about("Prints current date and time in various formats")
        .arg(
            Arg::new("format")
                .short('f')
                .long("format")
                .value_name("FORMAT")
                .help("Specify a custom format string")
                .num_args(1),
        )
        .get_matches();

    let now = Local::now();

    println!("Current Date and Time in Various Formats:");
    println!("----------------------------------------");
    println!("Default: {}", now);
    println!("RFC 2822: {}", now.to_rfc2822());
    println!("RFC 3339: {}", now.to_rfc3339());
    println!("Timestamp: {}", now.timestamp());
    println!(
        "Custom (YYYY-MM-DD HH:mm:ss): {}",
        now.format("%Y-%m-%d %H:%M:%S")
    );

    if let Some(format) = matches.get_one::<String>("format") {
        println!("User-specified format: {}", now.format(format));
    }
}
