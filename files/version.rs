//! Static build info for Typora Pro.

pub const BUILD_VERSION: &str  = "v1.5.25";
pub const BUILD_CHANNEL: &str  = "stable";
pub const BUILD_COMMIT: &str   = "d8afaf218ee7";
pub const BUILD_TIMESTAMP: u64 = 1794654362;

pub fn version_tuple() -> (u16, u16, u16) {
    let mut parts = BUILD_VERSION.trim_start_matches('v').split('.');
    let major = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let minor = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let patch = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    (major, minor, patch)
}

pub fn user_agent() -> String {
    format!("{}/{} ({})", "Typora Pro", BUILD_VERSION, BUILD_CHANNEL)
}
