//! Environment-driven runtime configuration that is not part of
//! `indexer::Config`: currently the indexer poll interval.

use std::time::Duration;

use crate::indexer::POLL_INTERVAL;

/// Env var holding the poll interval in whole seconds.
pub const POLL_INTERVAL_VAR: &str = "RWA_POLL_INTERVAL_SECS";
const MIN_POLL_SECS: u64 = 1;
const MAX_POLL_SECS: u64 = 3600;

/// Parse a raw `RWA_POLL_INTERVAL_SECS` value (`None` = unset, use default).
pub fn parse_poll_interval(raw: Option<&str>) -> Result<Duration, String> {
    let Some(raw) = raw else {
        return Ok(POLL_INTERVAL);
    };
    let secs: u64 = raw.trim().parse().map_err(|_| {
        format!("{POLL_INTERVAL_VAR} must be a whole number of seconds, got {raw:?}")
    })?;
    if !(MIN_POLL_SECS..=MAX_POLL_SECS).contains(&secs) {
        return Err(format!(
            "{POLL_INTERVAL_VAR} must be between {MIN_POLL_SECS} and {MAX_POLL_SECS}, got {secs}"
        ));
    }
    Ok(Duration::from_secs(secs))
}

/// Poll interval from the environment; falls back to the default when unset
/// or invalid (invalid values are rejected at startup by `main`).
pub fn poll_interval() -> Duration {
    parse_poll_interval(std::env::var(POLL_INTERVAL_VAR).ok().as_deref()).unwrap_or(POLL_INTERVAL)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_when_unset() {
        assert_eq!(parse_poll_interval(None).unwrap(), POLL_INTERVAL);
    }

    #[test]
    fn parses_valid_value() {
        assert_eq!(parse_poll_interval(Some("30")).unwrap(), Duration::from_secs(30));
    }

    #[test]
    fn rejects_invalid_values() {
        for bad in ["abc", "0", "-1", "99999", ""] {
            let err = parse_poll_interval(Some(bad)).unwrap_err();
            assert!(err.contains("RWA_POLL_INTERVAL_SECS"), "{err}");
        }
    }
}
