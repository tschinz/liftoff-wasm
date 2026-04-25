//! Integration tests for the liftoff application.
//!
//! These tests verify the overall behavior of the timer components,
//! including mode switching, state transitions, and display logic.
//! They are run as a separate binary to test cross-module interactions.

use liftoff::model::{Theme, TimeDisplay, TimeFormat, TimerMode, TimerState};
use liftoff::update::{format_display, init, update, Msg};

// ──────────────────────────────────────────────────────────
// Helper functions
// ──────────────────────────────────────────────────────────

/// Create a fresh TimerState for testing
fn fresh_state() -> TimerState {
  TimerState::default()
}

/// Create a countdown timer state with a custom duration
fn countdown_state(duration_ms: u64) -> TimerState {
  TimerState {
    mode: TimerMode::Countdown,
    countdown_duration_ms: duration_ms,
    ..Default::default()
  }
}

/// Create a countdown-to state with a target timestamp
fn countdown_to_state(target_ms: u64) -> TimerState {
  TimerState {
    mode: TimerMode::CountdownTo,
    target_timestamp_ms: Some(target_ms),
    ..Default::default()
  }
}

// ──────────────────────────────────────────────────────────
// TimerState lifecycle tests
// ──────────────────────────────────────────────────────────

#[test]
fn timer_state_default_is_valid() {
  let state = fresh_state();
  assert_eq!(state.mode, TimerMode::Clock);
  assert_eq!(state.theme, Theme::Dark);
  assert_eq!(state.countdown_duration_ms, 600_000);
}

#[test]
fn timer_state_clone_preserves_values() {
  let mut original = fresh_state();
  original.mode = TimerMode::Countdown;
  original.elapsed_ms = 5000;
  original.is_counting_up = true;

  let cloned = original.clone();
  assert_eq!(cloned.mode, TimerMode::Countdown);
  assert_eq!(cloned.elapsed_ms, 5000);
  assert!(cloned.is_counting_up);
}

// ──────────────────────────────────────────────────────────
// Mode switching tests
// ──────────────────────────────────────────────────────────

#[test]
fn switch_from_clock_to_countdown_works() {
  let mut state = fresh_state();
  update(&mut state, Msg::SwitchMode(TimerMode::Countdown));
  assert_eq!(state.mode, TimerMode::Countdown);
  assert_eq!(state.countdown_duration_ms, 600_000);
}

#[test]
fn switch_from_countdown_to_clock_resets_state() {
  let mut state = countdown_state(10_000);
  update(&mut state, Msg::StartCountdown);
  update(&mut state, Msg::SwitchMode(TimerMode::Clock));
  assert_eq!(state.mode, TimerMode::Clock);
  assert_eq!(state.elapsed_ms, 0);
  assert!(!state.is_counting_up);
}

#[test]
fn switch_from_countdown_to_countdown_to_works() {
  let mut state = countdown_state(10_000);
  update(&mut state, Msg::SwitchMode(TimerMode::CountdownTo));
  assert_eq!(state.mode, TimerMode::CountdownTo);
}

#[test]
fn switching_modes_resets_countdown_timer() {
  let mut state = countdown_state(10_000);
  update(&mut state, Msg::StartCountdown);
  state.elapsed_ms = 5000; // Partial countdown

  update(&mut state, Msg::SwitchMode(TimerMode::Countdown));

  // Should reset to full duration
  assert_eq!(state.countdown_duration_ms, 600_000);
}

// ──────────────────────────────────────────────────────────
// Countdown timer tests
// ──────────────────────────────────────────────────────────

#[test]
fn start_countdown_works() {
  let mut state = countdown_state(10_000);
  update(&mut state, Msg::StartCountdown);
  assert_eq!(state.elapsed_ms, 10_000);
  assert!(!state.is_counting_up);
}

#[test]
fn countdown_reaches_zero_and_starts_counting_up() {
  let mut state = countdown_state(10_000);
  update(&mut state, Msg::StartCountdown);
  // Simulate time passing by manually setting elapsed to equal duration
  state.elapsed_ms = 10_000;
  assert!(state.is_counting_up);
}

#[test]
fn countdown_timer_resets_correctly() {
  let mut state = countdown_state(10_000);
  update(&mut state, Msg::StartCountdown);
  state.elapsed_ms = 5000; // Partial

  update(&mut state, Msg::ResetCountdown);
  assert_eq!(state.elapsed_ms, 10_000);
  assert!(!state.is_counting_up);
}

#[test]
fn change_countdown_duration_updates_timer() {
  let mut state = countdown_state(10_000);
  update(&mut state, Msg::ChangeCountdownDuration(60_000));
  assert_eq!(state.countdown_duration_ms, 60_000);
}

#[test]
fn countdown_duration_zero_does_not_panic() {
  let mut state = countdown_state(0);
  update(&mut state, Msg::StartCountdown);
  // Should not panic
  assert_eq!(state.elapsed_ms, 0);
}

// ──────────────────────────────────────────────────────────
// Countdown to timestamp tests
// ──────────────────────────────────────────────────────────

#[test]
fn set_target_timestamp_works() {
  let mut state = fresh_state();
  let target: u64 = 1_700_000_000_000; // A future timestamp
  update(&mut state, Msg::SetTargetTimestamp(target));
  assert_eq!(state.target_timestamp_ms, Some(target));
}

#[test]
fn set_target_timestamp_clears_counting_up() {
  let mut state = countdown_to_state(1_700_000_000_000);
  state.is_counting_up = true;
  update(&mut state, Msg::SetTargetTimestamp(1_700_000_000_000));
  assert!(!state.is_counting_up);
}

// ──────────────────────────────────────────────────────────
// Theme toggle tests
// ──────────────────────────────────────────────────────────

#[test]
fn toggle_theme_cycles_dark_light() {
  let mut state = fresh_state();
  assert_eq!(state.theme, Theme::Dark);

  update(&mut state, Msg::ToggleTheme);
  assert_eq!(state.theme, Theme::Light);

  update(&mut state, Msg::ToggleTheme);
  assert_eq!(state.theme, Theme::Dark);
}

#[test]
fn set_theme_explicitly_works() {
  let mut state = fresh_state();
  update(&mut state, Msg::SetTheme(Theme::Light));
  assert_eq!(state.theme, Theme::Light);
}

// ──────────────────────────────────────────────────────────
// Time display formatting tests
// ──────────────────────────────────────────────────────────

#[test]
fn format_display_clock_returns_valid_hours() {
  let state = fresh_state();
  let display = format_display(&state);
  assert!(display.hours < 24);
}

#[test]
fn format_display_clock_returns_valid_minutes() {
  let state = fresh_state();
  let display = format_display(&state);
  assert!(display.minutes < 60);
}

#[test]
fn format_display_clock_returns_valid_seconds() {
  let state = fresh_state();
  let display = format_display(&state);
  assert!(display.seconds < 60);
}

#[test]
fn format_display_countdown_full_duration() {
  let state = countdown_state(60_000);
  let display = format_display(&state);
  assert_eq!(display.minutes, 1);
  assert_eq!(display.seconds, 0);
}

#[test]
fn format_display_countdown_partial_duration() {
  let mut state = countdown_state(65_000);
  state.elapsed_ms = 5_000;
  let display = format_display(&state);
  assert_eq!(display.minutes, 1);
  assert_eq!(display.seconds, 0);
}

#[test]
fn format_display_countdown_to_returns_valid() {
  let state = countdown_to_state(1_700_000_000_000);
  let display = format_display(&state);
  // Should return a valid display without panicking
  // total_seconds is u64, so it's always >= 0
  assert!(display.total_seconds < 1_000_000_000);
}

#[test]
fn format_display_without_target_returns_zeros() {
  let mut state = countdown_to_state(1_700_000_000_000);
  state.target_timestamp_ms = None;
  let display = format_display(&state);
  assert_eq!(display.hours, 0);
  assert_eq!(display.minutes, 0);
  assert_eq!(display.seconds, 0);
}

// ──────────────────────────────────────────────────────────
// TimeDisplay utility tests
// ──────────────────────────────────────────────────────────

#[test]
fn time_display_from_ms_converts_correctly() {
  let display = TimeDisplay::from_ms(3_661_234, TimeFormat::Standard);
  assert_eq!(display.hours, 1);
  assert_eq!(display.minutes, 1);
  assert_eq!(display.seconds, 1);
  assert_eq!(display.millis, 234);
}

#[test]
fn time_display_from_negative_ms_shows_negative() {
  let display = TimeDisplay::from_ms(-3_600_000, TimeFormat::Standard);
  assert!(display.is_negative);
  assert_eq!(display.hours, 1);
}

#[test]
fn time_display_hours_str_short_hours() {
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
fn time_display_minutes_always_two_digits() {
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
fn time_display_seconds_always_two_digits() {
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

// ──────────────────────────────────────────────────────────
// Edge case tests
// ──────────────────────────────────────────────────────────

#[test]
fn timer_with_zero_duration_does_not_panic() {
  let mut state = countdown_state(0);
  update(&mut state, Msg::StartCountdown);
  update(&mut state, Msg::ResetCountdown);
  // Should not panic
}

#[test]
fn timer_with_very_large_duration_works() {
  let mut state = countdown_state(u64::MAX / 2);
  update(&mut state, Msg::StartCountdown);
  assert_eq!(state.elapsed_ms, u64::MAX / 2);
}

#[test]
fn theme_serde_roundtrip() {
  let state = fresh_state();
  let json = serde_json::to_string(&state).expect("serde failed");
  let restored: TimerState = serde_json::from_str(&json).expect("serde failed");
  assert_eq!(state.theme, restored.theme);
  assert_eq!(state.mode, restored.mode);
}

#[test]
fn all_timer_modes_have_labels() {
  assert!(!TimerMode::Clock.label().is_empty());
  assert!(!TimerMode::Countdown.label().is_empty());
  assert!(!TimerMode::CountdownTo.label().is_empty());
}

#[test]
fn theme_icons_are_not_empty() {
  let dark_icon = Theme::Dark.icon();
  let light_icon = Theme::Light.icon();
  assert!(dark_icon as usize > 0);
  assert!(light_icon as usize > 0);
}

// ──────────────────────────────────────────────────────────
// Cross-module interaction tests
// ──────────────────────────────────────────────────────────

#[test]
fn full_user_journey_clock_to_countdown() {
  // Start with clock
  let mut state = init();
  assert_eq!(state.mode, TimerMode::Clock);

  // Switch to countdown
  update(&mut state, Msg::SwitchMode(TimerMode::Countdown));
  assert_eq!(state.mode, TimerMode::Countdown);

  // Start countdown
  update(&mut state, Msg::StartCountdown);
  assert_eq!(state.elapsed_ms, 600_000);

  // Change duration
  update(&mut state, Msg::ChangeCountdownDuration(60_000));
  assert_eq!(state.countdown_duration_ms, 60_000);

  // Reset
  update(&mut state, Msg::ResetCountdown);
  assert_eq!(state.elapsed_ms, 60_000);

  // Switch back to clock
  update(&mut state, Msg::SwitchMode(TimerMode::Clock));
  assert_eq!(state.mode, TimerMode::Clock);
  assert_eq!(state.elapsed_ms, 0);
}

#[test]
fn full_user_journey_countdown_to_countdown_to() {
  // Start with countdown
  let mut state = init();
  update(&mut state, Msg::SwitchMode(TimerMode::Countdown));
  update(&mut state, Msg::StartCountdown);

  // Switch to countdown-to
  let future_target: u64 = 1_700_000_000_000;
  update(&mut state, Msg::SwitchMode(TimerMode::CountdownTo));
  update(&mut state, Msg::SetTargetTimestamp(future_target));
  assert_eq!(state.target_timestamp_ms, Some(future_target));

  // Toggle theme
  update(&mut state, Msg::ToggleTheme);
  assert_eq!(state.theme, Theme::Light);
}

#[test]
fn theme_toggle_during_countdown_preserves_state() {
  let mut state = init();
  update(&mut state, Msg::SwitchMode(TimerMode::Countdown));
  update(&mut state, Msg::StartCountdown);

  let saved_elapsed = state.elapsed_ms;
  let saved_mode = state.mode;

  update(&mut state, Msg::ToggleTheme);
  update(&mut state, Msg::ToggleTheme);

  assert_eq!(state.elapsed_ms, saved_elapsed);
  assert_eq!(state.mode, saved_mode);
  assert_eq!(state.theme, Theme::Dark);
}
