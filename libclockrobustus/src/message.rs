use crate::{alarm::Alarm, clock::ClockMessage, error::ClockError};

const ALARM_MESSAGE_HEADER: u8 = 0xFF;
const CLOCK_MESSAGE_HEADER: u8 = 0xFE;
/// Wrapper enum around [ClockMessage] and [Alarm] to discriminate them as they are passed as binary data through the queues.
/// Adds a binary header code for each message type and permits conversion in both ways.
///
/// # Examples
/// ```
/// use libclockrobustus::{message::Message, clock::ClockMessage, alarm::{Alarm, ActiveDays}};
///
/// let clock_message = ClockMessage::default();
/// let alarm = Alarm {
///     id: None,
///     active_days: ActiveDays(0x01),
///     hour: 12,
///     minute: 0,
///     seconds: 0,
/// };
///
/// let message1 = Message::from(clock_message);
/// let message2 = Message::from(alarm);
///
/// assert_eq!(message1.as_bytes()[0], 0xFE);
/// assert_eq!(message2.as_bytes()[0], 0xFF);
/// ```
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Message {
    Clock(ClockMessage),
    Alarm(Alarm),
}

impl From<ClockMessage> for Message {
    fn from(value: ClockMessage) -> Self {
        Self::Clock(value)
    }
}

impl From<Alarm> for Message {
    fn from(value: Alarm) -> Self {
        Self::Alarm(value)
    }
}

impl TryFrom<Vec<u8>> for Message {
    type Error = ClockError;
    /// Try to instantiate a new [Message] using the passed binary vector/
    ///
    /// # Panics
    ///
    /// Panics if the passed vector is empty, if the header byte value is unknown or if any of the inner conversions fails
    ///
    /// # Examples
    /// ```
    /// use libclockrobustus::{message::Message, alarm::{Alarm, ActiveDays}, error::ClockError};
    ///
    /// let empty = vec![];
    /// let garbage = vec![0x01, 0x02];
    /// let good_header_but_empty_after = vec![0xFF];
    /// let good_header_but_garbage_after = vec![0xFF, 0x01];
    /// let good = vec![0xFF, 0x01, 12, 0, 0];
    ///
    /// let res_empty = Message::try_from(empty);
    /// let res_garbage = Message::try_from(garbage);
    /// let res_good_header_but_empty_after = Message::try_from(good_header_but_empty_after);
    /// let res_good_header_but_garbage_after = Message::try_from(good_header_but_garbage_after);
    /// let res_good = Message::try_from(good);
    ///
    /// assert!(res_empty.is_err());
    /// assert!(res_garbage.is_err());
    /// assert!(res_good_header_but_empty_after.is_err());
    /// assert!(res_good_header_but_garbage_after.is_err());
    /// assert_eq!(res_good.unwrap(), Message::from(Alarm {
    ///     id: None,
    ///     active_days: ActiveDays(0x01),
    ///     hour: 12,
    ///     minute: 0,
    ///     seconds: 0,
    /// }));
    /// ```
    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err(ClockError("Cannot convert message from empty byte vector"))
        } else {
            match value[0] {
                ALARM_MESSAGE_HEADER => Ok(Self::Alarm(Alarm::try_from(
                    value[1..value.len()].to_vec(),
                )?)),
                CLOCK_MESSAGE_HEADER => Ok(Self::Clock(ClockMessage::try_from(
                    value[1..value.len()].to_vec(),
                )?)),
                _ => Err(ClockError("Unknown message header")),
            }
        }
    }
}

impl Message {
    /// Convert a [Message] to a vector of bytes
    ///
    /// # Examples
    ///
    /// ```
    /// use libclockrobustus::{message::Message, clock::ClockMessage};
    ///
    /// let message = Message::from(ClockMessage::default());
    /// let bytes = message.as_bytes();
    ///
    /// assert_eq!(Message::try_from(bytes).unwrap(), message);
    pub fn as_bytes(&self) -> Vec<u8> {
        match self {
            Self::Alarm(alarm) => velcro::vec![ALARM_MESSAGE_HEADER, ..alarm.as_bytes(),],
            Self::Clock(clock) => velcro::vec![CLOCK_MESSAGE_HEADER, ..clock.as_bytes(),],
        }
    }
}
