use crossbeam_channel::unbounded;
use eframe;
use midi_mapper::jackmidi::MidiMsg;
use std::sync::mpsc;
use wmidi;
mod midi_debugger_gui;
use midi_debugger_gui::MidiDebuggerGui;

fn main() {
    let (midi_sender, midi_receiver): (
        std::sync::mpsc::SyncSender<MidiMsg>,
        std::sync::mpsc::Receiver<MidiMsg>,
    ) = mpsc::sync_channel(64);
    let midi_debugger_gui = MidiDebuggerGui {
        midi_receiver: Some(midi_receiver),
    };
}
