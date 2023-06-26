// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use clockrobustus::{alarms, events};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            events::clock_events,
            alarms::get_alarms,
            alarms::upsert_alarm,
            alarms::delete_alarm,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
