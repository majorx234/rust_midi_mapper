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
    jackmidi::{MidiMsgAdvanced, MidiMsgBase},
    midi_egui_elements::midi_id_value_indicator,
    midi_egui_elements::{midi_id_double_precision_value_indicator, midi_note_status_indicator},
    midi_function::MidiFunction,
};
use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

pub struct MidiElementsGui {
    pub midi_receiver: Option<std::sync::mpsc::Receiver<Box<dyn MidiMsgBase>>>,
    pub midi_thread: Option<std::thread::JoinHandle<()>>,
    pub tx_close: Option<crossbeam_channel::Sender<bool>>,
    pub n_items: usize,
    pub midi_functions: HashSet<MidiFunction>,
    pub midi_functions_with_elements_ids: HashMap<String, Vec<MidiMsgAdvanced>>,
    pub midi_elements_map: HashMap<u16, (usize, MidiMsgAdvanced)>,
    pub selected_midi_function: Option<String>,
    pub last_midi_msg: Option<Box<dyn MidiMsgBase>>,
    pub num_detected_midi_ids: usize,
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
            last_midi_msg: None,
            num_detected_midi_ids: 0,
        }
    }
}

impl eframe::App for MidiElementsGui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Some(ref midi_receiver) = self.midi_receiver {
            while let Ok(current_midi_msg) = midi_receiver.try_recv() {
                let midi_advanced_msg = MidiMsgAdvanced::from_current_and_last_opt_midi_msgs((
                    current_midi_msg,
                    &mut self.last_midi_msg,
                ));
                if let Some(midi_advanced_msg) = midi_advanced_msg {
                    let id = midi_advanced_msg.get_id();
                    if let Some((detection_number, _)) = self.midi_elements_map.get(&id) {
                        self.midi_elements_map
                            .insert(id, (*detection_number, midi_advanced_msg));
                    } else {
                        self.midi_elements_map
                            .insert(id, (self.num_detected_midi_ids, midi_advanced_msg));
                        self.num_detected_midi_ids += 1;
                    }
                }
            }
        }
        egui::TopBottomPanel::top("control").show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.heading("MidiElementsGui");
                ui.horizontal(|ui| {
                    if ui.button("close").clicked() {
                        if let Some(x) = &self.tx_close {
                            x.send(false).unwrap();
                            ctx.send_viewport_cmd(ViewportCommand::Close)
                        };
                    }
                    let mut json_path = "".to_string();
                    let _ = ui.add(egui::TextEdit::singleline(&mut json_path));
                    if ui.button("export as json").clicked() {
                        let _path = PathBuf::from(json_path);
                        let json_output =
                            serde_json::to_string(&self.midi_functions_with_elements_ids).unwrap();
                        println!("{}", json_output);
                    }
                });
            });
        });
        // show list of offered midi functions
        egui::CentralPanel::default().show(ctx, |ui| {
            for midi_function in self.midi_functions.iter() {
                if ui.button(format!("{}", midi_function)).clicked() {
                    let midi_function_name = midi_function.get_name();
                    self.selected_midi_function = Some(midi_function_name);
                }
            }
        });
        let len_hashmap = self.midi_elements_map.len();
        self.n_items = self.midi_elements_map.len();
        let mut midi_elements_vec: Vec<(u16, MidiMsgAdvanced)> =
            vec![(0, MidiMsgAdvanced::MidiEmpty); len_hashmap];
        for (key, (detection_number, midi_advanced_messsage)) in self.midi_elements_map.iter() {
            midi_elements_vec[*detection_number] = (*key, midi_advanced_messsage.clone());
        }

        // show list of Midi events
        egui::SidePanel::right("midi id events").show(ctx, |ui| {
            let window_rect = ctx.input(|i| i.viewport().outer_rect).unwrap();
            let window_width = window_rect.width();
            let window_height = window_rect.height();

            ui.vertical(|ui| {
                // self.n_items = self.midi_elements_map.len();
                let text_style = egui::TextStyle::Body;
                let row_height = ui.text_style_height(&text_style);
                ScrollArea::vertical()
                    .stick_to_bottom(true)
                    .min_scrolled_height(window_height - 60.0)
                    .max_height(window_height - 60.0)
                    .min_scrolled_width(window_width - 40.0)
                    .max_width(window_width - 40.0)
                    .show_rows(ui, row_height, self.n_items, |ui, row_range| {
                        for (row, (_key, midi_advanced_msg)) in midi_elements_vec.iter().enumerate()
                        {
                            if row_range.contains(&row) {
                                if let MidiMsgAdvanced::MidiNoteOnOff(
                                    id0,
                                    id1,
                                    value,
                                    note,
                                    intensity,
                                ) = midi_advanced_msg
                                {
                                    if ui
                                        .add(midi_note_status_indicator(*id0 as u32, &value))
                                        .clicked()
                                    {
                                        if let Some(ref selected_midi_function) =
                                            self.selected_midi_function
                                        {
                                            if let Some(ref mut midi_elements_id) = self
                                                .midi_functions_with_elements_ids
                                                .get_mut(selected_midi_function)
                                            {
                                                let midi_advancecd_msg =
                                                    MidiMsgAdvanced::MidiNoteOnOff(
                                                        *id0, *id1, false, *note, *intensity,
                                                    );
                                                midi_elements_id.push(midi_advancecd_msg);
                                            }
                                        }
                                    }
                                };

                                if let MidiMsgAdvanced::MidiControlIdValue(id, value) =
                                    midi_advanced_msg
                                {
                                    if ui
                                        .add(midi_id_value_indicator(*id as u32, *value as u32))
                                        .clicked()
                                    {
                                        if let Some(ref selected_midi_function) =
                                            self.selected_midi_function
                                        {
                                            if let Some(ref mut midi_elements_id) = self
                                                .midi_functions_with_elements_ids
                                                .get_mut(selected_midi_function)
                                            {
                                                let midi_advancecd_msg =
                                                    MidiMsgAdvanced::MidiControlIdValue(*id, 0);
                                                midi_elements_id.push(midi_advancecd_msg);
                                            }
                                        }
                                    }
                                }
                                if let MidiMsgAdvanced::MidiControl2IdsValue(id0, id1, value) =
                                    midi_advanced_msg
                                {
                                    if ui
                                        .add(midi_id_double_precision_value_indicator(
                                            *id0 as u32,
                                            *value as u32,
                                        ))
                                        .clicked()
                                    {
                                        if let Some(ref selected_midi_function) =
                                            self.selected_midi_function
                                        {
                                            if let Some(ref mut midi_elements_id) = self
                                                .midi_functions_with_elements_ids
                                                .get_mut(selected_midi_function)
                                            {
                                                let midi_advancecd_msg =
                                                    MidiMsgAdvanced::MidiControl2IdsValue(
                                                        *id0, *id1, 0,
                                                    );
                                                midi_elements_id.push(midi_advancecd_msg);
                                            }
                                        }
                                    }
                                }
                            } else {
                                println!("row error");
                            }
                        }
                    });
                ui.ctx().request_repaint();
            });
        });
        // show list of midi events of selected midi function
        egui::SidePanel::right("midi function with ids events").show(ctx, |ui| {
            if let Some(ref midi_function) = self.selected_midi_function {
                if let Some(ref mut selected_midi_events) =
                    self.midi_functions_with_elements_ids.get_mut(midi_function)
                {
                    let mut index_to_remove: Option<usize> = None;
                    for (idx, event) in selected_midi_events.iter().enumerate() {
                        if ui.button(format!("{}", *event)).clicked() {
                            index_to_remove = Some(idx);
                        }
                    }
                    if let Some(index_to_remove) = index_to_remove {
                        selected_midi_events.remove(index_to_remove);
                    }
                }
            }
        });
    }
}
