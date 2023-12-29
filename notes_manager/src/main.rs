use core::panic;
use clap::Parser;
use lab::Lab;
use helpers::read_config;
use machine::{MachineInfo,MachineDifficulty};
use notes::create_notes;
use std::io;

mod machine;
mod lab;
mod helpers;
mod notes;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Command (only supporting "new" for now)
    command: String,

    /// HTB/VL
    lab: String,

    /// Level (easy, medium, hard, insane)
    level: String,

    /// Machine name
    machine_name: String,
}

fn main() {
    // Read config file
    let config_path: String = String::from("C:\\Users\\patri\\Documents\\Programming projects\\Rust\\Obsidian-Notes-Manager\\notes_manager\\config\\config.json");
    let labs: Vec<Lab> = read_config(config_path.to_string());

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

     // Find the LabConfig based on the provided lab parameter
    let lab_config: Result<&Lab, io::Error> = labs
        .iter()
        .find(|config| config.name == args.lab)
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Lab not found"));

    let lab: Lab = lab_config.unwrap().clone();

    let mut machine_path: String = lab.path.to_owned();
    
    machine_path.push_str("\\");
    machine_path.push_str(&level.to_string());
    machine_path.push_str("\\");
    machine_path.push_str(&args.machine_name);

    let machine: MachineInfo = MachineInfo::new(args.machine_name, lab, level, machine_path);

    // Array of filename String slices
    let files: [&str; 5] = [
        "00 - Credentials",
        "10 - Nmap",
        "20 - Enumeration",
        "30 - User",
        "40 - Root"
    ];

    create_notes(machine, files);
}
