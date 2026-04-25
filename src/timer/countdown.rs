/// Countdown timer logic.
///
/// Handles a timer that counts down from a user-specified duration to zero,
/// then continues counting up once it reaches zero.
use crate::model::TimerState;

/// The state of a countdown timer
#[derive(Debug, Clone, PartialEq)]
pub struct Countdown {
  /// The initial duration in milliseconds
  pub duration_ms: u64,
  /// The current remaining time in milliseconds
  pub remaining_ms: u64,
  /// Whether the countdown has reached zero and is now counting up
  pub is_elapsed: bool,
  /// Whether the timer is currently running
  pub is_running: bool,
}

impl Countdown {
  /// Create a new countdown timer with the given duration
  pub fn new(duration_ms: u64) -> Self {
    Self {
      duration_ms,
      remaining_ms: duration_ms,
      is_elapsed: false,
      is_running: false,
    }
  }

  /// Start or restart the countdown
  pub fn start(&mut self) {
    self.remaining_ms = self.duration_ms;
    self.is_elapsed = false;
    self.is_running = true;
  }

  /// Pause the countdown
  pub fn pause(&mut self) {
    self.is_running = false;
  }

  /// Resume the countdown from where it was paused
  pub fn resume(&mut self) {
    self.is_running = true;
  }

  /// Reset the countdown to its initial state
  pub fn reset(&mut self) {
    self.remaining_ms = self.duration_ms;
    self.is_elapsed = false;
  }

  /// Update the countdown by the given delta in milliseconds
  pub fn update(&mut self, delta_ms: u64) {
    if !self.is_running || self.is_elapsed {
      return;
    }

    if self.remaining_ms > delta_ms {
      self.remaining_ms -= delta_ms;
    } else {
      // Reached zero, start counting up
      self.is_elapsed = true;
      self.is_running = false;
    }
  }

  /// Get the formatted time string for display
  pub fn format_time(&self) -> String {
    let remaining = if self.is_elapsed {
      // Counting up
      (self.duration_ms as i64) - (self.remaining_ms as i64)
    } else {
      self.remaining_ms as i64
    };

    if remaining < 0 {
      let elapsed = (-remaining) as u64;
      format!("+{elapsed}ms")
    } else if remaining == 0 {
      "0:00.0".to_string()
    } else {
      let seconds = remaining / 1000;
      let millis = (remaining % 1000) / 100;
      let minutes = seconds / 60;
      let secs = seconds % 60;
      format!("{minutes}:{secs:02}.{millis}")
    }
  }

  /// Get the percentage of the countdown remaining
  pub fn progress(&self) -> f64 {
    if self.duration_ms == 0 {
      return 0.0;
    }
    (self.remaining_ms as f64) / (self.duration_ms as f64)
  }
}

/// Create a countdown from the current TimerState
pub fn countdown_from_state(state: &TimerState) -> Countdown {
  if state.is_counting_up {
    // Already elapsed, show as elapsed
    Countdown {
      duration_ms: state.countdown_duration_ms,
      remaining_ms: 0,
      is_elapsed: true,
      is_running: true,
    }
  } else {
    Countdown {
      duration_ms: state.countdown_duration_ms,
      remaining_ms: state.elapsed_ms,
      is_elapsed: false,
      is_running: true,
    }
  }
}
