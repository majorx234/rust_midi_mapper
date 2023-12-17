use std::convert::From;

const MAX_MIDI: usize = 3;

pub trait MidiMsg: Send {
    fn type_of(&self) -> &str;
    fn get_data(&self) -> &[u8];
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
    fn get_data(&self) -> &[u8] {
        &self.data
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
