use clap::error::ErrorFormatter;
use serde::{Deserialize, Serialize};
use std::io::Read;
use std::result::Result;
use std::{collections::HashSet, error::Error, fmt, fs::File};

#[derive(Hash, Eq, PartialEq, Serialize, Deserialize, Copy, Clone, Debug)]
pub enum MidiFunction {
    Play,
    Volume,
    Modulate,
    FmIntensity,
    // ToDo
}

impl fmt::Display for MidiFunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MidiFunctionFile {
    pub midi_functions: Vec<MidiFunction>,
}

pub fn parse_json_file_to_midi_functions(file_path_str: &String) -> Result<MidiFunctionFile, &str> {
    let mut file_content = match File::open(file_path_str) {
        Ok(file) => file,
        Err(_) => return Err("Could not read the json file"),
    };
    let mut contents = String::new();
    match file_content.read_to_string(&mut contents) {
        Ok(_) => {}
        Err(err) => {
            return Err("Could not read file to string");
        }
    };
    let module: MidiFunctionFile = match serde_json::from_str(&contents.as_str()) {
        Ok(module) => module,
        Err(_) => {
            return Err("error in json desirialize");
        }
    };
    Ok(module)
}
