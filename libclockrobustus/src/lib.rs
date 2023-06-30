/// Small, serializable and essential clock representations.
pub mod alarm;
pub mod clock;
pub mod env;
pub mod error;
pub mod message;
pub mod queue;

/// Handy function to check if the database file exists (creates it otherwise)
/// Unix version version
#[cfg(target_family = "unix")]
pub fn check_database_directory() -> Result<String, error::ClockError> {
    let home = std::env::var("HOME")?;
    let dbpath = format!("{}/.config/clockrobustus/dbase.sqlite", home);
    if !std::path::PathBuf::from(dbpath.clone()).exists() {
        std::process::Command::new("mkdir")
            .arg("-p")
            .arg(&format!("{}/.config/clockrobustus", home))
            .output()?;
        std::process::Command::new("touch")
            .arg(dbpath.clone())
            .output()?;
    }
    Ok(dbpath)
}

/// Version for Windows
#[cfg(target_family = "windows")]
pub fn check_database_directory() -> Result<String, error::ClockError> {
    let dbpath = "C:\\ProgramData\\ClockRobustus\\dbase.sqlite".to_string();

    if !std::path::PathBuf::from(dbpath.clone()).exists() {
        std::fs::create_dir_all("C:\\ProgramData\\ClockRobustus")?;
        std::fs::File::create(dbpath.clone())?;
    }

    Ok(dbpath)
}
