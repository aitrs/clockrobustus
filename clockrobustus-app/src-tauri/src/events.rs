use libclockrobustus::{alarm::Alarm, clock::ClockMessage, env::ClockEnv};
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
};
use tauri::Window;
/// Handler to retrieve events from zmq and to dispatch them to the frontend
#[tauri::command]
pub fn clock_events(window: Window) {
    // Spawning a thread to ensure the invoke method does not block !
    thread::spawn(move || {
        // Clock env initializations
        let env = ClockEnv::new().unwrap();
        let ctx = zmq::Context::new();
        let socket = ctx.socket(zmq::SUB).unwrap();
        let mut msg = zmq::Message::new();
        let running = Arc::new(AtomicBool::new(true));
        let rc = running.clone();
        // Stop handler
        let stop_handler = window.once("STOP", move |_| rc.store(false, Ordering::SeqCst));

        socket.set_subscribe(b"").unwrap();
        socket
            .connect(&format!(
                "tcp://{}:{}",
                env.queue().host(),
                env.queue().port()
            ))
            .unwrap();

        loop {
            if !running.load(Ordering::SeqCst) {
                break;
            }
            socket.recv(&mut msg, 0).unwrap();

            let bytes = msg.iter().copied().collect::<Vec<u8>>();

            // Check if we have an alarm or clock message according to it's first byte
            if bytes[0] == 0xff {
                let alarm = Alarm::from(bytes);

                window
                    .emit("ALARM", alarm)
                    .expect("Problem sending ALARM event");
            } else {
                let clock_message = ClockMessage::try_from(bytes).unwrap();

                window
                    .emit("CLOCK", clock_message)
                    .expect("Problem sending CLOCK event");
            }
        }

        window.unlisten(stop_handler);
    });
}
