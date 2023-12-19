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
use eframe;
use midi_mapper::{jackmidi::MidiMsg, jackprocess::start_jack_thread};
use std::sync::mpsc;
mod midi_debugger_gui;
use midi_debugger_gui::MidiDebuggerGui;

fn main() {
    let (midi_sender, midi_receiver): (
        std::sync::mpsc::SyncSender<Box<dyn MidiMsg>>,
        std::sync::mpsc::Receiver<Box<dyn MidiMsg>>,
    ) = mpsc::sync_channel(64);
    let (tx_close, rx_close) = unbounded();
    let jack_midi_thread = start_jack_thread(rx_close, midi_sender);
    let midi_debugger_gui = MidiDebuggerGui {
        midi_receiver: Some(midi_receiver),
        midi_thread: Some(jack_midi_thread),
        tx_close: Some(tx_close),
        n_items: 0,
        midi_msgs: Vec::new(),
    };

    let mut options = eframe::NativeOptions::default();
    let window_size: eframe::egui::Vec2 = eframe::egui::Vec2::new(350.0, 530.0);
    options.initial_window_size = Some(window_size);

    let _ = eframe::run_native(
        "MidiDebuggerGui",
        options,
        Box::new(|_cc| Box::new(midi_debugger_gui)),
    );
}
