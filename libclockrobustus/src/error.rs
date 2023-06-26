use std::array::TryFromSliceError;
use std::env::VarError;
use std::fmt::Display;
use std::io;
use std::num::{IntErrorKind, ParseIntError};
use std::time::SystemTimeError;
/// Thread-safe error object that bridges before many of error types encountered during a typical
/// run  
/// Every [From] trait implementation also prints to stdout the details of each error it binds to.
#[derive(Debug)]
pub struct ClockError(pub &'static str);

impl std::error::Error for ClockError {}

impl Display for ClockError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<zmq::Error> for ClockError {
    fn from(value: zmq::Error) -> Self {
        println!("{:?}", value);
        match value {
            zmq::Error::EACCES => Self("ZMQ Error: No Access"),
            zmq::Error::EADDRINUSE => Self("ZMQ Error: Address in use"),
            zmq::Error::EAGAIN => Self("ZMQ Error: Would block"),
            zmq::Error::EBUSY => Self("ZMQ Error: Resource busy"),
            zmq::Error::ECONNREFUSED => Self("ZMQ Error: Connection refused"),
            zmq::Error::ENOTCONN => Self("ZMQ Error: Not connected"),
            zmq::Error::EADDRNOTAVAIL => Self("ZMQ Error: Address not available"),
            zmq::Error::EINVAL => Self("ZMQ Error: Invalid input"),
            zmq::Error::EINTR => Self("ZMQ Error: Interrput"),
            _ => Self("ZMQ Error"),
        }
    }
}

impl From<ParseIntError> for ClockError {
    fn from(value: ParseIntError) -> Self {
        println!("{:?}", value);
        match value.kind() {
            IntErrorKind::Empty => Self("Parse Int Error: Empty string"),
            IntErrorKind::InvalidDigit => Self("Parse Int Error: Invalid digit"),
            IntErrorKind::PosOverflow => Self("Parse Int Error: Too large"),
            IntErrorKind::NegOverflow => Self("Parse Int Error: Too small"),
            IntErrorKind::Zero => Self("Parse Int Error: Zero on non-zero type"),
            _ => Self("Parse Int Error"),
        }
    }
}

impl From<TryFromSliceError> for ClockError {
    fn from(_value: TryFromSliceError) -> Self {
        Self("Conversion from slice failed")
    }
}

impl From<SystemTimeError> for ClockError {
    fn from(_value: SystemTimeError) -> Self {
        Self("System time error")
    }
}

impl From<sqlite::Error> for ClockError {
    fn from(value: sqlite::Error) -> Self {
        println!("{:?}", value);
        Self("Database Error")
    }
}

impl From<io::Error> for ClockError {
    fn from(value: io::Error) -> Self {
        println!("{:?}", value);
        Self("IO Error")
    }
}

impl From<VarError> for ClockError {
    fn from(value: VarError) -> Self {
        println!("{:?}", value);
        Self("Env Var Error")
    }
}

impl From<ctrlc::Error> for ClockError {
    fn from(value: ctrlc::Error) -> Self {
        println!("{:?}", value);
        Self("Unable to setup Ctrl+C handler")
    }
}
