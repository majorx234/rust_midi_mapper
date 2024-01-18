use serde::{Deserialize, Serialize};
use std::io::Read;
use std::result::Result;
use std::{fmt, fs::File};

#[derive(Hash, Eq, PartialEq, Serialize, Deserialize, Clone, Debug)]
pub struct MidiFunction {
    name: String,
    // ToDo: invert, log,linear, scaling..
}

impl MidiFunction {
    pub fn new(name: String) -> Self {
        MidiFunction { name }
    }
    pub fn get_name(&self) -> String {
        self.name.as_str().to_string()
    }
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
        Err(err) => {
            println!("Could not read the json file: {}", err);
            return Err("Could not read the json file");
        }
    };
    let mut contents = String::new();
    match file_content.read_to_string(&mut contents) {
        Ok(_) => {}
        Err(err) => {
            println!("Could not read file to string: {}", err);
            return Err("Could not read file to string");
        }
    };
    let module: MidiFunctionFile = match serde_json::from_str(contents.as_str()) {
        Ok(module) => module,
        Err(err) => {
            println!("error in json desirialize: {}", err);
            return Err("error in json desirialize");
        }
    };
    println!("{:?}", module);
    Ok(module)
}
