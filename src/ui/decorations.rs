use crate::ui::badges::{Badge, BadgeStyle};
use std::collections::HashMap;

/// Manages decorations (badges) for buffer lines
///
/// Note: The current zed_extension_api (v0.7.0) doesn't expose a direct decoration API.
/// This implementation uses worktree operations and provides the foundation for when
/// the API becomes available, or can be integrated with custom rendering approaches.
pub struct DecorationManager {
    /// Cache of active badges per buffer
    badges_cache: HashMap<String, Vec<Badge>>,
}

impl DecorationManager {
    /// Create a new DecorationManager
    pub fn new() -> Self {
        Self {
            badges_cache: HashMap::new(),
        }
    }

    /// Update badges for a buffer
    ///
    /// This method stores badges in the cache and triggers decoration updates.
    /// When zed_extension_api supports direct decoration APIs, this will
    /// render the badges inline.
    pub fn update_badges(&mut self, badges: Vec<Badge>, buffer_id: &str) {
        // Store badges in cache
        self.badges_cache.insert(buffer_id.to_string(), badges.clone());

        // In a full implementation, this would call Zed's decoration API
        // For now, we store the badges and they can be retrieved for display
    }

    /// Get badges for a specific buffer
    pub fn get_badges(&self, buffer_id: &str) -> Option<&Vec<Badge>> {
        self.badges_cache.get(buffer_id)
    }

    /// Clear badges for a buffer
    pub fn clear_badges(&mut self, buffer_id: &str) {
        self.badges_cache.remove(buffer_id);
    }

    /// Clear all badges
    pub fn clear_all(&mut self) {
        self.badges_cache.clear();
    }

    /// Check if a buffer has any badges
    pub fn has_badges(&self, buffer_id: &str) -> bool {
        self.badges_cache
            .get(buffer_id)
            .map(|b| !b.is_empty())
            .unwrap_or(false)
    }

    /// Get badge count for a buffer
    pub fn badge_count(&self, buffer_id: &str) -> usize {
        self.badges_cache
            .get(buffer_id)
            .map(|b| b.len())
            .unwrap_or(0)
    }
}

impl Default for DecorationManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Create badges from version comparison results
///
/// Takes a list of (package_name, current_version, latest_version, line_number) tuples
/// and creates appropriate badges based on version differences.
pub fn create_badges_from_versions(
    versions: Vec<(String, String, String, usize)>,
) -> Vec<Badge> {
    versions
        .into_iter()
        .map(|(name, current, latest, line)| {
            let style = BadgeStyle::from_version_diff(&current, &latest);

            let text = match style {
                BadgeStyle::UpToDate => format!("{} -> {} (current)", current, latest),
                BadgeStyle::Outdated => format!("{} -> {} (outdated)", current, latest),
                BadgeStyle::MajorDiff => format!("{} -> {} (major)", current, latest),
            };

            Badge::new(text, style, line, name)
        })
        .collect()
}

/// Filter badges by style
pub fn filter_badges_by_style<'a>(badges: &'a [Badge], style: &'a BadgeStyle) -> Vec<&'a Badge> {
    badges.iter().filter(|b| &b.style == style).collect()
}

/// Get all unique buffer IDs that have badges
pub fn get_active_buffer_ids(manager: &DecorationManager) -> Vec<String> {
    manager
        .badges_cache
        .iter()
        .filter(|(_, badges)| !badges.is_empty())
        .map(|(id, _)| id.clone())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decoration_manager() {
        let mut manager = DecorationManager::new();

        let badges = vec![
            Badge::up_to_date("lodash".to_string(), "4.17.21", "4.17.21", 5),
            Badge::outdated("express".to_string(), "4.18.0", "4.19.0", 10),
        ];

        manager.update_badges(badges, "buffer-1");

        assert!(manager.has_badges("buffer-1"));
        assert_eq!(manager.badge_count("buffer-1"), 2);

        manager.clear_badges("buffer-1");
        assert!(!manager.has_badges("buffer-1"));
    }

    #[test]
    fn test_create_badges_from_versions() {
        let versions = vec![
            ("lodash".to_string(), "4.17.21".to_string(), "4.17.21".to_string(), 5),
            ("express".to_string(), "4.18.0".to_string(), "4.19.0".to_string(), 10),
            ("react".to_string(), "17.0.0".to_string(), "18.0.0".to_string(), 15),
        ];

        let badges = create_badges_from_versions(versions);

        assert_eq!(badges.len(), 3);
        assert_eq!(badges[0].style, BadgeStyle::UpToDate);
        assert_eq!(badges[1].style, BadgeStyle::Outdated);
        assert_eq!(badges[2].style, BadgeStyle::MajorDiff);
    }
}
