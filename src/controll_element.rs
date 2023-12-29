use crate::midi_function::MidiFunction;
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Serialize, Deserialize, Copy, Clone, Debug)]
pub enum Scaling {
    LINEAR,
    LOGARITHMIC,
    EXPONETIONAL,
}

#[derive(Eq, PartialEq, Serialize, Deserialize, Copy, Clone, Debug)]
pub struct ControllElement {
    pub midi_function: MidiFunction,
    pub invert: bool,
    pub scaling: Option<Scaling>,
}
