use core::panic;
use clap::Parser;
use lab::Lab;
use helpers::read_config;
use machine::{MachineInfo,MachineDifficulty};
use notes::create_notes;

mod machine;
mod lab;
mod helpers;
mod notes;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Command (only supporting "new" for now)
    command: String,

    /// Level (easy, medium, hard, insane)
    level: String,

    /// Machine name
    machine_name: String,
}

fn main() {
    // Read config file
    let config_path: String = String::from("C:\\Users\\patri\\Documents\\Programming projects\\Rust\\Obsidian-Notes-Manager\\notes_manager\\config\\config.json");
    let lab: Lab = read_config(config_path.to_string());

    // Parse CLI arguments
    let args: Args = Args::parse();

    // Get the machine difficulty from CMDLINE args
    let level: MachineDifficulty;

    if args.level.to_lowercase() == "easy" {
        level = MachineDifficulty::Easy;
    } else if args.level.to_lowercase() == "medium" {
        level = MachineDifficulty::Medium;
    } else if args.level.to_lowercase() == "hard" {
        level = MachineDifficulty::Hard;
    } else if args.level.to_lowercase() == "insane" {
        level = MachineDifficulty::Insane;
    } else {
        panic!("Invalid machine level was specified!");
    }

    let mut machine_path: String = lab.path.to_owned();
    
    machine_path.push_str("\\");
    machine_path.push_str(&level.to_string());
    machine_path.push_str("\\");
    machine_path.push_str(&args.machine_name);

    let machine: MachineInfo = MachineInfo::new(args.machine_name, lab, level, machine_path);

    // Array of filename String slices
    let files: [&str; 6] = [
        "00 - Credentials",
        "10 - Nmap",
        "20 - Enumeration",
        "30 - Initial Foothold",
        "40 - Post-exploitation Enumeration",
        "50 - Privilege Escalation"
    ];

    create_notes(machine, files);
}
