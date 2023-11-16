use crossbeam_channel::unbounded;
use eframe::egui::{self, ScrollArea};
use midi_mapper::jackmidi::MidiMsg;

pub struct MidiDebuggerGui {
    pub midi_receiver: Option<std::sync::mpsc::Receiver<MidiMsg>>,
    pub midi_thread: Option<std::thread::JoinHandle<()>>,
    pub tx_close: Option<crossbeam_channel::Sender<bool>>,
    pub n_items: usize,
    pub midi_msgs: Vec<MidiMsg>,
}

impl Default for MidiDebuggerGui {
    fn default() -> Self {
        Self {
            midi_receiver: None,
            midi_thread: None,
            tx_close: None,
            n_items: 0,
            midi_msgs: Vec::new(),
        }
    }
}

impl eframe::App for MidiDebuggerGui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut received_midi_msgs = Vec::new();
        if let Some(ref midi_receiver) = self.midi_receiver {
            while let Ok(m) = midi_receiver.try_recv() {
                received_midi_msgs.push(m);
            }
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("MidiDebugger");
            ui.vertical(|ui| {
                if ui.button("close").clicked() {
                    if let Some(x) = &self.tx_close {
                        x.send(false).unwrap();
                        _frame.quit();
                    };
                }

                self.midi_msgs.append(&mut received_midi_msgs);
                self.n_items = self.midi_msgs.len();
                let text_style = egui::TextStyle::Body;
                let row_height = ui.text_style_height(&text_style);
                ScrollArea::vertical().stick_to_bottom().show_rows(
                    ui,
                    row_height,
                    self.n_items,
                    |ui, row_range| {
                        for row in row_range {
                            if row > 0 {
                                let bytes: &[u8] = &self.midi_msgs[row - 1].data;
                                if let Ok(message) = wmidi::MidiMessage::try_from(bytes) {
                                    let text = format!("{:?}", message);
                                    ui.label(text);
                                }
                            }
                        }
                    },
                );

                self.n_items += 1;
                ui.ctx().request_repaint();
            });
        });
    }
}
