/// Small, serializable and essential clock representations.
pub mod alarm;
pub mod clock;
pub mod env;
pub mod error;

/// Ties the whole thing to linux (and some other unix-likes).
/// Handy function to check if the database file exists (creates it otherwise)
/// TODO : Port it to other platforms...
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
