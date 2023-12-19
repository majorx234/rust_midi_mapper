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

use std::convert::From;

const MAX_MIDI: usize = 3;

pub trait MidiMsg: Send {
    fn type_of(&self) -> &str;
    fn get_data(&self) -> Vec<u8>;
}

//a fixed size container to copy data out of real-time thread
#[derive(Copy, Clone)]
pub struct MidiMsgGeneric {
    pub len: usize,
    pub data: [u8; MAX_MIDI],
    pub time: jack::Frames,
}

impl MidiMsg for MidiMsgGeneric {
    fn type_of(&self) -> &str {
        "MidiMsgGeneric"
    }
    fn get_data(&self) -> Vec<u8> {
        self.data.into_iter().collect()
    }
}

pub struct MidiMsgControlChange {
    pub channel: u8,
    pub number: u8,
    pub value: u8,
    pub time: jack::Frames,
}

impl MidiMsg for MidiMsgControlChange {
    fn type_of(&self) -> &str {
        "MidiMsgControlChange"
    }
    fn get_data(&self) -> Vec<u8> {
        vec![self.channel, self.number, self.value]
    }
}

pub struct MidiMsgNoteOn {
    pub channel: u8,
    pub key: u8,
    pub velocity: u8,
    pub time: jack::Frames,
}

impl MidiMsg for MidiMsgNoteOn {
    fn type_of(&self) -> &str {
        "MidiMsgNoteOn"
    }
    fn get_data(&self) -> Vec<u8> {
        vec![self.channel, self.key, self.velocity]
    }
}

pub struct MidiMsgNoteOff {
    pub channel: u8,
    pub key: u8,
    pub velocity: u8,
    pub time: jack::Frames,
}

impl MidiMsg for MidiMsgNoteOff {
    fn type_of(&self) -> &str {
        "MidiMsgNoteOff"
    }
    fn get_data(&self) -> Vec<u8> {
        vec![self.channel, self.key, self.velocity]
    }
}

pub struct MidiMsgCC {
    pub channel: u8,
    pub control: u8,
    pub value: u8,
    pub time: jack::Frames,
}

impl MidiMsg for MidiMsgCC {
    fn type_of(&self) -> &str {
        "MidiMsgCC"
    }
    fn get_data(&self) -> Vec<u8> {
        vec![self.channel, self.control, self.value]
    }
}

impl From<jack::RawMidi<'_>> for Box<dyn MidiMsg> {
    fn from(midi: jack::RawMidi<'_>) -> Box<dyn MidiMsg> {
        let len = std::cmp::min(MAX_MIDI, midi.bytes.len());
        let mut data = [0; MAX_MIDI];
        data[..len].copy_from_slice(&midi.bytes[..len]);
        Box::new(MidiMsgGeneric {
            len,
            data,
            time: midi.time,
        })
    }
}

impl std::fmt::Debug for MidiMsgGeneric {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Midi {{ time: {}, len: {}, data: {:?} }}",
            self.time,
            self.len,
            &self.data[..self.len]
        )
    }
}
