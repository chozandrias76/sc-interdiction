//! Text utilities for the TUI.
//!
//! Provides unicode-aware text handling including truncation and scrolling.

use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

/// State for scrolling text display.
#[derive(Debug, Clone, Default)]
pub struct ScrollState {
    /// Currently hovered line index.
    pub hovered_line: Option<usize>,
    /// Last hovered line (to detect changes).
    last_hovered_line: Option<usize>,
    /// Scroll offset for text that exceeds available width.
    scroll_offset: usize,
    /// Tick counter for scroll animation timing.
    tick_count: usize,
}

impl ScrollState {
    /// Create a new scroll state.
    pub fn new() -> Self {
        Self::default()
    }

    /// Update scroll state on tick. Call this from the main event loop.
    pub fn on_tick(&mut self) {
        self.tick_count = self.tick_count.wrapping_add(1);

        // Reset scroll offset when hovered line changes
        if self.hovered_line != self.last_hovered_line {
            self.scroll_offset = 0;
            self.last_hovered_line = self.hovered_line;
        }

        // Advance scroll offset every 2 ticks for smooth scrolling
        if self.tick_count.is_multiple_of(2) {
            self.scroll_offset = self.scroll_offset.wrapping_add(1);
        }
    }

    /// Set the currently hovered line index.
    pub fn set_hovered(&mut self, line_index: Option<usize>) {
        self.hovered_line = line_index;
    }

    /// Get current scroll offset.
    pub fn offset(&self) -> usize {
        self.scroll_offset
    }

    /// Check if a specific line is currently hovered.
    pub fn is_hovered(&self, line_index: usize) -> bool {
        self.hovered_line == Some(line_index)
    }
}

/// Format text to fit within a maximum width, with scrolling when hovered.
///
/// When the text is too long:
/// - If not hovered: shows "text..." (truncated with ellipsis)
/// - If hovered: scrolls through the full text
///
/// # Arguments
/// * `text` - The text to display
/// * `max_width` - Maximum display width in terminal columns
/// * `line_index` - Unique identifier for this text field (for hover detection)
/// * `prefix` - Optional prefix (e.g., "▶ ") that takes display width
/// * `state` - Current scroll state
pub fn scroll_text(
    text: &str,
    max_width: usize,
    line_index: usize,
    prefix: &str,
    state: &ScrollState,
) -> String {
    let prefix_width = prefix.width();
    let available = max_width.saturating_sub(prefix_width);
    let text_width = text.width();

    if text_width <= available {
        // Text fits - return as-is with prefix
        return format!("{}{}", prefix, text);
    }

    // Text is too long - check if we're hovering this line
    if state.is_hovered(line_index) {
        scroll_text_animated(text, available, prefix, state.offset())
    } else {
        truncate_with_ellipsis(text, available, prefix)
    }
}

/// Render scrolling text animation.
fn scroll_text_animated(text: &str, available: usize, prefix: &str, offset: usize) -> String {
    // Add spacing for wrap-around effect
    let padded = format!("{}   ", text);
    let padded_width = padded.width();
    let scroll_pos = offset % padded_width;

    // Find character boundary for the scroll position
    let char_start = find_char_at_width(&padded, scroll_pos);

    // Build visible portion by iterating from the scroll position
    let mut result = String::with_capacity(available);
    let mut current_width = 0;

    // Get characters starting from scroll position, wrapping around
    let bytes: Vec<u8> = padded.bytes().collect();
    let len = bytes.len();
    let mut byte_pos = char_start;

    while current_width < available {
        // Wrap around if we've reached the end
        if byte_pos >= len {
            byte_pos = 0;
        }

        // Get the character at this position
        if let Some(c) = padded[byte_pos..].chars().next() {
            let cw = c.width().unwrap_or(0);
            if current_width + cw > available {
                break;
            }
            result.push(c);
            current_width += cw;
            byte_pos += c.len_utf8();
        } else {
            break;
        }
    }

    format!("{}{}", prefix, result)
}

/// Find the byte index of the character at a given display width.
fn find_char_at_width(s: &str, target_width: usize) -> usize {
    let mut width_count = 0;
    for (i, c) in s.char_indices() {
        if width_count >= target_width {
            return i;
        }
        width_count += c.width().unwrap_or(0);
    }
    s.len()
}

/// Truncate text with ellipsis to fit within a maximum width.
fn truncate_with_ellipsis(text: &str, available: usize, prefix: &str) -> String {
    let ellipsis = "...";
    let ellipsis_width = ellipsis.width();
    let truncate_to = available.saturating_sub(ellipsis_width);

    let mut result = String::with_capacity(truncate_to + ellipsis_width);
    let mut current_width = 0;

    for c in text.chars() {
        let cw = c.width().unwrap_or(0);
        if current_width + cw > truncate_to {
            break;
        }
        result.push(c);
        current_width += cw;
    }

    format!("{}{}{}", prefix, result, ellipsis)
}

/// Truncate text to fit within a maximum display width.
///
/// Unlike `scroll_text`, this always truncates and never scrolls.
/// Useful for static text fields that don't support hover.
pub fn truncate(s: &str, max_width: usize) -> String {
    let text_width = s.width();
    if text_width <= max_width {
        return s.to_string();
    }

    let ellipsis = "...";
    let target_width = max_width.saturating_sub(ellipsis.len());

    let mut result = String::new();
    let mut current_width = 0;

    for c in s.chars() {
        let char_width = c.width().unwrap_or(0);
        if current_width + char_width > target_width {
            break;
        }
        result.push(c);
        current_width += char_width;
    }

    format!("{}{}", result, ellipsis)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scroll_state_on_tick() {
        let mut state = ScrollState::new();
        assert_eq!(state.offset(), 0);

        state.on_tick();
        assert_eq!(state.tick_count, 1);
        assert_eq!(state.offset(), 0); // Only advances on even ticks

        state.on_tick();
        assert_eq!(state.tick_count, 2);
        assert_eq!(state.offset(), 1);
    }

    #[test]
    fn test_scroll_state_resets_on_hover_change() {
        let mut state = ScrollState::new();
        state.set_hovered(Some(5));

        // Advance scroll a few times
        for _ in 0..10 {
            state.on_tick();
        }
        assert!(state.offset() > 0);

        // Change hover - should reset offset
        state.set_hovered(Some(10));
        state.on_tick();
        assert_eq!(state.offset(), 0);
    }

    #[test]
    fn test_scroll_text_fits() {
        let state = ScrollState::new();
        let result = scroll_text("Hello", 20, 0, ">> ", &state);
        assert_eq!(result, ">> Hello");
    }

    #[test]
    fn test_scroll_text_truncates() {
        let state = ScrollState::new();
        let result = scroll_text("Hello World Test", 10, 0, "", &state);
        assert!(result.ends_with("..."));
        assert!(result.width() <= 10);
    }

    #[test]
    fn test_scroll_text_scrolls_when_hovered() {
        let mut state = ScrollState::new();
        state.set_hovered(Some(5));
        state.scroll_offset = 3;

        let result = scroll_text("Hello World Test", 15, 5, "", &state);
        assert!(!result.ends_with("..."));
    }

    #[test]
    fn test_truncate_basic() {
        assert_eq!(truncate("Hello", 10), "Hello");
        assert_eq!(truncate("Hello World Test", 10), "Hello W...");
    }

    #[test]
    fn test_truncate_unicode() {
        // Each CJK character is 2 columns wide
        let cjk = "你好世界";
        let result = truncate(cjk, 6);
        assert!(result.width() <= 6);
        assert!(result.ends_with("..."));
    }
}
