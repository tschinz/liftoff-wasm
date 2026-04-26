use web_sys::{Event, HtmlInputElement, WheelEvent};
use yew::prelude::*;

use crate::model::{Theme, TimeDisplay, TimerMode, TimerState};
use crate::update::Msg;

/// Props for the time display component
#[derive(Properties, PartialEq)]
pub struct TimeDisplayProps {
  pub time_display: TimeDisplay,
  pub mode: TimerMode,
  pub on_wheel: Option<Callback<(String, f64)>>,
  pub show_seconds: bool,
  pub is_counting_up: bool,
}

/// Component that renders the large, striking time display
#[function_component(TimeDisplayComponent)]
pub fn time_display_component(props: &TimeDisplayProps) -> Html {
  let TimeDisplayProps {
    time_display,
    mode,
    on_wheel,
    show_seconds,
    is_counting_up,
  } = props;

  let format_class = match mode {
    TimerMode::Clock => "clock",
    TimerMode::Countdown => "countdown",
    TimerMode::CountdownTo => "countdown-to",
  };

  let display_class = classes!(
    "time-display-value",
    format_class,
    time_display.is_negative.then_some("negative"),
    (*is_counting_up && *mode == TimerMode::Countdown).then_some("counting-up"),
    (time_display.is_negative && *mode == TimerMode::CountdownTo).then_some("counting-up"),
  );

  // Build the formatted time string for accessibility
  let time_str = if time_display.has_large_units() {
    let mut parts = Vec::new();
    if time_display.years > 0 {
      parts.push(format!("{}y", time_display.years));
    }
    if time_display.months > 0 {
      parts.push(format!("{}mo", time_display.months));
    }
    if time_display.days > 0 {
      parts.push(format!("{}d", time_display.days));
    }
    parts.push(format!("{}:{:02}", time_display.hours, time_display.minutes));
    if *show_seconds {
      parts.push(format!(":{:02}", time_display.seconds));
    }
    parts.join(" ")
  } else if *show_seconds {
    format!(
      "{}:{:02}:{:02}.{}",
      time_display.hours,
      time_display.minutes,
      time_display.seconds,
      time_display.millis_str(),
    )
  } else {
    format!("{}:{:02}", time_display.hours, time_display.minutes,)
  };

  // Create wheel handlers for each digit if callback is provided
  let create_wheel_handler = |unit: &'static str, on_wheel: &Option<Callback<(String, f64)>>| {
    if let Some(callback) = on_wheel {
      let callback = callback.clone();
      Callback::from(move |e: WheelEvent| {
        e.prevent_default();
        let delta = if e.delta_y() < 0.0 { 1.0 } else { -1.0 };
        callback.emit((unit.to_string(), delta));
      })
    } else {
      Callback::from(|_: WheelEvent| {})
    }
  };

  let years_wheel = create_wheel_handler("years", on_wheel);
  let months_wheel = create_wheel_handler("months", on_wheel);
  let days_wheel = create_wheel_handler("days", on_wheel);
  let hours_wheel = create_wheel_handler("hours", on_wheel);
  let minutes_wheel = create_wheel_handler("minutes", on_wheel);
  let seconds_wheel = create_wheel_handler("seconds", on_wheel);

  let is_scrollable = on_wheel.is_some();

  // Determine prefix for Timer and Countdown To modes
  let prefix = if *mode == TimerMode::Countdown {
    if *is_counting_up {
      Some("T+")
    } else {
      Some("T-")
    }
  } else if *mode == TimerMode::CountdownTo {
    if time_display.is_negative {
      Some("T+")
    } else {
      Some("T-")
    }
  } else {
    None
  };

  html! {
      <div class="time-display-container">
          <div class={display_class} aria_label={format!("Timer display: {time_str}")}>
              {
                  if let Some(prefix_str) = prefix {
                      html! {
                          <span class="time-prefix" style="margin-right: 0.3em; color: var(--text-tertiary);">
                              {prefix_str}
                          </span>
                      }
                  } else {
                      html! {}
                  }
              }
              {
                  if time_display.has_large_units() {
                      // Show long-form units (years, months, days) then HH:MM:SS format
                      // Show all units including zeros once any larger unit appears
                      html! {
                          <>
                              {if time_display.years > 0 {
                                  html! {
                                      <>
                                          <span
                                              class="time-digit"
                                              title={is_scrollable.then_some("Scroll to adjust years")}
                                              style={is_scrollable.then_some("cursor: ns-resize;")}
                                              onwheel={years_wheel.clone()}
                                          >
                                              {time_display.years_str()}
                                          </span>
                                          <span class="time-separator" aria_hidden="true">{"y "}</span>
                                      </>
                                  }
                              } else {
                                  html! {}
                              }}
                              {if time_display.years > 0 || time_display.months > 0 {
                                  html! {
                                      <>
                                          <span
                                              class="time-digit"
                                              title={is_scrollable.then_some("Scroll to adjust months")}
                                              style={is_scrollable.then_some("cursor: ns-resize;")}
                                              onwheel={months_wheel.clone()}
                                          >
                                              {time_display.months_str()}
                                          </span>
                                          <span class="time-separator" aria_hidden="true">{"mo "}</span>
                                      </>
                                  }
                              } else {
                                  html! {}
                              }}
                              {if time_display.years > 0 || time_display.months > 0 || time_display.days > 0 {
                                  html! {
                                      <>
                                          <span
                                              class="time-digit"
                                              title={is_scrollable.then_some("Scroll to adjust days")}
                                              style={is_scrollable.then_some("cursor: ns-resize;")}
                                              onwheel={days_wheel.clone()}
                                          >
                                              {time_display.days_str()}
                                          </span>
                                          <span class="time-separator" aria_hidden="true">{"d "}</span>
                                      </>
                                  }
                              } else {
                                  html! {}
                              }}
                              <span
                                  class="time-digit"
                                  title={is_scrollable.then_some("Scroll to adjust hours")}
                                  style={is_scrollable.then_some("cursor: ns-resize;")}
                                  onwheel={hours_wheel.clone()}
                              >
                                  {format!("{:02}", time_display.hours)}
                              </span>
                              <span class="time-separator" aria_hidden="true">{":"}</span>
                              <span
                                  class="time-digit"
                                  title={is_scrollable.then_some("Scroll to adjust minutes")}
                                  style={is_scrollable.then_some("cursor: ns-resize;")}
                                  onwheel={minutes_wheel.clone()}
                              >
                                  {format!("{:02}", time_display.minutes)}
                              </span>
                              {if *show_seconds {
                                  html! {
                                      <>
                                          <span class="time-separator" aria_hidden="true">{":"}</span>
                                          <span
                                              class="time-digit"
                                              title={is_scrollable.then_some("Scroll to adjust seconds")}
                                              style={is_scrollable.then_some("cursor: ns-resize;")}
                                              onwheel={seconds_wheel.clone()}
                                          >
                                              {format!("{:02}", time_display.seconds)}
                                          </span>
                                      </>
                                  }
                              } else {
                                  html! {}
                              }}
                          </>
                      }
                  } else {
                      // Show standard HH:MM:SS format
                      html! {
                          <>
                              <span
                                  class="time-digit"
                                  title={is_scrollable.then_some("Scroll to adjust hours")}
                                  style={is_scrollable.then_some("cursor: ns-resize;")}
                                  onwheel={hours_wheel}
                              >
                                  {format!("{:02}", time_display.hours)}
                              </span>
                              <span class="time-separator" aria_hidden="true">{":"}</span>
                              <span
                                  class="time-digit"
                                  title={is_scrollable.then_some("Scroll to adjust minutes")}
                                  style={is_scrollable.then_some("cursor: ns-resize;")}
                                  onwheel={minutes_wheel}
                              >
                                  {format!("{:02}", time_display.minutes)}
                              </span>
                              {if *show_seconds {
                                  html! {
                                      <>
                                          <span class="time-separator" aria_hidden="true">{":"}</span>
                                          <span
                                              class="time-digit"
                                              title={is_scrollable.then_some("Scroll to adjust seconds")}
                                              style={is_scrollable.then_some("cursor: ns-resize;")}
                                              onwheel={seconds_wheel}
                                          >
                                              {format!("{:02}", time_display.seconds)}
                                          </span>
                                          <span class="time-millis" aria_hidden="true">{time_display.millis_str()}</span>
                                      </>
                                  }
                              } else {
                                  html! {}
                              }}
                          </>
                      }
                  }
              }
          </div>
      </div>
  }
}

/// Props for the mode switcher
#[derive(Properties, PartialEq)]
pub struct ModeSwitcherProps {
  pub active_mode: TimerMode,
  pub on_switch: Callback<TimerMode>,
}

/// Horizontal mode switcher with tab-like buttons
#[function_component(ModeSwitcher)]
pub fn mode_switcher(props: &ModeSwitcherProps) -> Html {
  let ModeSwitcherProps { active_mode, on_switch } = props;

  html! {
      <nav class="mode-switcher" role="tablist" aria_label="Timer mode selection">
          {TimerMode::Clock.render_tab(*active_mode, on_switch)}
          {TimerMode::Countdown.render_tab(*active_mode, on_switch)}
          {TimerMode::CountdownTo.render_tab(*active_mode, on_switch)}
      </nav>
  }
}

/// Trait for rendering a timer mode as a tab button.
pub trait ModeTab {
  fn render_tab(&self, active_mode: TimerMode, on_switch: &Callback<TimerMode>) -> Html;
}

impl ModeTab for TimerMode {
  fn render_tab(&self, active_mode: TimerMode, on_switch: &Callback<TimerMode>) -> Html {
    let is_active = *self == active_mode;
    let label = self.label();
    let mode = *self;
    let on_switch = on_switch.clone();

    let onclick = Callback::from(move |_: MouseEvent| {
      on_switch.emit(mode);
    });

    html! {
        <button
            role="tab"
            class={classes!("mode-tab", is_active.then_some("active"))}
            aria_selected={is_active.to_string()}
            aria_controls={format!("mode-{}", self.label().to_lowercase().replace(' ', "-"))}
            onclick={onclick}
        >
            {label}
        </button>
    }
  }
}

/// Props for theme toggle button
#[derive(Properties, PartialEq)]
pub struct ThemeToggleProps {
  pub theme: Theme,
  pub on_toggle: Callback<()>,
}

/// Dark/light theme toggle button
#[function_component(ThemeToggle)]
pub fn theme_toggle(props: &ThemeToggleProps) -> Html {
  let icon = props.theme.icon();

  let aria_label = match props.theme {
    Theme::Dark => "Switch to light theme",
    Theme::Light => "Switch to dark theme",
  };

  let on_toggle = props.on_toggle.clone();
  let onclick = Callback::from(move |_: MouseEvent| {
    on_toggle.emit(());
  });

  html! {
      <button
          class="theme-toggle"
          aria_label={aria_label}
          onclick={onclick}
      >
          {icon}
      </button>
  }
}

/// Props for countdown controls
#[derive(Properties, PartialEq)]
pub struct CountdownControlsProps {
  #[allow(dead_code)]
  pub on_switch: Callback<TimerMode>,
  pub on_tick: Callback<Msg>,
  pub state: TimerState,
}

/// Mode-specific controls (start/pause/reset for countdown, info for clock)
#[function_component(CountdownControls)]
pub fn countdown_controls(props: &CountdownControlsProps) -> Html {
  let CountdownControlsProps { state, on_tick, .. } = props;

  let timer_controls = match state.mode {
    TimerMode::Clock => {
      html! {
          <div class="mode-info">
              <span class="info-label">{"Current Time"}</span>
          </div>
      }
    }
    TimerMode::Countdown => {
      let is_complete = state.is_counting_up || state.elapsed_ms == 0;
      let status_text = if is_complete { "Elapsed Time" } else { "Time Remaining" };

      html! {
          <div class="countdown-controls">
              <div class="countdown-status">
                  {status_text}
              </div>
          </div>
      }
    }
    TimerMode::CountdownTo => {
      let target_str = if let Some(target) = state.target_timestamp_ms {
        let date = js_sys::Date::new(&wasm_bindgen::JsValue::from_f64(target as f64));
        format!(
          "{:04}-{:02}-{:02}T{:02}:{:02}",
          date.get_full_year(),
          date.get_month() + 1,
          date.get_date(),
          date.get_hours(),
          date.get_minutes()
        )
      } else {
        // Default to 1 hour from now
        let future = js_sys::Date::now() + 3600000.0;
        let date = js_sys::Date::new(&wasm_bindgen::JsValue::from_f64(future));
        format!(
          "{:04}-{:02}-{:02}T{:02}:{:02}",
          date.get_full_year(),
          date.get_month() + 1,
          date.get_date(),
          date.get_hours(),
          date.get_minutes()
        )
      };

      let target_change = {
        let on_tick = on_tick.clone();
        Callback::from(move |e: Event| {
          if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
            let value = input.value();
            // Parse datetime-local format: YYYY-MM-DDTHH:MM
            let date = js_sys::Date::new(&wasm_bindgen::JsValue::from_str(&value));
            let timestamp = date.value_of();
            if !timestamp.is_nan() {
              on_tick.emit(Msg::SetTargetTimestamp(timestamp as u64));
            }
          }
        })
      };

      html! {
          <div class="countdown-controls">
              <div class="countdown-status">
                  {"Countdown to:"}
              </div>

              <div class="countdown-config">
                  <label class="config-label" for="target-input">{"Target date & time:"}</label>
                  <input
                      id="target-input"
                      type="datetime-local"
                      class="datetime-input"
                      value={target_str}
                      onchange={target_change}
                  />
              </div>
          </div>
      }
    }
  };

  html! {
      <div class="timer-controls">
          {timer_controls}
      </div>
  }
}

/// Props for keyboard shortcuts info component
#[derive(Properties, PartialEq)]
pub struct KeyboardShortcutsProps {
  pub mode: TimerMode,
}

/// Component that displays keyboard shortcuts information
#[function_component(KeyboardShortcuts)]
pub fn keyboard_shortcuts(props: &KeyboardShortcutsProps) -> Html {
  let KeyboardShortcutsProps { mode } = props;

  let shortcuts = match mode {
    TimerMode::Clock => {
      html! {
        <div class="keyboard-shortcuts">
          <span class="shortcut-hint">{"↔ Arrow keys to switch modes"}</span>
        </div>
      }
    }
    TimerMode::Countdown => {
      html! {
        <div class="keyboard-shortcuts">
          <span class="shortcut-hint">{"Scroll to adjust"}</span>
          <span class="shortcut-separator">{" • "}</span>
          <span class="shortcut-hint"><kbd>{"Space"}</kbd>{" to start/stop"}</span>
          <span class="shortcut-separator">{" • "}</span>
          <span class="shortcut-hint"><kbd>{"R"}</kbd>{" to reset"}</span>
          <span class="shortcut-separator">{" • "}</span>
          <span class="shortcut-hint">{"↔ Arrow keys to switch modes"}</span>
        </div>
      }
    }
    TimerMode::CountdownTo => {
      html! {
        <div class="keyboard-shortcuts">
          <span class="shortcut-hint">{"Scroll to adjust target"}</span>
          <span class="shortcut-separator">{" • "}</span>
          <span class="shortcut-hint">{"↔ Arrow keys to switch modes"}</span>
        </div>
      }
    }
  };

  shortcuts
}
