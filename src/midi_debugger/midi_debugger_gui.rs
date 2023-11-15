use crossbeam_channel::unbounded;
use eframe::egui;
use midi_mapper::jackmidi::MidiMsg;

pub struct MidiDebuggerGui {
    pub midi_receiver: Option<std::sync::mpsc::Receiver<MidiMsg>>,
}

impl Default for MidiDebuggerGui {
    fn default() -> Self {
        Self {
            midi_receiver: None,
        }
    }
}

impl eframe::App for MidiDebuggerGui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("MidiMapper");
        });
        if let Some(ref midi_receiver) = self.midi_receiver {
            if let Ok(m) = midi_receiver.recv() {
                let bytes: &[u8] = &m.data;
                if let Ok(message) = wmidi::MidiMessage::try_from(bytes) {
                    println!("{:?}", message);
                }
            }
        }
    }
}
