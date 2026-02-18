use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Debouncer to prevent excessive operations on the same resource
///
/// Used to debounce buffer change events and API calls to avoid
/// hammering the registry with requests when the user is typing.
pub struct Debouncer {
    /// Map of keys to their last processed timestamp
    delays: HashMap<String, Instant>,
    /// Timeout duration in milliseconds
    timeout_ms: u64,
}

impl Debouncer {
    /// Create a new debouncer with the specified timeout
    ///
    /// # Arguments
    /// * `timeout_ms` - The minimum time in milliseconds that must elapse
    ///                  before the same key can be processed again
    ///
    /// # Example
    /// ```rust
    /// let debouncer = Debouncer::new(500); // 500ms debounce
    /// ```
    pub fn new(timeout_ms: u64) -> Self {
        Self {
            delays: HashMap::new(),
            timeout_ms,
        }
    }

    /// Check if an action should proceed for the given key
    ///
    /// Returns `true` if:
    /// - The key has never been seen before, OR
    /// - The debounce period has elapsed since the last action
    ///
    /// When returning `true`, updates the timestamp for this key.
    ///
    /// # Arguments
    /// * `key` - The unique identifier for this action (e.g., file path)
    ///
    /// # Returns
    /// * `true` - The action should proceed
    /// * `false` - The action should be debounced
    pub fn should_proceed(&mut self, key: &str) -> bool {
        let now = Instant::now();

        // Check if we've seen this key before
        if let Some(last_time) = self.delays.get(key) {
            let elapsed = now.duration_since(*last_time);
            let timeout = Duration::from_millis(self.timeout_ms);

            if elapsed < timeout {
                // Still within debounce period
                return false;
            }
        }

        // Either never seen or debounce period elapsed - proceed and update
        self.delays.insert(key.to_string(), now);
        true
    }

    /// Reset the debouncer for a specific key
    ///
    /// This forces the next call to `should_proceed` to return `true`
    /// for the given key, effectively resetting its debounce state.
    ///
    /// # Arguments
    /// * `key` - The key to reset
    pub fn reset(&mut self, key: &str) {
        self.delays.remove(key);
    }

    /// Clear all debounce state
    ///
    /// This resets the entire debouncer, clearing all timestamps.
    pub fn clear(&mut self) {
        self.delays.clear();
    }

    /// Get the remaining time before a key can proceed again
    ///
    /// # Arguments
    /// * `key` - The key to check
    ///
    /// # Returns
    /// * `Some(Duration)` - Remaining time, if the key exists and is debounced
    /// * `None` - If the key doesn't exist or is ready to proceed
    pub fn remaining_time(&self, key: &str) -> Option<Duration> {
        if let Some(last_time) = self.delays.get(key) {
            let elapsed = Instant::now().duration_since(*last_time);
            let timeout = Duration::from_millis(self.timeout_ms);

            if elapsed < timeout {
                return Some(timeout - elapsed);
            }
        }
        None
    }

    /// Check if a specific key is currently debounced
    ///
    /// # Arguments
    /// * `key` - The key to check
    ///
    /// # Returns
    /// * `true` - The key exists and is within its debounce period
    /// * `false` - The key doesn't exist or is ready to proceed
    pub fn is_debounced(&self, key: &str) -> bool {
        if let Some(last_time) = self.delays.get(key) {
            let elapsed = Instant::now().duration_since(*last_time);
            let timeout = Duration::from_millis(self.timeout_ms);
            elapsed < timeout
        } else {
            false
        }
    }
}

impl Default for Debouncer {
    fn default() -> Self {
        // Default to 500ms debounce
        Self::new(500)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_key_proceeds() {
        let mut debouncer = Debouncer::new(100);
        assert!(debouncer.should_proceed("file1"));
    }

    #[test]
    fn test_same_key_debounced() {
        let mut debouncer = Debouncer::new(100);

        // First call proceeds
        assert!(debouncer.should_proceed("file1"));

        // Immediate second call should be debounced
        assert!(!debouncer.should_proceed("file1"));
    }

    #[test]
    fn test_different_keys_independent() {
        let mut debouncer = Debouncer::new(100);

        assert!(debouncer.should_proceed("file1"));
        assert!(debouncer.should_proceed("file2"));
    }

    #[test]
    fn test_reset() {
        let mut debouncer = Debouncer::new(100);

        assert!(debouncer.should_proceed("file1"));
        assert!(!debouncer.should_proceed("file1"));

        debouncer.reset("file1");

        // Should proceed again after reset
        assert!(debouncer.should_proceed("file1"));
    }

    #[test]
    fn test_clear() {
        let mut debouncer = Debouncer::new(100);

        debouncer.should_proceed("file1");
        debouncer.should_proceed("file2");

        debouncer.clear();

        // All keys should proceed after clear
        assert!(debouncer.should_proceed("file1"));
        assert!(debouncer.should_proceed("file2"));
    }

    #[test]
    fn test_remaining_time() {
        let debouncer = Debouncer::new(100);

        // Key doesn't exist yet
        assert!(debouncer.remaining_time("nonexistent").is_none());
    }

    #[test]
    fn test_is_debounced() {
        let mut debouncer = Debouncer::new(100);

        // Key doesn't exist
        assert!(!debouncer.is_debounced("file1"));

        debouncer.should_proceed("file1");

        // Just added, should be debounced
        assert!(debouncer.is_debounced("file1"));
    }
}
