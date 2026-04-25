//! Current clock mode timer logic.
//!
//! This module handles the display of the current time of day,
//! updating every frame to show hours, minutes, seconds, and milliseconds.

/// Represents the current time of day
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ClockTime {
  pub hours: u32,
  pub minutes: u32,
  pub seconds: u32,
  pub milliseconds: u32,
}

impl ClockTime {
  /// Get the current time from the browser's Date API
  pub fn now() -> Self {
    let now_ms = js_sys::Date::now();
    let date = js_sys::Date::new(&now_ms.into());

    // Get time components with proper timezone handling
    let hours = date.get_hours();
    let minutes = date.get_minutes();
    let seconds = date.get_seconds();
    let milliseconds = date.get_milliseconds();

    Self {
      hours,
      minutes,
      seconds,
      milliseconds,
    }
  }

  /// Format as HH:MM:SS.mmm for display
  pub fn formatted(&self) -> String {
    format!("{:02}:{:02}:{:02}.{:03}", self.hours, self.minutes, self.seconds, self.milliseconds)
  }

  /// Format as HH:MM:SS without milliseconds
  pub fn formatted_short(&self) -> String {
    format!("{:02}:{:02}:{:02}", self.hours, self.minutes, self.seconds)
  }
}

// Timer loop functions removed - timer is now managed directly in app.rs using gloo_timers::callback::Interval

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_clock_time_formatting() {
    let time = ClockTime {
      hours: 9,
      minutes: 5,
      seconds: 3,
      milliseconds: 7,
    };
    assert_eq!(time.formatted(), "09:05:03.007");
    assert_eq!(time.formatted_short(), "09:05:03");
  }

  #[test]
  fn test_clock_time_midnight() {
    let time = ClockTime {
      hours: 0,
      minutes: 0,
      seconds: 0,
      milliseconds: 0,
    };
    assert_eq!(time.formatted(), "00:00:00.000");
  }

  #[test]
  fn test_clock_time_noon() {
    let time = ClockTime {
      hours: 12,
      minutes: 30,
      seconds: 45,
      milliseconds: 999,
    };
    assert_eq!(time.formatted(), "12:30:45.999");
    assert_eq!(time.formatted_short(), "12:30:45");
  }
}
