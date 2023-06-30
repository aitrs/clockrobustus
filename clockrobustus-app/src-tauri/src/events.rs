use libclockrobustus::{message::Message, queue::listen};
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
        let running = Arc::new(AtomicBool::new(true));
        let rc = running.clone();
        // Stop handler
        let stop_handler = window.once("STOP", move |_| rc.store(false, Ordering::SeqCst));

        listen(running, |message| match message {
            Message::Alarm(alarm) => window
                .emit("ALARM", alarm)
                .expect("Unable to send ALARM event to window"),
            Message::Clock(clock_message) => window
                .emit("CLOCK", clock_message)
                .expect("Unable to send CLOCK event to window"),
        })
        .expect("Unable to listen on client side");

        window.unlisten(stop_handler);
    });
}
