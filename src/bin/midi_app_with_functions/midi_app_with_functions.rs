use midi_mapper::jackmidi::{MidiMsg, MidiMsgAdvanced};
use std::collections::HashMap;

pub fn basic_loop(
    function_msgs_map: HashMap<String, Vec<MidiMsgAdvanced>>,
    midi_thread: Option<std::thread::JoinHandle<()>>,
    tx_close: Option<crossbeam_channel::Sender<bool>>,
    midi_receiver: Option<std::sync::mpsc::Receiver<Box<dyn MidiMsg>>>,
) {
    let mut last_midi_msg: Option<Box<dyn MidiMsg>> = None;
    if let Some(ref midi_receiver) = midi_receiver {
        while let Ok(current_midi_msg) = midi_receiver.try_recv() {
            let midi_advanced_msg = MidiMsgAdvanced::from_current_and_last_opt_midi_msgs((
                current_midi_msg,
                &mut last_midi_msg,
            ));
            if let Some(midi_advanced_msg) = midi_advanced_msg {
                let id = midi_advanced_msg.get_id();
            }
        }
    }
}
