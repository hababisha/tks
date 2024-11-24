use std::env;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use rand::seq::SliceRandom;
use std::os::unix::fs::PermissionsExt;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: tks <command>");
        return;
    }

    match args[1].as_str() {
        "init" => initialize(),
        "add" => add_quote(),
        _ => eprintln!("Unknown command. Use 'tks init' or 'tks add'."),
    }
}

fn initialize() {
    let home_dir = env::var("HOME").expect("Could not find HOME directory");
    let bashrc_path = format!("{}/.bashrc", home_dir);
    let quotes_file_path = format!("{}/.tks/quotes.txt", home_dir);

    // Ensure the directory exists
    fs::create_dir_all(format!("{}/.tks", home_dir)).expect("Failed to create .tks directory");

    // Create the random_quote script
    let random_quote_script = format!("{}/.tks/random_quote", home_dir);
    let script_content = format!(
        "#!/bin/bash\ncargo run --quiet --bin tks display_random_quote\n"
    );
    fs::write(&random_quote_script, script_content).expect("Failed to write random_quote script");

    // Make the random_quote script executable
    let mut permissions = fs::metadata(&random_quote_script).expect("Failed to get file metadata")
        .permissions();
    permissions.set_mode(0o755); // rwx for owner, rx for group and others
    fs::set_permissions(&random_quote_script, permissions).expect("Failed to set permissions");

    // Add command to .bashrc to run random_quote script
    let init_command = format!("\n# Show a random quote from tks\n~/.tks/random_quote\n");

    let mut bashrc_file = OpenOptions::new()
        .append(true)
        .open(&bashrc_path)
        .expect("Failed to open .bashrc");

    bashrc_file
        .write_all(init_command.as_bytes())
        .expect("Failed to write to .bashrc");

    println!("tks initialized! A random quote will now display when you open a terminal.");
}

fn add_quote() {
    let home_dir = env::var("HOME").expect("Could not find HOME directory");
    let quotes_file_path = format!("{}/.tks/quotes.txt", home_dir);

    println!("Enter a new quote: ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let input = input.trim();

    if input.is_empty() {
        eprintln!("Quote cannot be empty!");
        return;
    }

    // Create quotes file if it doesn't exist
    fs::create_dir_all(format!("{}/.tks", home_dir)).expect("Failed to create .tks directory");
    let mut quotes_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(quotes_file_path)
        .expect("Failed to open quotes file");

    writeln!(quotes_file, "{}", input).expect("Failed to write quote to file");

    println!("Quote added successfully!");
}

fn display_random_quote() {
    let home_dir = env::var("HOME").expect("Could not find HOME directory");
    let quotes_file_path = format!("{}/.tks/quotes.txt", home_dir);

    let quotes = fs::read_to_string(&quotes_file_path)
        .expect("Failed to read quotes file")
        .lines()
        .map(String::from)
        .collect::<Vec<String>>();

    if quotes.is_empty() {
        eprintln!("No quotes available. Add some with 'tks add'.");
        return;
    }

    let random_quote = quotes.choose(&mut rand::thread_rng()).unwrap();
    println!("{}", random_quote);
}
