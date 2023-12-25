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
use midi_mapper::{
    jackmidi::MidiMsg, midi_egui_elements::midi_id_value_indicator, midi_function::MidiFunction,
};
use std::collections::{HashMap, HashSet};

pub struct MidiElementsGui {
    pub midi_receiver: Option<std::sync::mpsc::Receiver<Box<dyn MidiMsg>>>,
    pub midi_thread: Option<std::thread::JoinHandle<()>>,
    pub tx_close: Option<crossbeam_channel::Sender<bool>>,
    pub n_items: usize,
    pub midi_functions: HashSet<MidiFunction>,
    pub midi_functions_with_elements_ids: HashMap<MidiFunction, u16>,
    pub midi_elements_map: HashMap<u16, u16>,
    pub selected_midi_function: Option<MidiFunction>,
}

impl Default for MidiElementsGui {
    fn default() -> Self {
        Self {
            midi_receiver: None,
            midi_thread: None,
            tx_close: None,
            n_items: 0,
            midi_functions: HashSet::new(),
            midi_functions_with_elements_ids: HashMap::new(),
            midi_elements_map: HashMap::new(),
            selected_midi_function: None,
        }
    }
}

impl eframe::App for MidiElementsGui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut received_midi_msgs = Vec::new();
        if let Some(ref midi_receiver) = self.midi_receiver {
            while let Ok(m) = midi_receiver.try_recv() {
                received_midi_msgs.push(m);
            }
        }
        egui::TopBottomPanel::top("control").show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.heading("MidiElementsGui");
                if ui.button("close").clicked() {
                    if let Some(x) = &self.tx_close {
                        x.send(false).unwrap();
                        ctx.send_viewport_cmd(ViewportCommand::Close)
                    };
                }
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("nothing here yet!");
        });
        egui::SidePanel::right("midi id events").show(ctx, |ui| {
            let window_rect = ctx.input(|i| i.viewport().outer_rect).unwrap();
            let window_width = window_rect.width();
            let window_height = window_rect.height();

            ui.vertical(|ui| {
                for msg in received_midi_msgs {
                    let id = msg.get_id();
                    let value = msg.get_value();
                    self.midi_elements_map.insert(id, value);
                }

                self.n_items = self.midi_elements_map.len();
                let text_style = egui::TextStyle::Body;
                let row_height = ui.text_style_height(&text_style);
                ScrollArea::vertical()
                    .stick_to_bottom(true)
                    .min_scrolled_height(window_height - 60.0)
                    .max_height(window_height - 60.0)
                    .min_scrolled_width(window_width - 40.0)
                    .max_width(window_width - 40.0)
                    .show_rows(ui, row_height, self.n_items, |ui, row_range| {
                        for (row, (key, value)) in self.midi_elements_map.iter().enumerate() {
                            if row_range.contains(&row) {
                                ui.add(midi_id_value_indicator(*key as u32, *value as u32));
                            } else {
                                println!("row error");
                            }
                        }
                    });

                ui.ctx().request_repaint();
            });
        });
        egui::SidePanel::right("midi function with ids events").show(ctx, |ui| {
            ui.label("nothing here yet2!");
        });
    }
}
