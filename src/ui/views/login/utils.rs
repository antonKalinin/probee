use regex::Regex;

/**
 * This function uses a very simple regex to check if an email is valid.
 * For a more accurate check, consider using a more complex regex or a library.
 */
pub fn is_valid_email(email: &str) -> bool {
    // Basic length check
    if email.len() > 254 || email.is_empty() {
        return false;
    }

    // Split into local and domain parts
    let parts: Vec<&str> = email.split('@').collect();
    if parts.len() != 2 {
        return false;
    }

    let (local, domain) = (parts[0], parts[1]);

    // Check lengths of local and domain parts
    if local.len() > 64 || local.is_empty() || domain.len() > 255 || domain.is_empty() {
        return false;
    }

    // - Can have dots but not consecutively and not at start/end
    if local.ends_with('.') || local.starts_with('.') || local.contains("..") {
        return false;
    }

    let email_regex = Regex::new(r"^[a-z0-9._%+-]+@[a-z0-9.-]+\.[a-z]{2,4}$").unwrap();

    email_regex.is_match(email)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_emails() {
        assert!(is_valid_email("user@example.com"));
        assert!(is_valid_email("user.name@example.com"));
        assert!(is_valid_email("user+tag@example.com"));
        assert!(is_valid_email("user@subdomain.example.com"));
    }

    #[test]
    fn test_invalid_emails() {
        assert!(!is_valid_email(""));
        assert!(!is_valid_email("@example.com"));
        assert!(!is_valid_email("user@"));
        assert!(!is_valid_email("user.@example.com"));
        assert!(!is_valid_email(".user@example.com"));
        assert!(!is_valid_email("user@example"));
        assert!(!is_valid_email("user..name@example.com"));
    }
}
