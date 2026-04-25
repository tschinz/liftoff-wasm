use serde::{Deserialize, Serialize};

/// The three modes the app can operate in
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TimerMode {
  /// Current time display (always running)
  Clock,
  /// Countdown from a user-specified duration
  /// Counts down to zero, then continues counting up
  Countdown,
  /// Countdown to a specific target date/time
  CountdownTo,
}

impl TimerMode {
  /// Display label for this mode
  pub fn label(&self) -> &'static str {
    match self {
      TimerMode::Clock => "Clock",
      TimerMode::Countdown => "Timer",
      TimerMode::CountdownTo => "Countdown To",
    }
  }
}

/// The direction/time display format for the countdown
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TimeFormat {
  /// HH:MM:SS display
  Standard,
  /// HhMmSs (editorial) display
  Editorial,
}

impl TimeFormat {
  /// Check if this format should show leading zeros
  pub fn show_leading_zeros(&self) -> bool {
    matches!(self, TimeFormat::Standard)
  }
}

/// Dark or light theme preference
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Theme {
  Dark,
  Light,
}

impl Theme {
  /// Get the CSS variable prefix for this theme
  pub fn css_class(&self) -> &'static str {
    match self {
      Theme::Dark => "theme-dark",
      Theme::Light => "theme-light",
    }
  }

  /// Default icon character for toggle button
  pub fn icon(&self) -> char {
    match self {
      Theme::Dark => '☀',
      Theme::Light => '☾',
    }
  }
}

/// Core timer state shared across all modes
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimerState {
  /// The currently active mode
  pub mode: TimerMode,

  /// Total elapsed time in milliseconds (for Clock and + side of Countdown)
  pub elapsed_ms: u64,

  /// For Countdown mode: the initial duration in milliseconds
  pub countdown_duration_ms: u64,

  /// For CountdownTo mode: the target timestamp in milliseconds since epoch
  pub target_timestamp_ms: Option<u64>,

  /// Whether the countdown has reached zero and is now counting up
  pub is_counting_up: bool,

  /// Whether the countdown is currently running (not paused)
  pub is_running: bool,

  /// Current time offset in seconds (for synchronization with browser)
  pub offset_seconds: f64,

  /// The display format to use
  pub time_format: TimeFormat,

  /// Current theme
  pub theme: Theme,

  /// Whether to show seconds and milliseconds (true) or just hours and minutes (false)
  pub show_seconds: bool,
}

impl Default for TimerState {
  fn default() -> Self {
    Self {
      mode: TimerMode::Clock,
      elapsed_ms: 0,
      countdown_duration_ms: 600_000,
      target_timestamp_ms: None,
      is_counting_up: false,
      is_running: false,
      offset_seconds: 0.0,
      time_format: TimeFormat::Editorial,
      theme: Theme::Dark,
      show_seconds: true,
    }
  }
}

/// Parsed time values for display
#[derive(Debug, Clone, PartialEq)]
pub struct TimeDisplay {
  /// Years part
  pub years: u32,
  /// Months part
  pub months: u32,
  /// Days part
  pub days: u32,
  /// Hours part
  pub hours: u32,
  /// Minutes part
  pub minutes: u32,
  /// Seconds part
  pub seconds: u32,
  /// Milliseconds part (for precision)
  pub millis: u32,
  /// Whether the value is negative (for countdown before target)
  pub is_negative: bool,
  /// The total elapsed seconds (for display logic)
  pub total_seconds: u64,
}

impl TimeDisplay {
  /// Create a time display from milliseconds, showing negative when appropriate
  pub fn from_ms(total_ms: i64, _format: TimeFormat) -> Self {
    let is_negative = total_ms < 0;
    let abs_ms = total_ms.unsigned_abs();

    let total_seconds = abs_ms / 1000;
    let millis = (abs_ms % 1000) as u32;

    // Calculate larger time units
    let mut remaining_seconds = total_seconds;

    // Approximate time units (not exact calendar calculations)
    let seconds_per_minute = 60;
    let seconds_per_hour = 3600;
    let seconds_per_day = 86400;
    let seconds_per_month = 2592000; // 30 days
    let seconds_per_year = 31536000; // 365 days

    let years = (remaining_seconds / seconds_per_year) as u32;
    remaining_seconds %= seconds_per_year;

    let months = (remaining_seconds / seconds_per_month) as u32;
    remaining_seconds %= seconds_per_month;

    let days = (remaining_seconds / seconds_per_day) as u32;
    remaining_seconds %= seconds_per_day;

    let hours = (remaining_seconds / seconds_per_hour) as u32;
    remaining_seconds %= seconds_per_hour;

    let minutes = (remaining_seconds / seconds_per_minute) as u32;
    let seconds = (remaining_seconds % seconds_per_minute) as u32;

    Self {
      years,
      months,
      days,
      hours,
      minutes,
      seconds,
      millis,
      is_negative,
      total_seconds,
    }
  }

  /// Check if we should show larger time units (days, months, years)
  pub fn has_large_units(&self) -> bool {
    self.years > 0 || self.months > 0 || self.days > 0
  }

  /// Format years string
  pub fn years_str(&self) -> String {
    self.years.to_string()
  }

  /// Format months string
  pub fn months_str(&self) -> String {
    self.months.to_string()
  }

  /// Format days string
  pub fn days_str(&self) -> String {
    self.days.to_string()
  }

  /// Format hours with or without leading zeros based on format
  pub fn hours_str(&self) -> String {
    if self.has_large_units() {
      // When showing larger units, hours don't need leading zeros
      self.hours.to_string()
    } else if self.hours < 100 && !self.is_negative {
      // For values under 100 hours, don't show leading zeros
      self.hours.to_string()
    } else {
      format!("{:02}", self.hours)
    }
  }

  /// Format minutes with or without leading zeros
  pub fn minutes_str(&self) -> String {
    format!("{:02}", self.minutes)
  }

  /// Format seconds with or without leading zeros
  pub fn seconds_str(&self) -> String {
    format!("{:02}", self.seconds)
  }

  /// Format milliseconds without leading zero (just first digit)
  pub fn millis_str(&self) -> String {
    (self.millis / 100).to_string()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  // ── TimerMode tests ──────────────────────────────────────────

  #[test]
  fn timer_mode_label_clock() {
    assert_eq!(TimerMode::Clock.label(), "Clock");
  }

  #[test]
  fn timer_mode_label_countdown() {
    assert_eq!(TimerMode::Countdown.label(), "Timer");
  }

  #[test]
  fn timer_mode_label_countdown_to() {
    assert_eq!(TimerMode::CountdownTo.label(), "Countdown To");
  }

  #[test]
  fn timer_mode_equality() {
    assert_eq!(TimerMode::Clock, TimerMode::Clock);
    assert_ne!(TimerMode::Clock, TimerMode::Countdown);
    assert_ne!(TimerMode::Countdown, TimerMode::CountdownTo);
  }

  #[test]
  fn timer_mode_copy() {
    let mode = TimerMode::Clock;
    let copied = mode;
    assert_eq!(mode, copied);
  }

  // ── TimeFormat tests ─────────────────────────────────────────

  #[test]
  fn time_format_standard_show_leading_zeros() {
    assert!(TimeFormat::Standard.show_leading_zeros());
  }

  #[test]
  fn time_format_editorial_no_leading_zeros() {
    assert!(!TimeFormat::Editorial.show_leading_zeros());
  }

  #[test]
  fn time_format_equality() {
    assert_eq!(TimeFormat::Standard, TimeFormat::Standard);
    assert_ne!(TimeFormat::Standard, TimeFormat::Editorial);
  }

  // ── Theme tests ──────────────────────────────────────────────

  #[test]
  fn theme_css_class_dark() {
    assert_eq!(Theme::Dark.css_class(), "theme-dark");
  }

  #[test]
  fn theme_css_class_light() {
    assert_eq!(Theme::Light.css_class(), "theme-light");
  }

  #[test]
  fn theme_icon_dark() {
    assert_eq!(Theme::Dark.icon(), '☀');
  }

  #[test]
  fn theme_icon_light() {
    assert_eq!(Theme::Light.icon(), '☾');
  }

  #[test]
  fn theme_equality() {
    assert_eq!(Theme::Dark, Theme::Dark);
    assert_ne!(Theme::Dark, Theme::Light);
  }

  #[test]
  fn theme_toggle_dark_to_light() {
    let dark = Theme::Dark;
    let light = match dark {
      Theme::Dark => Theme::Light,
      Theme::Light => Theme::Dark,
    };
    assert_eq!(light, Theme::Light);
  }

  #[test]
  fn theme_toggle_light_to_dark() {
    let light = Theme::Light;
    let dark = match light {
      Theme::Dark => Theme::Light,
      Theme::Light => Theme::Dark,
    };
    assert_eq!(dark, Theme::Dark);
  }

  // ── TimerState tests ─────────────────────────────────────────

  #[test]
  fn timer_state_default_mode() {
    let state = TimerState::default();
    assert_eq!(state.mode, TimerMode::Clock);
  }

  #[test]
  fn timer_state_default_elapsed() {
    let state = TimerState::default();
    assert_eq!(state.elapsed_ms, 0);
  }

  #[test]
  fn timer_state_default_countdown_duration() {
    let state = TimerState::default();
    assert_eq!(state.countdown_duration_ms, 600_000);
  }

  #[test]
  fn timer_state_default_target_none() {
    let state = TimerState::default();
    assert!(state.target_timestamp_ms.is_none());
  }

  #[test]
  fn timer_state_default_not_counting_up() {
    let state = TimerState::default();
    assert!(!state.is_counting_up);
  }

  #[test]
  fn timer_state_default_offset() {
    let state = TimerState::default();
    assert_eq!(state.offset_seconds, 0.0);
  }

  #[test]
  fn timer_state_default_format() {
    let state = TimerState::default();
    assert_eq!(state.time_format, TimeFormat::Editorial);
  }

  #[test]
  fn timer_state_default_theme() {
    let state = TimerState::default();
    assert_eq!(state.theme, Theme::Dark);
  }

  #[test]
  fn timer_state_clone() {
    let mut state = TimerState {
      mode: TimerMode::Countdown,
      ..Default::default()
    };
    let cloned = state.clone();
    assert_eq!(state.mode, cloned.mode);
    state.mode = TimerMode::Clock;
    assert_ne!(state.mode, cloned.mode);
  }

  #[test]
  fn timer_state_serde() {
    let state = TimerState::default();
    let json = serde_json::to_string(&state).expect("serde serialize failed");
    let restored: TimerState = serde_json::from_str(&json).expect("serde deserialize failed");
    assert_eq!(state.mode, restored.mode);
    assert_eq!(state.theme, restored.theme);
  }

  // ── TimeDisplay tests ────────────────────────────────────────

  #[test]
  fn time_display_from_ms_zero() {
    let display = TimeDisplay::from_ms(0, TimeFormat::Standard);
    assert_eq!(display.years, 0);
    assert_eq!(display.months, 0);
    assert_eq!(display.days, 0);
    assert_eq!(display.hours, 0);
    assert_eq!(display.minutes, 0);
    assert_eq!(display.seconds, 0);
    assert_eq!(display.millis, 0);
    assert!(!display.is_negative);
  }

  #[test]
  fn time_display_from_ms_negative() {
    let display = TimeDisplay::from_ms(-5000, TimeFormat::Standard);
    assert_eq!(display.hours, 0);
    assert_eq!(display.minutes, 0);
    assert_eq!(display.seconds, 5);
    assert_eq!(display.millis, 0);
    assert!(display.is_negative);
  }

  #[test]
  fn time_display_from_ms_one_hour() {
    let display = TimeDisplay::from_ms(3_600_000, TimeFormat::Standard);
    assert_eq!(display.hours, 1);
    assert_eq!(display.minutes, 0);
    assert_eq!(display.seconds, 0);
    assert!(!display.is_negative);
  }

  #[test]
  fn time_display_from_ms_two_hours_thirty_minutes() {
    let display = TimeDisplay::from_ms(9_000_000, TimeFormat::Standard);
    assert_eq!(display.hours, 2);
    assert_eq!(display.minutes, 30);
    assert_eq!(display.seconds, 0);
  }

  #[test]
  fn time_display_from_ms_with_milliseconds() {
    let display = TimeDisplay::from_ms(1_234_567, TimeFormat::Standard);
    assert_eq!(display.hours, 0);
    assert_eq!(display.minutes, 20);
    assert_eq!(display.seconds, 34);
    assert_eq!(display.millis, 567);
  }

  #[test]
  fn time_display_from_ms_large_value() {
    let display = TimeDisplay::from_ms(86_400_000, TimeFormat::Standard);
    assert_eq!(display.days, 1);
    assert_eq!(display.hours, 0);
    assert_eq!(display.minutes, 0);
    assert_eq!(display.seconds, 0);
  }

  #[test]
  fn time_display_hours_str_under_100_no_leading_zeros() {
    let display = TimeDisplay {
      years: 0,
      months: 0,
      days: 0,
      hours: 5,
      minutes: 0,
      seconds: 0,
      millis: 0,
      is_negative: false,
      total_seconds: 0,
    };
    assert_eq!(display.hours_str(), "5");
  }

  #[test]
  fn time_display_hours_str_over_100_with_leading_zeros() {
    let display = TimeDisplay {
      years: 0,
      months: 0,
      days: 0,
      hours: 123,
      minutes: 0,
      seconds: 0,
      millis: 0,
      is_negative: false,
      total_seconds: 0,
    };
    assert_eq!(display.hours_str(), "123");
  }

  #[test]
  fn time_display_minutes_str_always_two_digits() {
    let display = TimeDisplay {
      years: 0,
      months: 0,
      days: 0,
      hours: 0,
      minutes: 5,
      seconds: 0,
      millis: 0,
      is_negative: false,
      total_seconds: 0,
    };
    assert_eq!(display.minutes_str(), "05");
  }

  #[test]
  fn time_display_seconds_str_always_two_digits() {
    let display = TimeDisplay {
      years: 0,
      months: 0,
      days: 0,
      hours: 0,
      minutes: 0,
      seconds: 5,
      millis: 0,
      is_negative: false,
      total_seconds: 0,
    };
    assert_eq!(display.seconds_str(), "05");
  }

  #[test]
  fn time_display_millis_str_first_digit() {
    let display = TimeDisplay {
      years: 0,
      months: 0,
      days: 0,
      hours: 0,
      minutes: 0,
      seconds: 0,
      millis: 156,
      is_negative: false,
      total_seconds: 0,
    };
    assert_eq!(display.millis_str(), "1");
  }

  #[test]
  fn time_display_millis_str_hundred() {
    let display = TimeDisplay {
      years: 0,
      months: 0,
      days: 0,
      hours: 0,
      minutes: 0,
      seconds: 0,
      millis: 999,
      is_negative: false,
      total_seconds: 0,
    };
    assert_eq!(display.millis_str(), "9");
  }

  #[test]
  fn time_display_clone() {
    let display = TimeDisplay::from_ms(3_600_000, TimeFormat::Standard);
    let cloned = display.clone();
    assert_eq!(display.hours, cloned.hours);
    assert_eq!(display.minutes, cloned.minutes);
    assert_eq!(display.seconds, cloned.seconds);
  }

  #[test]
  fn time_display_negative_value_hours() {
    let display = TimeDisplay::from_ms(-7_200_000, TimeFormat::Standard);
    assert!(display.is_negative);
    assert_eq!(display.hours, 2);
    assert_eq!(display.minutes, 0);
    assert_eq!(display.seconds, 0);
  }

  #[test]
  fn time_display_from_ms_exactly_one_second() {
    let display = TimeDisplay::from_ms(1_000, TimeFormat::Standard);
    assert_eq!(display.hours, 0);
    assert_eq!(display.minutes, 0);
    assert_eq!(display.seconds, 1);
    assert_eq!(display.millis, 0);
  }

  #[test]
  fn time_display_from_ms_999_milliseconds() {
    let display = TimeDisplay::from_ms(999, TimeFormat::Standard);
    assert_eq!(display.hours, 0);
    assert_eq!(display.minutes, 0);
    assert_eq!(display.seconds, 0);
    assert_eq!(display.millis, 999);
  }

  #[test]
  fn time_display_from_ms_total_seconds() {
    let display = TimeDisplay::from_ms(65_000, TimeFormat::Standard);
    assert_eq!(display.total_seconds, 65);
  }

  #[test]
  fn time_display_from_ms_format_agnostric() {
    let display_editorial = TimeDisplay::from_ms(3_600_000, TimeFormat::Editorial);
    let display_standard = TimeDisplay::from_ms(3_600_000, TimeFormat::Standard);
    // Format doesn't affect the numeric values
    assert_eq!(display_editorial.hours, display_standard.hours);
    assert_eq!(display_editorial.minutes, display_standard.minutes);
    assert_eq!(display_editorial.seconds, display_standard.seconds);
  }
}
