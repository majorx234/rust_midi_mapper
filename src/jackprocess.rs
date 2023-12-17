extern crate jack;
extern crate wmidi;

use crate::jackmidi::MidiMsg;
use crossbeam_channel::unbounded;

use std::{process::exit, thread, time::Duration};

pub fn start_jack_thread(
    rx_close: crossbeam_channel::Receiver<bool>,
    midi_sender: std::sync::mpsc::SyncSender<Box<dyn MidiMsg>>,
) -> std::thread::JoinHandle<()> {
    std::thread::spawn(move || {
        let mut run: bool = true;
        let (client, _status) =
            jack::Client::new("midi mapper", jack::ClientOptions::NO_START_SERVER)
                .expect("No Jack server running\n");

        let sample_rate = client.sample_rate();
        // register ports
        let midi_in = client
            .register_port("midi_mapper_midi_in", jack::MidiIn::default())
            .unwrap();

        let mut frame_size = client.buffer_size() as usize;

        if client.set_buffer_size(frame_size as u32).is_ok() {
            // get frame size
            frame_size = client.buffer_size() as usize;
            println!(
                "client started with samplerate: {} and frame_size: {}",
                sample_rate, frame_size
            );
        } else {
            exit(-1);
        }
        let process_callback = move |_: &jack::Client, ps: &jack::ProcessScope| -> jack::Control {
            let show_p = midi_in.iter(ps);
            for e in show_p {
                let c: Box<dyn MidiMsg> = e.into();
                let _ = midi_sender.try_send(c);
            }
            jack::Control::Continue
        };
        let process = jack::ClosureProcessHandler::new(process_callback);
        let active_client = client.activate_async((), process).unwrap();

        while run {
            thread::sleep(Duration::from_millis(100));
            match rx_close.recv() {
                Ok(running) => run = running,
                Err(_) => run = false,
            }
        }
    })
}
