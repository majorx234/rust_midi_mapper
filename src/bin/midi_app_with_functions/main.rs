/*
 * This file is part of the rust_midi_mapper distribution (https://github.com/majorx234/rust_midi_mapper ).
 * Copyright (c) 2023-2024 Majorx234 <majorx234@googlemail.com>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, version 3.
 *
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
 * General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

use std::collections::HashMap;

use clap::Parser;
use midi_mapper::{
    jackmidi::MidiMsgAdvanced,
    midi_function::{
        parse_json_file_to_midi_functions,
        parse_json_file_to_midi_functions_with_midi_msgs_advanced, MidiFunction, MidiFunctionFile,
    },
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// midi_functions_filepath
    #[arg(short, long, value_name = "filepath")]
    pub midi_functions_filepath: Option<String>,
    /// midi_mapping_filepath
    #[arg(short, long, value_name = "filepath")]
    pub midi_mapping_filepath: Option<String>,
}

fn main() {
    let midi_functions = Args::parse().midi_functions_filepath.map_or_else(
        || MidiFunctionFile {
            midi_functions: vec![
                MidiFunction::new("Volume".to_string()),
                MidiFunction::new("Modulate".to_string()),
                MidiFunction::new("FmIntensity".to_string()),
            ],
        },
        |filepath| {
            parse_json_file_to_midi_functions(&filepath).map_or_else(
                |err| {
                    println!("{err}");
                    MidiFunctionFile::default()
                },
                |file_path| file_path,
            )
        },
    );
    println!("midi_function: {:?}", midi_functions);
    let midi_functions_with_midi_advanced_msgs: Result<
        HashMap<String, Vec<MidiMsgAdvanced>>,
        String,
    > = Args::parse().midi_mapping_filepath.map_or_else(
        || Ok(HashMap::<String, Vec<MidiMsgAdvanced>>::new()),
        |filepath| {
            // Todo: parse filepath
            parse_json_file_to_midi_functions_with_midi_msgs_advanced(&filepath)
        },
    );
    println!("midi_mapping: {:?}", midi_functions_with_midi_advanced_msgs);
}
