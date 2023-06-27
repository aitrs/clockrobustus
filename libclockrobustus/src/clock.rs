use crate::error::ClockError;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::f32::consts::PI;
/// A fully, minimal sized clock definition, serializable and deserializable (with [serde]),
/// and fully integrated in the ZeroMQ workflow. It synchronizes with local time on initialization.
/// it also carries angles in radians to place clock hands on a circular clock dial (thus limiting
/// frontend computations).
///
/// # Examples
///
/// ```
/// use libclockrobustus::clock::ClockMessage;
///
/// let message = ClockMessage::default();
///
/// assert_eq!(message.as_bytes().len(), 15usize)
/// ```
#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ClockMessage {
    hours: u8,
    minutes: u8,
    seconds: u8,
    hours_angle: f32,
    minutes_angle: f32,
    seconds_angle: f32,
}

impl ClockMessage {
    /// Convert a [ClockMessage] to a vector of bytes
    /// Useful for message queuing (and for binary saving)
    ///
    /// # Examples
    ///
    /// ```
    /// use libclockrobustus::clock::ClockMessage;
    ///
    /// let bytes = ClockMessage::default().as_bytes();
    ///
    /// assert_eq!(bytes.len(), 15usize);
    /// ```
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut v = Vec::new();

        v.push(self.hours);
        v.push(self.minutes);
        v.push(self.seconds);
        v.append(&mut self.hours_angle.to_be_bytes().to_vec());
        v.append(&mut self.minutes_angle.to_be_bytes().to_vec());
        v.append(&mut self.seconds_angle.to_be_bytes().to_vec());

        v
    }
}

impl TryFrom<Vec<u8>> for ClockMessage {
    type Error = ClockError;
    /// Initialize a [ClockMessage] from a binary vector
    ///
    /// # Examples
    /// ```
    /// use libclockrobustus::clock::ClockMessage;
    ///
    /// let message1 = ClockMessage::default();
    /// let bytes = message1.as_bytes();
    /// let message2 = ClockMessage::try_from(bytes).unwrap();
    ///
    /// assert_eq!(message1, message2);
    /// ```
    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        Ok(Self {
            hours: value[0],
            minutes: value[1],
            seconds: value[2],
            hours_angle: f32::from_be_bytes(value[3..7].try_into()?),
            minutes_angle: f32::from_be_bytes(value[7..11].try_into()?),
            seconds_angle: f32::from_be_bytes(value[11..15].try_into()?),
        })
    }
}

impl Default for ClockMessage {
    /// Default initializer for [ClockMessage], synchronizes to current local time.
    fn default() -> Self {
        let now = Local::now();
        let hours = now.hour() as u8;
        let minutes = now.minute() as u8;
        let seconds = now.second() as u8;

        Self {
            hours,
            minutes,
            seconds,
            hours_angle: Self::h24_to_radians(hours, minutes),
            minutes_angle: Self::ms60_to_radians(minutes, Some(seconds)),
            seconds_angle: Self::ms60_to_radians(seconds, None),
        }
    }
}

impl ClockMessage {
    /// Internal initialization handy method for hour hand angle computation (in radians)
    fn h24_to_radians(hours: u8, minutes: u8) -> f32 {
        let minute_arc = (minutes as f32) * PI / 360f32;
        let hour_arc = PI / 2f32 + (PI * (hours % 12) as f32) / 6f32;

        minute_arc + hour_arc
    }

    /// Internal initialization handy method for minutes and seconds hand angle computation (in
    /// radians)
    fn ms60_to_radians(value: u8, arc: Option<u8>) -> f32 {
        let arc = (arc.unwrap_or(0) as f32) * PI / 1800f32;
        let angle = PI / 2f32 + (PI * (value % 60) as f32) / 30f32;

        angle + arc
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // Tolerance between expected value and actual value in non-integer based tests.
    const TOLERANCE: f32 = 0.0000001f32;
    /// Handy function to compare the difference with the expected value and
    /// verify if it's under de specified above TOLERANCE
    fn tolerance_delta(actual_value: f32, expected_value: f32) {
        assert!(expected_value - actual_value < TOLERANCE)
    }

    #[test]
    fn test_clockmessage_h24_to_radians() {
        let test_cases = vec![
            // Tests when angle is 0 rad (eg: 9h00 in the first line below)
            (9, 0, 0f32),
            (21, 0, 0f32),
            // Tests when angle is PI/2 rad
            (0, 0, PI / 2f32),
            (12, 0, PI / 2f32),
            // Tests when angle is PI rad
            (3, 0, PI / 2f32),
            (15, 0, PI / 2f32),
            // Tests when angle si 3PI/2 rad
            (6, 0, 3f32 * PI / 2f32),
            (18, 0, 3f32 * PI / 2f32),
            // Tests when angle is PI/12
            (10, 0, PI / 12f32),
            (22, 0, PI / 12f32),
            // Tests when angle is PI/2 + PI/12
            (1, 0, 7f32 * PI / 12f32),
            (13, 0, 7f32 * PI / 12f32),
            // Minute arc test (due to linearity, we consider all to be correct if this test
            // passes)
            // 11h60 = 12h => PI/2 expected
            (11, 60, PI / 2f32),
        ];

        // for each test case, check the delta between expected and actual values.
        for (hours, minutes, expected_value) in test_cases {
            tolerance_delta(ClockMessage::h24_to_radians(hours, minutes), expected_value);
        }
    }

    #[test]
    fn test_clockmessage_ms60_to_radians() {
        let test_cases = vec![
            (45, 0f32),     // 0 rads
            (0, PI / 2f32), // as above... etc...
            (15, PI),
            (30, 3f32 * PI / 2f32),
            (46, PI / 60f32), // First minute arc in rads
        ];

        // Same as above !
        for (value, expected_value) in test_cases {
            tolerance_delta(ClockMessage::ms60_to_radians(value, None), expected_value);
        }
    }

    #[test]
    fn test_clockmessage_binary_convertion() {
        // Doing the conversion back and forth and testing equality.
        let message1 = ClockMessage::default();
        let bytes = message1.as_bytes();
        let message2 = ClockMessage::try_from(bytes).unwrap();

        assert_eq!(message1, message2);
    }
}
