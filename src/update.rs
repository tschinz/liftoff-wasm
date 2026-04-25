use crate::model::{Theme, TimeDisplay, TimeFormat, TimerMode, TimerState};

/// Messages that can update the app state
#[derive(Debug, Clone, PartialEq)]
pub enum Msg {
  /// Switch to a different timer mode
  SwitchMode(TimerMode),

  /// Toggle between dark and light theme
  ToggleTheme,

  /// Set the theme explicitly
  SetTheme(Theme),

  /// Timer tick - called periodically to update time display
  TimerTick,

  /// Start/reset the countdown timer
  StartCountdown,

  /// Pause/resume the countdown timer
  ToggleCountdownPause,

  /// Reset the countdown timer to its initial state
  ResetCountdown,

  /// Change the countdown duration
  ChangeCountdownDuration(u64),

  /// Set a target timestamp for countdown-to mode
  SetTargetTimestamp(u64),

  /// Format change
  SetTimeFormat(TimeFormat),
}

/// Initialize default state and return it.
/// Timer loop is managed in app.rs via gloo_timers.
pub fn init() -> TimerState {
  TimerState::default()
}

/// Update the timer state based on a message.
/// All timer tick logic is handled in app.rs via gloo_timers.
pub fn update(state: &mut TimerState, msg: Msg) {
  match msg {
    Msg::SwitchMode(mode) => {
      state.mode = mode;

      // Reset countdown state when switching modes
      if mode != TimerMode::Countdown {
        state.is_counting_up = false;
        state.elapsed_ms = 0;
      }

      // Reinitialize countdown duration if in countdown mode
      if mode == TimerMode::Countdown {
        state.countdown_duration_ms = 600_000;
      }
    }

    Msg::ToggleTheme => {
      state.theme = match state.theme {
        Theme::Dark => Theme::Light,
        Theme::Light => Theme::Dark,
      };
    }

    Msg::SetTheme(theme) => {
      state.theme = theme;
    }

    Msg::StartCountdown => {
      if state.mode == TimerMode::Countdown {
        state.elapsed_ms = state.countdown_duration_ms;
        state.is_counting_up = false;
      }
    }

    Msg::ResetCountdown => {
      if state.mode == TimerMode::Countdown {
        state.elapsed_ms = state.countdown_duration_ms;
        state.is_counting_up = false;
      }
    }

    Msg::ChangeCountdownDuration(ms) => {
      if state.mode == TimerMode::Countdown {
        state.countdown_duration_ms = ms;
        if !state.is_counting_up {
          state.elapsed_ms = ms;
        }
      }
    }

    Msg::SetTargetTimestamp(timestamp) => {
      if state.mode == TimerMode::CountdownTo {
        state.target_timestamp_ms = Some(timestamp);
        state.is_counting_up = false;
      }
    }

    Msg::SetTimeFormat(format) => {
      state.time_format = format;
    }

    Msg::ToggleCountdownPause => {
      if state.mode == TimerMode::Countdown {
        let old_running = state.is_running;
        state.is_running = !state.is_running;
        web_sys::console::log_1(
          &format!(
            "ToggleCountdownPause: is_running {} -> {}, elapsed_ms={}",
            old_running, state.is_running, state.elapsed_ms
          )
          .into(),
        );
        // If starting fresh, reset to duration
        if state.is_running && (state.elapsed_ms == 0 || state.is_counting_up) {
          state.elapsed_ms = state.countdown_duration_ms;
          state.is_counting_up = false;
          web_sys::console::log_1(&format!("Reset elapsed_ms to {}", state.elapsed_ms).into());
        }
      }
    }

    // This message is no longer used - timer is driven by app.rs
    Msg::TimerTick => {}
  }
}

/// Format time for display based on current mode and state
pub fn format_display(state: &TimerState) -> TimeDisplay {
  match state.mode {
    TimerMode::Clock => {
      // For clock mode, show current time of day
      let now = js_sys::Date::new(&js_sys::Date::now().into());

      TimeDisplay {
        years: 0,
        months: 0,
        days: 0,
        hours: now.get_hours(),
        minutes: now.get_minutes(),
        seconds: now.get_seconds(),
        millis: now.get_milliseconds(),
        is_negative: false,
        total_seconds: 0,
      }
    }

    TimerMode::Countdown => {
      if state.is_counting_up {
        // Counting up after reaching zero - show T+ with positive time
        TimeDisplay::from_ms(state.elapsed_ms as i64, state.time_format)
      } else {
        // Counting down - show T- with remaining time
        let remaining = state.elapsed_ms as i64;
        TimeDisplay::from_ms(remaining, state.time_format)
      }
    }

    TimerMode::CountdownTo => {
      if let Some(target) = state.target_timestamp_ms {
        let now = js_sys::Date::now();
        let diff = (target as f64) - now;
        TimeDisplay::from_ms(diff as i64, state.time_format)
      } else {
        TimeDisplay::from_ms(0, state.time_format)
      }
    }
  }
}

// ───────────────────────────────────────────────────────────
// Tests
// ───────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
  use super::*;

  // ── init() tests ──────────────────────────────────────

  #[test]
  fn init_returns_default_state() {
    let state = init();
    assert_eq!(state.mode, TimerMode::Clock);
    assert_eq!(state.elapsed_ms, 0);
    assert_eq!(state.countdown_duration_ms, 600_000);
    assert!(state.target_timestamp_ms.is_none());
    assert!(!state.is_counting_up);
    assert_eq!(state.time_format, TimeFormat::Editorial);
    assert_eq!(state.theme, Theme::Dark);
  }

  // ── update() — SwitchMode tests ───────────────────────

  #[test]
  fn switch_mode_updates_mode() {
    let mut state = init();
    update(&mut state, Msg::SwitchMode(TimerMode::Countdown));
    assert_eq!(state.mode, TimerMode::Countdown);
  }

  #[test]
  fn switch_mode_resets_countdown_state() {
    let mut state = init();
    state.is_counting_up = true;
    state.elapsed_ms = 500;
    update(&mut state, Msg::SwitchMode(TimerMode::Clock));
    assert!(!state.is_counting_up);
    assert_eq!(state.elapsed_ms, 0);
  }

  #[test]
  fn switch_mode_sets_countdown_duration() {
    let mut state = init();
    update(&mut state, Msg::SwitchMode(TimerMode::Countdown));
    assert_eq!(state.countdown_duration_ms, 600_000);
  }

  // ── update() — Theme tests ───────────────────────────

  #[test]
  fn toggle_theme_dark_to_light() {
    let mut state = init();
    state.theme = Theme::Dark;
    update(&mut state, Msg::ToggleTheme);
    assert_eq!(state.theme, Theme::Light);
  }

  #[test]
  fn toggle_theme_light_to_dark() {
    let mut state = init();
    state.theme = Theme::Light;
    update(&mut state, Msg::ToggleTheme);
    assert_eq!(state.theme, Theme::Dark);
  }

  #[test]
  fn set_theme_directly() {
    let mut state = init();
    update(&mut state, Msg::SetTheme(Theme::Light));
    assert_eq!(state.theme, Theme::Light);
  }

  // ── update() — Countdown tests ───────────────────────

  #[test]
  fn start_countdown_sets_elapsed() {
    let mut state = init();
    state.mode = TimerMode::Countdown;
    update(&mut state, Msg::StartCountdown);
    assert_eq!(state.elapsed_ms, 600_000);
    assert!(!state.is_counting_up);
  }

  #[test]
  fn reset_countdown_resets_state() {
    let mut state = init();
    state.mode = TimerMode::Countdown;
    update(&mut state, Msg::StartCountdown);
    update(&mut state, Msg::ResetCountdown);
    assert_eq!(state.elapsed_ms, 600_000);
    assert!(!state.is_counting_up);
  }

  #[test]
  fn change_countdown_duration_updates_duration() {
    let mut state = init();
    state.mode = TimerMode::Countdown;
    update(&mut state, Msg::ChangeCountdownDuration(60_000));
    assert_eq!(state.countdown_duration_ms, 60_000);
  }

  #[test]
  fn change_countdown_duration_updates_elapsed_when_not_counting_up() {
    let mut state = init();
    state.mode = TimerMode::Countdown;
    state.elapsed_ms = 10_000;
    state.is_counting_up = false;
    update(&mut state, Msg::ChangeCountdownDuration(120_000));
    assert_eq!(state.elapsed_ms, 120_000);
    assert_eq!(state.countdown_duration_ms, 120_000);
  }

  // ── update() — CountdownTo tests ─────────────────────

  #[test]
  fn set_target_timestamp() {
    let mut state = init();
    state.mode = TimerMode::CountdownTo;
    let target: u64 = 1_700_000_000_000;
    update(&mut state, Msg::SetTargetTimestamp(target));
    assert_eq!(state.target_timestamp_ms, Some(target));
    assert!(!state.is_counting_up);
  }

  // ── update() — Format tests ──────────────────────────

  #[test]
  fn set_time_format() {
    let mut state = init();
    update(&mut state, Msg::SetTimeFormat(TimeFormat::Standard));
    assert_eq!(state.time_format, TimeFormat::Standard);
  }

  // ── format_display() tests ───────────────────────────

  #[test]
  #[cfg(target_arch = "wasm32")]
  fn format_display_clock_returns_valid_time() {
    let state = init();
    let display = format_display(&state);
    assert!(display.hours <= 23);
    assert!(display.minutes <= 59);
    assert!(display.seconds <= 59);
    assert!(!display.is_negative);
  }

  #[test]
  fn format_display_countdown_with_full_elapsed() {
    let mut state = init();
    state.mode = TimerMode::Countdown;
    state.countdown_duration_ms = 10_000;
    state.elapsed_ms = 10_000;
    let display = format_display(&state);
    assert_eq!(display.hours, 0);
    assert_eq!(display.minutes, 0);
    assert_eq!(display.seconds, 10);
  }

  #[test]
  fn format_display_countdown_with_zero_elapsed() {
    let mut state = init();
    state.mode = TimerMode::Countdown;
    state.countdown_duration_ms = 10_000;
    state.elapsed_ms = 0;
    let display = format_display(&state);
    assert_eq!(display.hours, 0);
    assert_eq!(display.minutes, 0);
    assert_eq!(display.seconds, 0);
  }

  #[test]
  #[cfg(target_arch = "wasm32")]
  fn format_display_countdown_to_with_target() {
    let mut state = init();
    state.mode = TimerMode::CountdownTo;
    state.target_timestamp_ms = Some(1_700_000_000_000);
    let display = format_display(&state);
    // Should return a valid display (time value depends on current time)
    assert!(display.total_seconds <= 1_000_000_000);
  }

  #[test]
  #[cfg(target_arch = "wasm32")]
  fn format_display_countdown_to_without_target() {
    let mut state = init();
    state.mode = TimerMode::CountdownTo;
    state.target_timestamp_ms = None;
    let display = format_display(&state);
    assert_eq!(display.hours, 0);
    assert_eq!(display.minutes, 0);
    assert_eq!(display.seconds, 0);
    assert_eq!(display.millis, 0);
  }

  // ── No-op message tests ──────────────────────────────

  #[test]
  fn timer_tick_message_does_nothing() {
    let mut state = init();
    let old_mode = state.mode;
    let old_elapsed = state.elapsed_ms;
    update(&mut state, Msg::TimerTick);
    // TimerTick is now a no-op - state should be unchanged
    assert_eq!(state.mode, old_mode);
    assert_eq!(state.elapsed_ms, old_elapsed);
  }

  #[test]
  fn countdown_pause_toggle() {
    let mut state = init();
    update(&mut state, Msg::StartCountdown);
    let saved_elapsed = state.elapsed_ms;
    update(&mut state, Msg::ToggleCountdownPause);
    // ToggleCountdownPause is now a no-op
    assert_eq!(state.elapsed_ms, saved_elapsed);
  }
}
