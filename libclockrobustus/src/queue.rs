use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use crate::{env::ClockEnv, error::ClockError, message::Message};
/// Zmq listener. Handling incoming binary messages on client side,
/// converts them to [Message] items and passes them to a callback.
pub fn listen<F>(running_flag: Arc<AtomicBool>, callback: F) -> Result<(), ClockError>
where
    F: Fn(Message),
{
    let env = ClockEnv::new()?;
    let ctx = zmq::Context::new();
    let socket = ctx.socket(zmq::SUB)?;
    let mut msg = zmq::Message::new();

    socket.set_subscribe(b"")?;
    socket.connect(&format!(
        "tcp://{}:{}",
        env.queue().host(),
        env.queue().port(),
    ))?;

    loop {
        if !running_flag.load(Ordering::SeqCst) {
            break;
        }

        socket.recv(&mut msg, 0)?;

        let bytes = msg.iter().copied().collect::<Vec<u8>>();
        let message = Message::try_from(bytes)?;

        callback(message);
    }

    Ok(())
}
