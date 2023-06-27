use std::sync::{Arc, RwLock};

use libclockrobustus::{alarm::{Alarm}, check_database_directory};

static mut CONN: Option<Arc<RwLock<sqlite::Connection>>> = None;

fn db_check_init() {
    unsafe {
        if CONN.is_none() {
            let db_path = check_database_directory().expect("Unable to check database directory");
            CONN = Some(Arc::new(RwLock::new(
                sqlite::Connection::open(db_path).expect("Unable to open sqlite connection"),
            )));
        }
    }
}

fn db_accessor<F, T>(mut callback: F) -> Option<T>
where
    F: FnMut(&sqlite::Connection) -> T,
{
    db_check_init();
    unsafe {
        if let Some(arconn) = &CONN {
            let arcc = arconn.clone();
            let conn = arcc
                .read()
                .expect("Unable to obtain lock for database connection");

            Some(callback(&conn))
        } else {
            None
        }
    }
}

#[tauri::command]
pub fn get_alarms() -> Vec<Alarm> {
    db_accessor(|conn| Alarm::all(conn).expect("Unable to retrieve alarms")).unwrap_or(vec![])
}

#[tauri::command]
pub fn upsert_alarm(alarm: Alarm) {
    db_accessor(move |conn| {
        alarm.save(conn).expect("Unable to save alarm");
    });
}

#[tauri::command]
pub fn delete_alarm(alarm: Alarm) {
    db_accessor(move |conn| {
        alarm.remove(conn).expect("Unable to delete alarm");
    });
}
