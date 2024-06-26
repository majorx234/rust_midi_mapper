use midi_mapper::jackmidi::{MidiMsgAdvanced, MidiMsgBase};
use std::collections::HashMap;

pub fn basic_loop(
    function_msgs_map: HashMap<String, Vec<MidiMsgAdvanced>>,
    _midi_thread: Option<std::thread::JoinHandle<()>>,
    _tx_close: Option<crossbeam_channel::Sender<bool>>,
    midi_receiver: Option<std::sync::mpsc::Receiver<Box<dyn MidiMsgBase>>>,
) {
    // creat a reverse Hashmap
    let mut midi_advanced_msgs2midi_functions: HashMap<MidiMsgAdvanced, Vec<String>> =
        HashMap::new();
    for (key, value_vec) in function_msgs_map {
        let key_insert = key.clone();
        for value in value_vec {
            if let Some(ref mut midi_function_vec) =
                midi_advanced_msgs2midi_functions.get_mut(&value)
            {
                midi_function_vec.push(key_insert.clone());
            } else {
                midi_advanced_msgs2midi_functions.insert(value, vec![key_insert.clone()]);
            }
        }
    }
    let mut last_midi_msg: Option<Box<dyn MidiMsgBase>> = None;
    let mut _run_loop = true;
    while _run_loop {
        let mut received_midi_advanced_messages: Vec<MidiMsgAdvanced> = Vec::new();
        // check buffer for new message
        if let Some(ref midi_receiver) = midi_receiver {
            while let Ok(current_midi_msg) = midi_receiver.try_recv() {
                let midi_advanced_msg = MidiMsgAdvanced::from_current_and_last_opt_midi_msgs((
                    current_midi_msg,
                    &mut last_midi_msg,
                ));
                if let Some(midi_advanced_msg) = midi_advanced_msg {
                    let _id = midi_advanced_msg.get_id();
                    received_midi_advanced_messages.push(midi_advanced_msg);
                }
            }
        }
        for midi_advanced_msg in received_midi_advanced_messages {
            if let Some(functions) = midi_advanced_msgs2midi_functions.get(&midi_advanced_msg) {
                for function in functions {
                    println!(
                        "function: {} called with msgs: {}",
                        function, midi_advanced_msg
                    );
                }
            }
        }
        // compare
    }
}
