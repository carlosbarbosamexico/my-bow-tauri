//! Navigation guard module for controlling WebView navigation
//! 
//! This module provides utilities to restrict navigation to allowed domains,
//! preventing users from navigating to arbitrary external websites while
//! still allowing necessary auth flows.

use crate::ALLOWED_HOSTS;

/// Validate if a navigation target is permitted
pub fn validate_navigation(url: &str) -> NavigationResult {
    // Always allow the main app
    if url.starts_with("https://app.bowsapp.com") {
        return NavigationResult::Allow;
    }

    // Allow local/internal URLs
    if url.starts_with("tauri://") || url.starts_with("asset://") || url.starts_with("about:") {
        return NavigationResult::Allow;
    }

    // Parse and check against allowed hosts
    if let Ok(parsed) = url::Url::parse(url) {
        if let Some(host) = parsed.host_str() {
            for allowed in ALLOWED_HOSTS {
                if host == *allowed || host.ends_with(&format!(".{}", allowed)) {
                    return NavigationResult::Allow;
                }
            }
        }
    }

    NavigationResult::Block(url.to_string())
}

/// Result of navigation validation
#[derive(Debug)]
pub enum NavigationResult {
    /// Navigation is allowed
    Allow,
    /// Navigation is blocked - contains the blocked URL
    Block(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_app_allowed() {
        assert!(matches!(
            validate_navigation("https://app.bowsapp.com/my-bow"),
            NavigationResult::Allow
        ));
    }

    #[test]
    fn test_google_auth_allowed() {
        assert!(matches!(
            validate_navigation("https://accounts.google.com/oauth"),
            NavigationResult::Allow
        ));
    }

    #[test]
    fn test_external_blocked() {
        assert!(matches!(
            validate_navigation("https://example.com"),
            NavigationResult::Block(_)
        ));
    }

    #[test]
    fn test_tauri_protocol_allowed() {
        assert!(matches!(
            validate_navigation("tauri://localhost"),
            NavigationResult::Allow
        ));
    }
}

