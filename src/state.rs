use std::sync::atomic::{AtomicBool, Ordering};

/// Application state for the VersionLens extension.
///
/// Manages global state like whether inline badges are enabled.
pub struct AppState {
    inline_enabled: AtomicBool,
}

impl AppState {
    /// Create a new AppState with inline badges enabled by default.
    pub fn new() -> Self {
        Self {
            inline_enabled: AtomicBool::new(true),
        }
    }

    /// Check if inline badges are currently enabled.
    pub fn is_inline_enabled(&self) -> bool {
        self.inline_enabled.load(Ordering::SeqCst)
    }

    /// Toggle inline badge display.
    ///
    /// Returns the new state after toggling.
    pub fn toggle_inline(&self) -> bool {
        // Flip the current value and return the new value
        let new_value = !self.inline_enabled.load(Ordering::SeqCst);
        self.inline_enabled.store(new_value, Ordering::SeqCst);
        new_value
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_inline_enabled() {
        let state = AppState::new();
        assert!(state.is_inline_enabled());
    }

    #[test]
    fn test_toggle_inline() {
        let state = AppState::new();

        // Initially enabled
        assert!(state.is_inline_enabled());

        // Toggle to disable
        let new_state = state.toggle_inline();
        assert!(!new_state);
        assert!(!state.is_inline_enabled());

        // Toggle back to enable
        let new_state = state.toggle_inline();
        assert!(new_state);
        assert!(state.is_inline_enabled());
    }
}
