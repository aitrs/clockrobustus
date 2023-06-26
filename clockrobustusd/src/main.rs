use libclockrobustus::{
    alarm::Alarm, check_database_directory, clock::ClockMessage, env::ClockEnv, error::ClockError,
};
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread::sleep,
    time::Duration,
};

/// Tick function. Checks alarms and generates the clock signal.
/// (see libclockrobustus documentation for more explanations)
fn tick(socket: &zmq::Socket, conn: &sqlite::Connection) -> Result<(), ClockError> {
    // Fetching alarms
    let alarms = Alarm::all(conn)?;

    // Triggering relevant alarms
    for alarm in alarms {
        if alarm.must_ring()? {
            let msg = zmq::Message::from(alarm.as_bytes());

            socket.send(msg, 0)?;
        }
    }

    // Sending clockmessage.
    let clock_message = zmq::Message::from(ClockMessage::default().as_bytes());
    socket.send(clock_message, 0)?;

    Ok(())
}

fn main() -> Result<(), ClockError> {
    // Initializations (using an arc to concurrently tell the main loop to break if Ctlr+C is
    // pressed)
    let running = Arc::new(AtomicBool::new(true));
    let rc = running.clone();
    let db_path = check_database_directory()?;
    let env = ClockEnv::new()?;
    let zmq_context = zmq::Context::new();
    let socket = zmq_context.socket(zmq::PUB)?;
    let conn = sqlite::Connection::open(db_path)?;

    socket.bind(&format!(
        "tcp://{}:{}",
        env.queue().host(),
        env.queue().port(),
    ))?;

    ctrlc::set_handler(move || {
        println!("Interrupt, gracefully shutting down the service");
        rc.store(false, Ordering::SeqCst);
    })?;

    // Server mode = endless loop
    loop {
        if !running.load(Ordering::SeqCst) {
            break;
        }

        if let Err(error) = tick(&socket, &conn) {
            println!("Encountered an error during tick : {:?}", error);
            println!("Please check your configuration !");
            println!("Still running");
        }
        // Take a breath
        sleep(Duration::from_millis(env.constants().tick_duration()));
    }

    println!("zzzzZZZZZzzzzz");
    Ok(())
}
