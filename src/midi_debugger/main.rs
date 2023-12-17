use crossbeam_channel::unbounded;
use eframe;
use midi_mapper::{jackmidi::MidiMsg, jackprocess::start_jack_thread};
use std::sync::mpsc;
use wmidi;
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

    eframe::run_native(
        "MidiDebuggerGui",
        options,
        Box::new(|_cc| Box::new(midi_debugger_gui)),
    );
}
