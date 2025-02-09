use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_local_time_in_secs() -> i64 {
    // Get the local timezone offset in seconds
    #[cfg(target_family = "unix")]
    let local_offset = {
        use std::process::Command;

        // On Unix-like systems, we can use the `date` command
        let output = Command::new("date")
            .arg("+%z")
            .output()
            .expect("Failed to execute date command");

        let offset_str = String::from_utf8_lossy(&output.stdout);
        let hours: i32 = offset_str[..3].parse().unwrap_or(0);
        hours * 3600 // Convert hours to seconds
    };

    #[cfg(target_family = "windows")]
    let local_offset = {
        use std::process::Command;

        // On Windows, we can use PowerShell to get the timezone offset
        let output = Command::new("powershell")
            .arg("-Command")
            .arg("(Get-TimeZone).BaseUtcOffset.TotalHours")
            .output()
            .expect("Failed to execute PowerShell command");

        let offset_str = String::from_utf8_lossy(&output.stdout);
        let hours: i32 = offset_str.trim().parse().unwrap_or(0);
        hours * 3600 // Convert hours to seconds
    };

    // Get UTC time
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    // Add local offset
    let local_time = now + local_offset as i64;

    local_time
}
