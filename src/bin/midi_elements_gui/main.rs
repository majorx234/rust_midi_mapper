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

use crossbeam_channel::unbounded;
use eframe::{self, egui::ViewportBuilder};
use midi_mapper::{jackmidi::MidiMsg, jackprocess::start_jack_thread, midi_function::MidiFunction};
use std::{
    collections::{HashMap, HashSet},
    sync::mpsc,
};
mod midi_elements_gui;
use midi_elements_gui::MidiElementsGui;

fn main() {
    let (midi_sender, midi_receiver): (
        std::sync::mpsc::SyncSender<Box<dyn MidiMsg>>,
        std::sync::mpsc::Receiver<Box<dyn MidiMsg>>,
    ) = mpsc::sync_channel(64);
    let (tx_close, rx_close) = unbounded();
    let jack_midi_thread = start_jack_thread(rx_close, midi_sender);
    let midi_functions: HashSet<MidiFunction> = HashSet::from([
        MidiFunction::Volume,
        MidiFunction::Modulate,
        MidiFunction::FmIntensity,
    ]);
    let mut midi_functions_with_elements_ids: HashMap<MidiFunction, Vec<u16>> = HashMap::new();
    midi_functions_with_elements_ids.insert(MidiFunction::Volume, Vec::new());
    midi_functions_with_elements_ids.insert(MidiFunction::Modulate, Vec::new());
    midi_functions_with_elements_ids.insert(MidiFunction::FmIntensity, Vec::new());
    let midi_elements_gui = MidiElementsGui {
        midi_receiver: Some(midi_receiver),
        midi_thread: Some(jack_midi_thread),
        tx_close: Some(tx_close),
        n_items: 0,
        midi_functions,
        midi_functions_with_elements_ids,
        midi_elements_map: HashMap::new(),
        selected_midi_function: None,
        last_midi_msg: None,
        num_detected_midi_ids: 0,
    };

    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default().with_inner_size([600.0, 600.0]),
        ..Default::default()
    };

    let _ = eframe::run_native(
        "MidiDebuggerGui",
        options,
        Box::new(|_cc| Box::new(midi_elements_gui)),
    );
}
