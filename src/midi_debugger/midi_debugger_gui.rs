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
use eframe::egui::{self, ScrollArea};
use midi_mapper::jackmidi::MidiMsg;

pub struct MidiDebuggerGui {
    pub midi_receiver: Option<std::sync::mpsc::Receiver<Box<dyn MidiMsg>>>,
    pub midi_thread: Option<std::thread::JoinHandle<()>>,
    pub tx_close: Option<crossbeam_channel::Sender<bool>>,
    pub n_items: usize,
    pub midi_msgs: Vec<Box<dyn MidiMsg>>,
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
        let window_size = _frame.info().window_info.size;
        let window_width = window_size[0];
        let window_height = window_size[1];
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("MidiDebugger");
            ui.vertical(|ui| {
                if ui.button("close").clicked() {
                    if let Some(x) = &self.tx_close {
                        x.send(false).unwrap();
                        _frame.close();
                    };
                }

                self.midi_msgs.append(&mut received_midi_msgs);
                self.n_items = self.midi_msgs.len();
                let text_style = egui::TextStyle::Body;
                let row_height = ui.text_style_height(&text_style);
                ScrollArea::vertical()
                    .stick_to_bottom(true)
                    .min_scrolled_height(window_height)
                    .min_scrolled_width(window_width)
                    .show_rows(ui, row_height, self.n_items, |ui, row_range| {
                        for row in row_range {
                            if row > 0 {
                                let bytes: &[u8] = &self.midi_msgs[row - 1].get_data();
                                if let Ok(message) = wmidi::MidiMessage::try_from(bytes) {
                                    let text = format!("{:?}", message);
                                    ui.label(text);
                                }
                            }
                        }
                    });

                self.n_items += 1;
                ui.ctx().request_repaint();
            });
        });
    }
}
