use chrono::{Datelike, Duration, Local, NaiveTime, Weekday};
use serde::{de::Visitor, ser::SerializeSeq, Deserialize, Serialize};
use sqlite::State;

use crate::error::ClockError;
/// Extremely small memory footprint way to represent days of the week where an alarm is active.  
/// Serializes and Deserializes as an array of strings but uses a single byte to store data (not
/// true in the database representation but true in program memory).
///
/// # Examples
///
/// ```
/// use serde::{Serialize, Deserialize};
/// use libclockrobustus::alarm::ActiveDays;
///
/// let days = ActiveDays(0x01);
/// let json = serde_json::to_string(&days).unwrap();
/// assert_eq!(json, "[\"Monday\"]");
/// let days: ActiveDays = serde_json::from_str("[\"Monday\", \"Tuesday\"]").unwrap();
/// assert_eq!(days, ActiveDays(0x03));
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ActiveDays(pub u8);

impl ActiveDays {
    /// Used for code factorisation
    fn as_vec<T: Clone>(&self, src_vec: Vec<T>) -> Vec<T> {
        let mut days_vec = Vec::new();
        let mut mask = 0x01;

        for day in src_vec.iter().take(7) {
            if self.0 & mask > 0 {
                days_vec.push(day.clone());
            }

            mask <<= 1;
        }

        days_vec
    }

    /// Used for serialization
    pub(super) fn to_day_strings_vec(self) -> Vec<String> {
        let days_strings = vec![
            "Monday".to_string(),
            "Tuesday".to_string(),
            "Wednesday".to_string(),
            "Thursday".to_string(),
            "Friday".to_string(),
            "Saturday".to_string(),
            "Sunday".to_string(),
        ];

        self.as_vec(days_strings)
    }

    /// Handy method to convert an [ActiveDays] item to a vector for [chrono::Weekday] items  
    /// useful for comparison used in alarm triggering
    ///
    /// # Examples
    ///
    /// ```
    /// use libclockrobustus::alarm::ActiveDays;
    /// use chrono::Weekday;
    ///
    /// let ad = ActiveDays(0x03);
    /// assert_eq!(ad.to_weekdays(), vec![Weekday::Mon, Weekday::Tue]);
    pub fn to_weekdays(&self) -> Vec<Weekday> {
        let days_chrono = vec![
            Weekday::Mon,
            Weekday::Tue,
            Weekday::Wed,
            Weekday::Thu,
            Weekday::Fri,
            Weekday::Sat,
            Weekday::Sun,
        ];

        self.as_vec(days_chrono)
    }
}

impl Serialize for ActiveDays {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Serializing is easy, we have a method to transform this into a vector of strings, hence we're
        // serializing it as a vector.
        let vec = self.to_day_strings_vec();
        let mut seq = serializer.serialize_seq(Some(vec.len()))?;
        for e in vec {
            seq.serialize_element(&e)?;
        }
        seq.end()
    }
}

impl<'de> Deserialize<'de> for ActiveDays {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Deserializing is hard... We have to define a visitor to a sequence that will convert
        // days in the array into mapped values.
        struct ActiveDaysVisitor;

        impl<'de> Visitor<'de> for ActiveDaysVisitor {
            type Value = u8;

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut iteration = 0;
                let mut value = 0;
                loop {
                    if iteration > 7 {
                        break;
                    }
                    let elt = seq.next_element::<&str>()?;
                    if let Some(e) = elt {
                        // Bitwise affectation. Each bit stands for a day (except the last one)
                        value |= match e {
                            "Monday" => 0x01,
                            "Tuesday" => 0x02,
                            "Wednesday" => 0x04,
                            "Thursday" => 0x08,
                            "Friday" => 0x10,
                            "Saturday" => 0x20,
                            "Sunday" => 0x40,
                            _ => 0x00,
                        };
                    } else {
                        break;
                    }

                    iteration += 1;
                }

                Ok(value)
            }

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "string vec")
            }
        }

        let value = deserializer.deserialize_seq(ActiveDaysVisitor)?;

        Ok(ActiveDays(value))
    }
}

const TNAME: &str = "alarms";
/// Serializable, deserializable, writable in database structure to hold all necesary information
/// about alarms.
///
/// # Examples
///
/// ```
/// use libclockrobustus::alarm::{Alarm, ActiveDays};
/// use serde::Deserialize;
///
/// let json = "{
///     \"activeDays\": [
///         \"Monday\",
///         \"Tuesday\"
///     ],
///     \"hour\": 12,
///     \"minute\": 0,
///     \"seconds\": 0
/// }";
///
/// let alarm: Alarm = serde_json::from_str(json).unwrap();
///
/// assert_eq!(alarm, Alarm {
///     id: None,
///     active_days: ActiveDays(0x03),
///     hour: 12,
///     minute: 0,
///     seconds: 0,
/// });
/// ```
#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub struct Alarm {
    pub id: Option<i64>,
    pub active_days: ActiveDays,
    pub hour: u8,
    pub minute: u8,
    pub seconds: u8,
}

impl Alarm {
    /// Returns true if alarm is set in timespan between it's own defined time and one second
    /// later.
    ///
    /// # Panics
    ///
    /// Panics if the current alarm cannot be converted to [chrono::NaiveTime].
    pub fn must_ring(&self) -> Result<bool, ClockError> {
        let local = Local::now();
        let alarm_naive =
            NaiveTime::from_hms_opt(self.hour as u32, self.minute as u32, self.seconds as u32)
                .ok_or(ClockError("Could not create naive time for alarm"))?;
        if self.active_days.to_weekdays().contains(&local.weekday()) {
            let alarm_delta = local.time() - alarm_naive;
            if local.time() >= alarm_naive && alarm_delta < Duration::seconds(1) {
                Ok(true)
            } else {
                Ok(false)
            }
        } else {
            Ok(false)
        }
    }

    // Essential db check
    fn check_table(conn: &sqlite::Connection) -> Result<(), ClockError> {
        let query = "SELECT name FROM sqlite_master WHERE type='table' AND name = ?";
        if conn.prepare(query)?.into_iter().bind((1, TNAME))?.count() == 0 {
            let query = format!(
                "CREATE TABLE {} (
                id INTEGER PRIMARY KEY,
                active_days INTEGER NOT NULL,
                hour INTEGER NOT NULL,
                minute INTEGER NOT NULL,
                seconds INTEGER NOT NULL
                )",
                TNAME
            );
            conn.execute(query)?;
        }

        Ok(())
    }

    /// Saves the current clock using the given [sqlite::Connection]. Creates the table 'alarms' if
    /// not present.
    ///
    /// # Panics
    ///
    /// Panics if an SQL error has been encountered
    ///
    /// # Examples
    ///
    /// ```
    /// use libclockrobustus::alarm::{Alarm, ActiveDays};
    ///
    /// let alarm = Alarm {
    ///     id: None,
    ///     active_days: ActiveDays(0x01),
    ///     hour: 12,
    ///     minute: 0,
    ///     seconds: 0,
    /// };
    ///
    /// let conn = sqlite::open(":memory:").unwrap();
    ///
    /// assert!(alarm.save(&conn).is_ok());
    /// ```
    pub fn save(&self, conn: &sqlite::Connection) -> Result<(), ClockError> {
        Self::check_table(conn)?;
        if let Some(eid) = self.id {
            let query = format!(
                "UPDATE {} SET
                active_days = {},
                hour = {},
                minute = {},
                seconds = {}
                WHERE id = {}",
                TNAME, self.active_days.0, self.hour, self.minute, self.seconds, eid,
            );

            conn.execute(query)?;
        } else {
            let query = format!(
                "INSERT INTO {} (
                    active_days,
                    hour,
                    minute,
                    seconds
                ) VALUES (
                    {}, {}, {}, {}
                )",
                TNAME, self.active_days.0, self.hour, self.minute, self.seconds,
            );

            conn.execute(query)?;
        }
        Ok(())
    }

    /// Get all the alarms stored in database
    ///
    /// # Panics
    ///
    /// Panics if a SQL error is encountered
    ///
    /// # Examples
    ///
    /// ```
    /// use libclockrobustus::alarm::{Alarm, ActiveDays};
    ///
    /// let alarm = Alarm {
    ///     id: None,
    ///     active_days: ActiveDays(0x01),
    ///     hour: 12,
    ///     minute: 0,
    ///     seconds: 0,
    /// };
    ///
    /// let conn = sqlite::open(":memory:").unwrap();
    /// alarm.save(&conn).unwrap();
    ///
    /// let alarms = Alarm::all(&conn).unwrap();
    ///
    /// assert!(alarms.len() > 0);
    /// ```
    pub fn all(conn: &sqlite::Connection) -> Result<Vec<Self>, ClockError> {
        Self::check_table(conn)?;
        let query = format!("SELECT * FROM {}", TNAME);
        let mut res = Vec::new();
        let mut statement = conn.prepare(query)?;

        while let Ok(State::Row) = statement.next() {
            res.push(Alarm {
                id: Some(statement.read::<i64, _>("id")?),
                active_days: ActiveDays(statement.read::<i64, _>("active_days")? as u8),
                hour: statement.read::<i64, _>("hour")? as u8,
                minute: statement.read::<i64, _>("minute")? as u8,
                seconds: statement.read::<i64, _>("seconds")? as u8,
            })
        }

        Ok(res)
    }

    /// Removes a saved alarm
    ///
    /// # Panics
    ///
    /// Panics if the represented alarm has no id (eg: not saved).
    ///
    /// # Examples
    ///
    /// ```
    /// use libclockrobustus::alarm::{Alarm, ActiveDays};
    ///
    /// let alarm = Alarm {
    ///     id: None,
    ///     active_days: ActiveDays(0x01),
    ///     hour: 12,
    ///     minute: 0,
    ///     seconds: 0,
    /// };
    ///
    /// let conn = sqlite::open(":memory:").unwrap();
    /// alarm.save(&conn).unwrap();
    ///
    /// let alarms = Alarm::all(&conn).unwrap();
    ///
    /// assert!(alarms.len() > 0);
    /// assert!(alarms[0].remove(&conn).is_ok());
    ///
    /// let alarm3 = Alarm {
    ///     id: None,
    ///     active_days: ActiveDays(0x02),
    ///     hour: 12,
    ///     minute: 13,
    ///     seconds: 25,
    /// };
    ///
    /// assert!(alarm3.remove(&conn).is_err());
    /// ```
    pub fn remove(&self, conn: &sqlite::Connection) -> Result<(), ClockError> {
        Self::check_table(conn)?;

        let eid = self
            .id
            .ok_or(ClockError("Impossible to delete an unsaved alarm"))?;
        let query = format!("DELETE FROM {} WHERE id = {}", TNAME, eid);

        conn.execute(query)?;
        Ok(())
    }

    /// Binary representation of the alarm (to be used in a queue). It is differenciated from the
    /// time given by the clock as it starts with an 0xFF byte, so the app can know it must display
    /// an alarm ringing. (the 0xFF value is never reached by clock messages, since they start with
    /// the hour and it goes only up to 23 (0x17)
    ///
    /// # Examples
    ///
    /// ```
    /// use libclockrobustus::alarm::{Alarm, ActiveDays};
    ///
    /// let alarm = Alarm {
    ///     id: None,
    ///     active_days: ActiveDays(0x01),
    ///     hour: 12,
    ///     minute: 9,
    ///     seconds: 9,
    /// };
    ///
    /// assert_eq!(alarm.as_bytes(), vec![0xff, 0x01, 12, 9, 9]);
    /// ```
    pub fn as_bytes(&self) -> Vec<u8> {
        vec![
            0xff,
            self.active_days.0,
            self.hour,
            self.minute,
            self.seconds,
        ]
    }
}

impl From<Vec<u8>> for Alarm {
    fn from(value: Vec<u8>) -> Self {
        Self {
            id: None,
            active_days: ActiveDays(value[1]),
            hour: value[2],
            minute: value[3],
            seconds: value[4],
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::{Local, Timelike};
    use sqlite::Connection;

    use super::{ActiveDays, Alarm};

    #[test]
    fn test_must_ring() {
        let now = Local::now();
        let time = now.time();
        let alarm = Alarm {
            id: None,
            active_days: ActiveDays(0xFF),
            hour: time.hour() as u8,
            minute: time.minute() as u8,
            seconds: time.second() as u8,
        };

        assert!(alarm.must_ring().unwrap());

        let alarm = Alarm {
            id: None,
            active_days: ActiveDays(0x01),
            hour: ((time.hour() + 4) % 24) as u8,
            minute: time.minute() as u8,
            seconds: time.second() as u8,
        };

        assert!(!alarm.must_ring().unwrap());
    }

    #[test]
    fn test_saving() {
        let conn = Connection::open(":memory:").unwrap();
        let alarm = Alarm {
            id: None,
            active_days: ActiveDays(0xFF),
            hour: 12,
            minute: 0,
            seconds: 0,
        };
        // Create
        assert!(alarm.save(&conn).is_ok());

        let alarms = Alarm::all(&conn).unwrap();

        assert!(!alarms.is_empty());

        // Update
        let mut current_alarm = alarms[0];

        current_alarm.hour = 13;
        current_alarm.minute = 42;
        current_alarm.seconds = 22;

        assert!(current_alarm.save(&conn).is_ok());

        let alarms = Alarm::all(&conn).unwrap();
        // Update check
        assert_eq!(alarms[0], current_alarm);
    }

    #[test]
    fn test_binary_conversion() {
        let alarm = Alarm {
            id: None,
            active_days: ActiveDays(0x02),
            hour: 13,
            minute: 12,
            seconds: 9,
        };

        let alarm2 = Alarm::from(alarm.as_bytes());

        assert_eq!(alarm, alarm2);
    }
}
