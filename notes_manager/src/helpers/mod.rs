use std::fs;
use crate::lab::Lab;

/// Reads the JSON config file and returns a Lab object to the caller
/// containing the name of the lab and path to the Obsidian vault
/// folder where the lab machine's notes are usually stored
pub fn read_config(config_path: String) -> Lab {
    // Read config file from the file system
    let file: Result<String, std::io::Error> = fs::read_to_string(config_path);

    let file_contents: String = match file {
        Ok(contents) => contents,
        Err(error) => panic!("Failed to open the file! {:?}", error)
    };

    // Deserialize the JSON config file into a "Lab::Lab" structure
    let json: Lab = serde_json::from_str(&file_contents).expect("JSON error!");

    return json;
}
