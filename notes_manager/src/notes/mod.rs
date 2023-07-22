use std::{fs::{self, File}, io::Write};
use crate::machine::MachineInfo;

/// Function
fn add_links(machine: &MachineInfo, files: &[&str; 6]) {
    let mut link_file_path: String = machine.path.to_owned();

    link_file_path.push_str("\\");
    link_file_path.push_str(&machine.name);
    link_file_path.push_str(".md");

    let f_open: Result<File, std::io::Error> = File::create(link_file_path);

    let mut f: File = match f_open {
        Ok(f) => f,
        Err(e) => panic!("Failed to open file: {:?}", e)
    };

    // f.write_all(b"Test").expect("Failed to write to a file!");

    for file in files {
        let obsidian_path: String = format!("[[Labs/HackTheBox/Boxes/{}/{}/{}|{}]]\n", machine.difficulty, machine.name, file, file);
        f.write_all(obsidian_path.as_bytes()).expect("Failed to write to a file!");
    }
}

pub fn create_notes(machine: MachineInfo, files: [&str; 6]) {
    
    println!("{}", machine.path);
   
    fs::create_dir_all(&machine.path).expect("Failed to create directories!");

    for file in files {
        println!("\t./{}", file);

        let mut file_path: String = machine.path.to_owned();
        
        file_path.push_str("\\");
        file_path.push_str(file);
        file_path.push_str(".md");

        fs::File::create(file_path).expect("Failed to create file!");
    }
    
    add_links(&machine, &files);
}
