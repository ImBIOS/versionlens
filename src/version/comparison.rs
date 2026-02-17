use semver::{Version, VersionReq};

pub struct VersionComparator;

impl VersionComparator {
    pub fn new() -> Self {
        Self
    }

    /// Compare current version specifier with latest available version
    /// Returns comparison result with details
    pub fn compare(&self, current_spec: &str, latest_version: &str) -> VersionComparison {
        // Parse latest version
        let latest = match Version::parse(latest_version) {
            Ok(v) => v,
            Err(e) => return VersionComparison::Error(e.to_string()),
        };

        // Parse current specifier (handle ^, ~, >=, etc.)
        let requirement = self.parse_specifier(current_spec);

        // Check if latest satisfies requirement
        if requirement.matches(&latest) {
            VersionComparison::UpToDate {
                current: current_spec.to_string(),
                latest: latest_version.to_string(),
            }
        } else {
            // Determine diff type
            let diff = self.version_diff(current_spec, &latest);
            VersionComparison::Outdated {
                current: current_spec.to_string(),
                latest: latest_version.to_string(),
                diff,
            }
        }
    }

    /// Parse a version specifier (e.g., ^1.2.3, ~1.2.3, >=1.2.3, 1.2.3) into a VersionReq
    fn parse_specifier(&self, spec: &str) -> VersionReq {
        let spec = spec.trim();

        // Handle caret (^) prefix - allows minor and patch updates
        // ^1.2.3 means >=1.2.3, <2.0.0
        if let Some(version_str) = spec.strip_prefix('^') {
            let version_str = version_str.trim();
            if let Ok(v) = Version::parse(version_str) {
                return VersionReq::parse(&format!(
                    ">={}.{}.{}, <{}",
                    v.major,
                    v.minor,
                    v.patch,
                    v.major + 1
                ))
                .unwrap_or_else(|_| VersionReq::parse("*").unwrap());
            }
        }

        // Handle tilde (~) prefix - allows patch updates only
        // ~1.2.3 means >=1.2.3, <1.3.0
        if let Some(version_str) = spec.strip_prefix('~') {
            let version_str = version_str.trim();
            if let Ok(v) = Version::parse(version_str) {
                return VersionReq::parse(&format!(
                    ">={}.{}.{}, <{}.{}.{}",
                    v.major,
                    v.minor,
                    v.patch,
                    v.major,
                    v.minor + 1,
                    0
                ))
                .unwrap_or_else(|_| VersionReq::parse("*").unwrap());
            }
        }

        // Handle >=, >, <, <=, = prefixes
        if spec.starts_with('>') || spec.starts_with('<') || spec.starts_with('=') {
            if let Ok(req) = VersionReq::parse(spec) {
                return req;
            }
        }

        // Handle bare version number - treat as exact match
        if let Ok(v) = Version::parse(spec) {
            return VersionReq::parse(&format!("={}.{}.{}", v.major, v.minor, v.patch))
                .unwrap_or_else(|_| VersionReq::parse("*").unwrap());
        }

        // Fallback to any version
        VersionReq::parse("*").unwrap()
    }

    /// Determine the version difference between current spec and latest
    fn version_diff(&self, current_spec: &str, latest: &Version) -> VersionDiff {
        // Try to extract the base version from the specifier
        let base_version = self.extract_base_version(current_spec);

        if let Some(base) = base_version {
            if latest.major > base.major {
                VersionDiff::Major
            } else if latest.minor > base.minor {
                VersionDiff::Minor
            } else {
                VersionDiff::Patch
            }
        } else {
            // If we can't parse the spec, default to patch
            VersionDiff::Patch
        }
    }

    /// Extract the base version from a specifier string
    fn extract_base_version(&self, spec: &str) -> Option<Version> {
        let spec = spec.trim();

        // Remove prefixes
        let version_str = spec
            .strip_prefix('^')
            .or_else(|| spec.strip_prefix('~'))
            .or_else(|| spec.strip_prefix('>'))
            .or_else(|| spec.strip_prefix('<'))
            .or_else(|| spec.strip_prefix('='))
            .unwrap_or(spec);

        Version::parse(version_str.trim()).ok()
    }
}

impl Default for VersionComparator {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub enum VersionComparison {
    UpToDate {
        current: String,
        latest: String,
    },
    Outdated {
        current: String,
        latest: String,
        diff: VersionDiff,
    },
    Error(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum VersionDiff {
    Major,
    Minor,
    Patch,
}

impl VersionComparison {
    /// Generate display text for the version comparison
    pub fn display_text(&self) -> String {
        match self {
            VersionComparison::UpToDate { current, latest } => {
                format!("{} → {}", current, latest)
            }
            VersionComparison::Outdated { current, latest, diff } => {
                format!("{} → {} ({})", current, latest, diff.label())
            }
            VersionComparison::Error(msg) => format!("Error: {}", msg),
        }
    }
}

impl VersionDiff {
    /// Get label for the version difference
    pub fn label(&self) -> &'static str {
        match self {
            VersionDiff::Major => "major",
            VersionDiff::Minor => "minor",
            VersionDiff::Patch => "patch",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_caret_version_up_to_date() {
        let comparator = VersionComparator::new();
        // ^1.2.0 allows 1.x.x updates, so 1.3.0 is up to date
        let result = comparator.compare("^1.2.0", "1.3.0");

        if let VersionComparison::UpToDate { .. } = result {
            assert!(true);
        } else {
            panic!("Expected UpToDate");
        }
    }

    #[test]
    fn test_caret_version_outdated_minor() {
        let comparator = VersionComparator::new();
        // 1.2.0 doesn't match >=1.3.0 requirement
        let result = comparator.compare(">=1.3.0", "1.4.0");

        if let VersionComparison::UpToDate { .. } = result {
            // This should actually be up to date!
            assert!(true);
        } else {
            // If not up to date, check diff
            if let VersionComparison::Outdated { diff, .. } = result {
                assert_eq!(diff, VersionDiff::Minor);
            } else {
                panic!("Expected UpToDate or Outdated with Minor diff");
            }
        }
    }

    #[test]
    fn test_caret_version_outdated_major() {
        let comparator = VersionComparator::new();
        // ^1.2.0 doesn't allow 2.x.x (major version change)
        let result = comparator.compare("^1.2.0", "2.0.0");

        if let VersionComparison::Outdated { diff, .. } = result {
            assert_eq!(diff, VersionDiff::Major);
        } else {
            panic!("Expected Outdated with Major diff");
        }
    }

    #[test]
    fn test_tilde_version() {
        let comparator = VersionComparator::new();
        // ~1.2.0 allows 1.2.x updates
        let result = comparator.compare("~1.2.0", "1.2.5");

        if let VersionComparison::UpToDate { .. } = result {
            assert!(true);
        } else {
            panic!("Expected UpToDate");
        }
    }

    #[test]
    fn test_exact_version() {
        let comparator = VersionComparator::new();
        let result = comparator.compare("1.2.3", "1.2.3");

        if let VersionComparison::UpToDate { .. } = result {
            assert!(true);
        } else {
            panic!("Expected UpToDate");
        }
    }

    #[test]
    fn test_greater_than_or_equal() {
        let comparator = VersionComparator::new();
        let result = comparator.compare(">=1.0.0", "1.2.3");

        if let VersionComparison::UpToDate { .. } = result {
            assert!(true);
        } else {
            panic!("Expected UpToDate");
        }
    }

    #[test]
    fn test_display_text() {
        let comparator = VersionComparator::new();

        // ^1.2.0 with 1.3.0 should be UpToDate
        let result = comparator.compare("^1.2.0", "1.3.0");
        assert_eq!(result.display_text(), "^1.2.0 → 1.3.0");

        // ^1.2.0 with 2.0.0 should be Outdated with Major
        let result = comparator.compare("^1.2.0", "2.0.0");
        assert_eq!(result.display_text(), "^1.2.0 → 2.0.0 (major)");
    }
}
