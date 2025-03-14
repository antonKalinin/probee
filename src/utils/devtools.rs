use std::backtrace::Backtrace;
use std::env;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::panic::PanicHookInfo;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn panic_gracefully(panic_info: &PanicHookInfo) {
    // Create logs directory if it doesn't exist
    let mut log_path;

    // Try to get home directory without using the dirs crate
    let home_dir = env::var("HOME")
        .or_else(|_| env::var("USERPROFILE"))
        .ok()
        .map(PathBuf::from);

    if let Some(home) = home_dir {
        log_path = home.join("Library/Logs/CommandI");
        std::fs::create_dir_all(&log_path).unwrap_or_else(|_| {
            // Fallback to temp directory if home directory isn't available
            log_path = env::temp_dir().join("CommandI/Logs");
            std::fs::create_dir_all(&log_path).unwrap_or(());
        });
    } else {
        // Fallback to temp directory if home directory isn't available
        log_path = env::temp_dir().join("CommandI/Logs");
        std::fs::create_dir_all(&log_path).unwrap_or(());
    }

    // Generate timestamp for the log file name
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let log_file = log_path.join(format!("crash_{}.log", timestamp));

    // Try to open or create the log file
    let mut file = match OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(&log_file)
    {
        Ok(file) => file,
        Err(_) => {
            // If we can't open the specified log file, try using a temporary file as fallback
            let fallback = env::temp_dir().join(format!("crash_{}.log", timestamp));
            File::create(fallback).unwrap_or_else(|_| {
                // This is our last resort - we can't do much if this fails
                panic!("Failed to create crash log file");
            })
        }
    };

    // Format current time in a human-readable format
    let now = SystemTime::now();
    let datetime = now.duration_since(UNIX_EPOCH).unwrap_or_default();
    let secs = datetime.as_secs();

    // Log basic system info
    let _ = writeln!(file, "====== CRASH REPORT ======");
    let _ = writeln!(file, "Time: {} seconds since epoch", secs);
    let _ = writeln!(file, "App version: {}", env!("CARGO_PKG_VERSION"));
    #[cfg(target_os = "macos")]
    {
        // Get macOS version
        use std::process::Command;
        if let Ok(output) = Command::new("sw_vers").output() {
            if let Ok(output_str) = String::from_utf8(output.stdout) {
                let _ = writeln!(file, "System info: {}", output_str);
            }
        }
    }

    // Log panic information
    let _ = writeln!(file, "\n====== PANIC INFO ======");
    let _ = writeln!(file, "Panic message: {}", panic_info);

    if let Some(location) = panic_info.location() {
        let _ = writeln!(
            file,
            "Panic occurred in file '{}' at line {}",
            location.file(),
            location.line()
        );
    }

    // Log backtrace using std::backtrace
    let _ = writeln!(file, "\n====== BACKTRACE ======");
    let backtrace = Backtrace::capture();
    let _ = writeln!(file, "{}", backtrace);

    // Log environment variables that might be relevant
    let _ = writeln!(file, "\n====== ENVIRONMENT ======");
    for (key, value) in env::vars() {
        if key.contains("DYLD") || key.contains("PATH") || key.contains("HOME") {
            let _ = writeln!(file, "{}: {}", key, value);
        }
    }

    let _ = writeln!(file, "\n");

    // Print to stderr as well
    eprintln!("Application crashed. Log saved to: {:?}", log_file);
}
