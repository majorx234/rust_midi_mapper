use crossbeam_channel::unbounded;
use eframe::egui;
use midi_mapper::jackmidi::MidiMsg;

pub struct MidiDebuggerGui {
    pub midi_receiver: Option<std::sync::mpsc::Receiver<MidiMsg>>,
    pub midi_thread: Option<std::thread::JoinHandle<()>>,
    pub tx_close: Option<crossbeam_channel::Sender<bool>>,
}

impl Default for MidiDebuggerGui {
    fn default() -> Self {
        Self {
            midi_receiver: None,
            midi_thread: None,
            tx_close: None,
        }
    }
}

impl eframe::App for MidiDebuggerGui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Some(ref midi_receiver) = self.midi_receiver {
            if let Ok(m) = midi_receiver.try_recv() {
                let bytes: &[u8] = &m.data;
                if let Ok(message) = wmidi::MidiMessage::try_from(bytes) {
                    println!("{:?}", message);
                }
            }
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("MidiMapper");
            ui.vertical(|ui| {
                if ui.button("close").clicked() {
                    if let Some(x) = &self.tx_close {
                        x.send(false).unwrap();
                        _frame.quit();
                    };
                }
            });
        });
    }
}
