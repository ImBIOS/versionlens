use semver::Version;
use serde::{Deserialize, Serialize};

/// Represents a version badge to display inline in the editor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Badge {
    /// The text to display in the badge (e.g., "^1.2.3 -> 1.4.0")
    pub text: String,
    /// The visual style of the badge
    pub style: BadgeStyle,
    /// The line number where this badge should be displayed
    pub line: usize,
    /// The package name this badge is for
    pub package_name: String,
}

impl Badge {
    /// Create a new badge with the given text, style, and line number
    pub fn new(text: String, style: BadgeStyle, line: usize, package_name: String) -> Self {
        Self {
            text,
            style,
            line,
            package_name,
        }
    }

    /// Create an up-to-date badge
    pub fn up_to_date(package_name: String, current: &str, latest: &str, line: usize) -> Self {
        Self::new(
            format!("{} -> {} (up to date)", current, latest),
            BadgeStyle::UpToDate,
            line,
            package_name,
        )
    }

    /// Create an outdated badge
    pub fn outdated(package_name: String, current: &str, latest: &str, line: usize) -> Self {
        Self::new(
            format!("{} -> {}", current, latest),
            BadgeStyle::Outdated,
            line,
            package_name,
        )
    }

    /// Create a major version diff badge
    pub fn major_diff(package_name: String, current: &str, latest: &str, line: usize) -> Self {
        Self::new(
            format!("{} -> {} (major)", current, latest),
            BadgeStyle::MajorDiff,
            line,
            package_name,
        )
    }
}

/// Visual style for badges with color coding
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BadgeStyle {
    /// Green - package is up to date
    UpToDate,
    /// Yellow - package is outdated but same major version
    Outdated,
    /// Red - major version difference
    MajorDiff,
}

impl BadgeStyle {
    /// Determine badge style from version comparison
    ///
    /// - MajorDiff: major version differs
    /// - Outdated: same major but different minor/patch
    /// - UpToDate: versions match
    pub fn from_version_diff(current: &str, latest: &str) -> Self {
        // Parse versions, default to Outdated if parsing fails
        let current_version = Self::parse_version(current);
        let latest_version = Self::parse_version(latest);

        match (current_version, latest_version) {
            (Some(curr), Some(latest)) => {
                // Compare major versions
                if curr.major != latest.major {
                    BadgeStyle::MajorDiff
                } else if curr != latest {
                    BadgeStyle::Outdated
                } else {
                    BadgeStyle::UpToDate
                }
            }
            // If we can't parse, assume outdated
            _ => BadgeStyle::Outdated,
        }
    }

    /// Parse a semver version string, handling caret (^) prefix
    fn parse_version(version: &str) -> Option<Version> {
        // Remove caret (^) prefix if present
        let cleaned = version.trim_start_matches('^');
        Version::parse(checked_version(cleaned)).ok()
    }

    /// Get the display color for this badge style
    /// Returns RGB hex values for Zed color codes
    pub fn color(&self) -> &'static str {
        match self {
            BadgeStyle::UpToDate => "#4ade80",    // Green
            BadgeStyle::Outdated => "#facc15",    // Yellow
            BadgeStyle::MajorDiff => "#f87171",   // Red
        }
    }

    /// Get the background color for this badge style
    pub fn background_color(&self) -> &'static str {
        match self {
            BadgeStyle::UpToDate => "#22c55e33",   // Green with transparency
            BadgeStyle::Outdated => "#eab30833",  // Yellow with transparency
            BadgeStyle::MajorDiff => "#ef444433", // Red with transparency
        }
    }
}

/// Handle npm version range notation
/// Converts ^1.2.3 -> 1.2.3 and ~1.2.3 -> 1.2.3 for semver parsing
fn checked_version(version: &str) -> &str {
    // Handle caret and tilde prefixes
    let trimmed = version.trim();
    if trimmed.starts_with('^') || trimmed.starts_with('~') {
        &trimmed[1..]
    } else if trimmed.starts_with('>') || trimmed.starts_with('<') || trimmed.starts_with('=') {
        // For ranges like >=1.0.0, just take the first version
        trimmed.split_whitespace().next().unwrap_or(trimmed)
    } else {
        trimmed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_major_diff() {
        let style = BadgeStyle::from_version_diff("1.0.0", "2.0.0");
        assert_eq!(style, BadgeStyle::MajorDiff);
    }

    #[test]
    fn test_outdated() {
        let style = BadgeStyle::from_version_diff("1.2.3", "1.4.0");
        assert_eq!(style, BadgeStyle::Outdated);
    }

    #[test]
    fn test_up_to_date() {
        let style = BadgeStyle::from_version_diff("1.2.3", "1.2.3");
        assert_eq!(style, BadgeStyle::UpToDate);
    }

    #[test]
    fn test_caret_prefix() {
        let style = BadgeStyle::from_version_diff("^1.2.3", "1.2.3");
        assert_eq!(style, BadgeStyle::UpToDate);
    }

    #[test]
    fn test_outdated_with_caret() {
        let style = BadgeStyle::from_version_diff("^1.2.3", "1.4.0");
        assert_eq!(style, BadgeStyle::Outdated);
    }
}
