use std::env;

use crate::error::ClockError;
/// Substructure related to queue data. Here to keep things tidy.
pub struct QueueEnv {
    port: u16,
    host: String,
}

impl QueueEnv {
    /// Read-only accessor.
    pub fn port(&self) -> u16 {
        self.port
    }

    /// Read-only accessor.
    pub fn host(&self) -> &str {
        &self.host
    }
}

/// Substructure related to constants data. Here to keep things tidy.
pub struct Constants {
    tick_duration: u64,
}

impl Constants {
    /// Read-only accessor.
    pub fn tick_duration(&self) -> u64 {
        self.tick_duration
    }
}

/// Environment, useful to retrieve default values or environment set ones  
///   
/// # Available env vars
///
/// - CLOCKROBUSTUS_INTERNAL_QUEUE_PORT: port for zeromq outgoing channel (defaults to 5555)
/// - CLOCKROBUSTUS_INTERNAL_QUEUE_HOST: host for zeromq outgoing channel (default to localhost)
/// - CLOCKROBUSTUS_TICK_DURATION_MS: tick duration for the clock server (defaults to 1000)
/// # Panics
///
/// The [ClockEnv] creation will panic if one of the numeric env values specified above is not
/// parseable as an integer.
///
/// # Examples
///
/// ```
/// use libclockrobustus::env::ClockEnv;
///
/// let env = ClockEnv::new().unwrap();
///
/// assert_eq!(env.queue().port(), 5555);
/// ```
pub struct ClockEnv {
    queue: QueueEnv,
    constants: Constants,
}

impl ClockEnv {
    pub fn new() -> Result<Self, ClockError> {
        Ok(ClockEnv {
            queue: QueueEnv {
                port: env::var("CLOCKROBUSTUS_INTERNAL_QUEUE_PORT")
                    .unwrap_or("5555".to_string())
                    .parse()?,
                host: env::var("CLOCKROBUSTUS_INTERNAL_QUEUE_HOST")
                    .unwrap_or("127.0.0.1".to_string()),
            },
            constants: Constants {
                tick_duration: env::var("CLOCKROBUSTUS_TICK_DURATION_MS")
                    .unwrap_or("1000".to_string())
                    .parse()?,
            },
        })
    }

    pub fn queue(&self) -> &QueueEnv {
        &self.queue
    }

    pub fn constants(&self) -> &Constants {
        &self.constants
    }
}

#[cfg(test)]
mod tests {
    use std::env::{remove_var, set_var};

    use super::*;

    fn clean_env() {
        remove_var("CLOCKROBUSTUS_INTERNAL_QUEUE_PORT");
        remove_var("CLOCKROBUSTUS_INTERNAL_QUEUE_HOST");
        remove_var("CLOCKROBUSTUS_TICK_DURATION_MS");
    }

    #[test]
    fn test_default_env() {
        clean_env();
        let env = ClockEnv::new().unwrap();

        assert_eq!(env.queue().port(), 5555u16);
        assert_eq!(env.queue().host(), "127.0.0.1");
        assert_eq!(env.constants().tick_duration(), 1000u64);
    }

    #[test]
    fn test_good_env() {
        set_var("CLOCKROBUSTUS_INTERNAL_QUEUE_PORT", "1234");
        set_var("CLOCKROBUSTUS_INTERNAL_QUEUE_HOST", "128.122.122.1");
        set_var("CLOCKROBUSTUS_TICK_DURATION_MS", "200");

        let env = ClockEnv::new().unwrap();

        assert_eq!(env.queue().host(), "128.122.122.1");
        assert_eq!(env.constants().tick_duration(), 200u64);
        assert_eq!(env.queue().port(), 1234u16);

        clean_env();
    }

    #[test]
    fn test_wrong_envs() {
        let wrong_envs = vec![
            // Env with unparseable port
            vec![
                ("CLOCKROBUSTUS_INTERNAL_QUEUE_HOST", "machine1"),
                ("CLOCKROBUSTUS_INTERNAL_QUEUE_PORT", "foobarbaz"),
                ("CLOCKROBUSTUS_TICK_DURATION_MS", "100"),
            ],
            // Env with unparseable tick duration
            vec![
                ("CLOCKROBUSTUS_INTERNAL_QUEUE_HOST", "machine2"),
                ("CLOCKROBUSTUS_INTERNAL_QUEUE_PORT", "1234"),
                ("CLOCKROBUSTUS_TICK_DURATION_MS", "foobazbar"),
            ],
        ];

        for env in wrong_envs {
            for (key, value) in &env {
                set_var(key, value);
            }

            let result = ClockEnv::new();

            assert!(result.is_err());
        }

        clean_env();
    }
}
