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

use eframe::egui::{self, ScrollArea, ViewportCommand};
use midi_mapper::jackmidi::MidiMsgBase;

pub struct MidiDebuggerGui {
    pub midi_receiver: Option<std::sync::mpsc::Receiver<Box<dyn MidiMsgBase>>>,
    pub midi_thread: Option<std::thread::JoinHandle<()>>,
    pub tx_close: Option<crossbeam_channel::Sender<bool>>,
    pub n_items: usize,
    pub midi_msgs: Vec<Box<dyn MidiMsgBase>>,
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
            let window_rect = ctx.input(|i| i.viewport().outer_rect).unwrap();
            let window_width = window_rect.width();
            let window_height = window_rect.height();
            ui.heading("MidiDebugger");
            ui.vertical(|ui| {
                if ui.button("close").clicked() {
                    if let Some(x) = &self.tx_close {
                        x.send(false).unwrap();
                        ctx.send_viewport_cmd(ViewportCommand::Close)
                    };
                }

                if ui.button("clear").clicked() {
                    // TODO clear
                    self.n_items = 0;
                    self.midi_msgs.clear();
                }

                self.midi_msgs.append(&mut received_midi_msgs);
                self.n_items = self.midi_msgs.len();
                let text_style = egui::TextStyle::Body;
                let row_height = ui.text_style_height(&text_style);
                if self.n_items > 0 {
                    ScrollArea::vertical()
                        .stick_to_bottom(true)
                        .min_scrolled_height(window_height - 80.0)
                        .max_height(window_height - 80.0)
                        .min_scrolled_width(window_width - 60.0)
                        .max_width(window_width - 60.0)
                        .show_rows(ui, row_height, self.n_items, |ui, row_range| {
                            for row in row_range {
                                let text = format!("{}", self.midi_msgs[row]);
                                ui.label(text);
                            }
                        });
                }
            });
        });
        ctx.request_repaint();
    }
}
