//! Countdown to a specific date/time target.
//!
//! This module implements the "Countdown To" timer mode, which calculates
//! the remaining time until a specific target timestamp.

use js_sys::Date;
use wasm_bindgen::prelude::*;
use web_sys::console;

use crate::model::{TimeDisplay, TimeFormat, TimerMode, TimerState};

/// Configuration for a countdown-to-date timer
#[derive(Debug, Clone, PartialEq)]
pub struct CountdownToConfig {
  /// Target timestamp in milliseconds since epoch
  pub target_ms: u64,
  /// Current state of the countdown
  pub remaining_ms: i64,
  /// Whether the countdown has reached zero
  pub is_complete: bool,
}

impl CountdownToConfig {
  /// Create a new countdown-to-date configuration
  pub fn new(target_ms: u64) -> Self {
    let now_ms = Date::now() as i64;
    let target = target_ms as i64;
    let remaining = target - now_ms;

    Self {
      target_ms,
      remaining_ms: remaining,
      is_complete: remaining <= 0,
    }
  }

  /// Update the countdown state based on current time
  pub fn update(&mut self) {
    let now_ms = Date::now() as i64;
    let target = self.target_ms as i64;
    let remaining = target - now_ms;

    self.remaining_ms = remaining;
    self.is_complete = remaining <= 0;
  }

  /// Get the absolute remaining time in milliseconds
  pub fn absolute_remaining(&self) -> u64 {
    self.remaining_ms.unsigned_abs()
  }

  /// Calculate time display from remaining time
  pub fn to_time_display(&self, format: TimeFormat) -> TimeDisplay {
    TimeDisplay::from_ms(self.remaining_ms, format)
  }

  /// Check if the countdown is in the past
  pub fn is_in_past(&self) -> bool {
    self.remaining_ms < 0
  }

  /// Format the remaining time as a human-readable string
  pub fn format_remaining(&self) -> String {
    let abs_remaining = self.absolute_remaining();
    let total_seconds = abs_remaining / 1000;
    let millis = (abs_remaining % 1000) as u32;

    let hours = (total_seconds / 3600) as u32;
    let minutes = ((total_seconds % 3600) / 60) as u32;
    let seconds = (total_seconds % 60) as u32;

    if hours > 0 {
      format!("{:02}:{:02}:{:02}.{}", hours, minutes, seconds, millis / 100)
    } else {
      format!("{:02}:{:02}.{}", minutes, seconds, millis / 100)
    }
  }

  /// Get the target date as a formatted string
  pub fn format_target(&self) -> String {
    let target = Date::new(&JsValue::from_f64(self.target_ms as f64));
    let year = target.get_full_year();
    let month = target.get_month() + 1;
    let day = target.get_date();
    let hours = target.get_hours();
    let minutes = target.get_minutes();
    format!("{:04}-{:02}-{:02} {:02}:{:02}", year, month, day, hours, minutes)
  }
}

/// Create a new countdown-to-date timer state
pub fn create_countdown_to(state: &mut TimerState, target_ms: u64) {
  state.mode = TimerMode::CountdownTo;
  state.target_timestamp_ms = Some(target_ms);
  state.is_counting_up = false;
  state.elapsed_ms = 0;

  // Calculate initial remaining time
  let now_ms = Date::now() as i64;
  let remaining = (target_ms as i64) - now_ms;
  state.elapsed_ms = remaining.unsigned_abs();

  if let Some(config) = get_countdown_to_config(target_ms) {
    console::log_1(&format!("Countdown to {} (remaining: {}ms)", config.format_target(), config.format_remaining()).into());
  }
}

/// Get the current countdown-to-date configuration
pub fn get_countdown_to_config(target_ms: u64) -> Option<CountdownToConfig> {
  let mut config = CountdownToConfig::new(target_ms);
  config.update();
  Some(config)
}

/// Calculate the display time for countdown-to mode
pub fn get_display_time(state: &TimerState) -> Option<TimeDisplay> {
  let target = state.target_timestamp_ms?;
  let config = get_countdown_to_config(target)?;

  Some(config.to_time_display(state.time_format))
}

/// Check if the countdown has reached zero
pub fn is_countdown_complete(state: &TimerState) -> bool {
  if let Some(target) = state.target_timestamp_ms {
    get_countdown_to_config(target).is_some_and(|config| config.is_complete)
  } else {
    false
  }
}

/// Reset the countdown to its original target
pub fn reset_countdown_to(state: &mut TimerState) {
  if let Some(target) = state.target_timestamp_ms {
    let config = CountdownToConfig::new(target);
    state.is_counting_up = config.is_complete;
    state.elapsed_ms = config.absolute_remaining();
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_countdown_to_new() {
    let target = Date::now() + 3600000.0; // 1 hour from now
    let config = CountdownToConfig::new(target as u64);

    assert!(!config.is_complete);
    assert!(!config.is_in_past());
    assert!(config.absolute_remaining() > 0);
  }

  #[test]
  fn test_countdown_to_past() {
    let target = Date::now() - 1000.0; // 1 second ago
    let config = CountdownToConfig::new(target as u64);

    assert!(config.is_complete);
    assert!(config.is_in_past());
  }
}
