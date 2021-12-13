use std::{env::current_dir, fs::File, io::Read};

pub use error::AoCError;

pub type Result<T> = std::result::Result<T, AoCError>;

mod error;

/// load input from specified day.
pub fn load(day: u64) -> String {
    let mut path = current_dir().unwrap();
    path.push(format!("src/bin/day{}/input", day));
    let mut file = File::open(path).unwrap();

    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    buf
}

/// handy function to set up general-used logger with fern.
pub fn setup_logger() -> Result<()> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        // .chain(std::io::stdout())
        .chain(fern::log_file("output.log")?)
        .apply()
        .unwrap();
    Ok(())
}
