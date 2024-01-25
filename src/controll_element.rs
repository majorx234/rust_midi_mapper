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

use crate::midi_function::MidiFunction;
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Serialize, Deserialize, Copy, Clone, Debug)]
pub enum Scaling {
    LINEAR,
    LOGARITHMIC,
    EXPONENTIAL,
}

#[derive(Eq, PartialEq, Serialize, Deserialize, Clone, Debug)]
pub struct ControllElement {
    pub midi_function: MidiFunction,
    pub invert: bool,
    pub scaling: Option<Scaling>,
}
