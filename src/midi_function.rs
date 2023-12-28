use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Hash, Eq, PartialEq, Serialize, Deserialize, Copy, Clone, Debug)]
pub enum MidiFunction {
    Play,
    Volume,
    Modulate,
    FmIntensity,
    // ToDo
}

impl fmt::Display for MidiFunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}
