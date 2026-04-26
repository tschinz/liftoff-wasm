use wasm_bindgen::JsCast;
use web_sys::{console, window, HtmlElement};
use yew::prelude::*;

use crate::model::{Theme, TimerMode, TimerState};
use crate::update::{format_display, update, Msg};
use crate::view::{CountdownControls, KeyboardShortcuts, ModeSwitcher, ThemeToggle, TimeDisplayComponent};

/// The root App component that composes all sub-components
/// and manages the timer loop.
#[function_component(App)]
pub fn app() -> Html {
  let state = use_state(TimerState::default);
  let render_tick = use_state(|| 0u32);

  // Apply theme class to body element
  {
    let theme = state.theme;
    use_effect_with(theme, move |theme| {
      if let Some(window) = window() {
        if let Some(document) = window.document() {
          if let Some(body) = document.body() {
            let body_element: HtmlElement = body;
            // Remove both theme classes first
            let _ = body_element.class_list().remove_1("theme-dark");
            let _ = body_element.class_list().remove_1("theme-light");
            // Add the current theme class
            let _ = body_element.class_list().add_1(theme.css_class());
          }
        }
      }
      || ()
    });
  }

  // Timer tick: update the display every ~16ms (60fps)
  // Recreated whenever state changes to ensure it sees latest values
  {
    let state_clone = state.clone();
    let render_tick = render_tick.clone();
    use_effect_with((*state).clone(), move |current_state| {
      let state = state_clone.clone();
      let render_tick = render_tick.clone();
      let mode = current_state.mode;
      let is_running = current_state.is_running;
      let is_counting_up = current_state.is_counting_up;

      let interval = gloo_timers::callback::Interval::new(16, move || {
        let current = (*state).clone();

        match mode {
          TimerMode::Clock => {
            // Clock mode - trigger render by updating render_tick
            render_tick.set((*render_tick).wrapping_add(1));
          }
          TimerMode::Countdown => {
            // Countdown mode - only tick if running
            if is_running {
              let mut new_state = current;

              if !is_counting_up {
                // Counting down
                if new_state.elapsed_ms > 16 {
                  new_state.elapsed_ms = new_state.elapsed_ms.saturating_sub(16);
                } else {
                  // Hit zero, switch to counting up
                  new_state.elapsed_ms = 0;
                  new_state.is_counting_up = true;
                }
              } else {
                // Counting up after hitting zero
                new_state.elapsed_ms = new_state.elapsed_ms.saturating_add(16);
              }

              state.set(new_state);
            }
          }
          TimerMode::CountdownTo => {
            // CountdownTo mode - always render to update countdown
            render_tick.set((*render_tick).wrapping_add(1));
          }
        }
      });

      move || drop(interval)
    });
  }

  // Auto-focus the app container for keyboard events
  {
    use_effect_with((), move |_| {
      if let Some(window) = window() {
        if let Some(document) = window.document() {
          if let Some(element) = document.query_selector(".app-container").ok().flatten() {
            let html_element: HtmlElement = element.unchecked_into();
            let _ = html_element.focus();
          }
        }
      }
      || ()
    });
  }

  // Keyboard handler
  let on_keydown = {
    let state = state.clone();
    Callback::from(move |e: KeyboardEvent| {
      let key = e.key();
      let current_state = (*state).clone();

      // Handle spacebar and 'r' in Countdown mode
      if current_state.mode == TimerMode::Countdown {
        if key == " " || key == "Spacebar" {
          e.prevent_default();
          let mut new_state = current_state.clone();
          new_state.is_running = !new_state.is_running;
          console::log_1(
            &format!(
              "Spacebar pressed: toggling is_running from {} to {}",
              current_state.is_running, new_state.is_running
            )
            .into(),
          );
          state.set(new_state);
        } else if key == "r" || key == "R" {
          e.prevent_default();
          let mut new_state = current_state.clone();
          new_state.elapsed_ms = new_state.countdown_duration_ms;
          new_state.is_counting_up = false;
          new_state.is_running = false;
          state.set(new_state);
        }
      }
      // Handle left/right arrows for mode switching
      if key == "ArrowLeft" || key == "ArrowRight" {
        e.prevent_default();

        let new_mode = if key == "ArrowRight" {
          match current_state.mode {
            TimerMode::Clock => TimerMode::Countdown,
            TimerMode::Countdown => TimerMode::CountdownTo,
            TimerMode::CountdownTo => TimerMode::Clock,
          }
        } else {
          match current_state.mode {
            TimerMode::Clock => TimerMode::CountdownTo,
            TimerMode::Countdown => TimerMode::Clock,
            TimerMode::CountdownTo => TimerMode::Countdown,
          }
        };

        let mut new_state = current_state.clone();
        update(&mut new_state, Msg::SwitchMode(new_mode));
        state.set(new_state);
      }
    })
  };

  // Callbacks for UI events
  let on_switch = {
    let state = state.clone();
    Callback::from(move |mode: TimerMode| {
      let mut new_state = (*state).clone();
      new_state.mode = mode;
      if mode == TimerMode::Countdown {
        // Initialize Timer mode with elapsed_ms set to duration
        new_state.elapsed_ms = new_state.countdown_duration_ms;
        new_state.is_counting_up = false;
        new_state.is_running = false;
      } else {
        new_state.is_counting_up = false;
        new_state.elapsed_ms = 0;
        new_state.is_running = false;
      }
      // Initialize target timestamp for CountdownTo mode if not set
      if mode == TimerMode::CountdownTo && new_state.target_timestamp_ms.is_none() {
        let future = js_sys::Date::now() + 3600000.0; // 1 hour from now
        new_state.target_timestamp_ms = Some(future as u64);
      }
      state.set(new_state);
    })
  };

  let on_toggle_theme = {
    let state = state.clone();
    Callback::from(move |_| {
      let mut new_state = (*state).clone();
      new_state.theme = match new_state.theme {
        Theme::Dark => Theme::Light,
        Theme::Light => Theme::Dark,
      };
      state.set(new_state);
    })
  };

  let on_toggle_seconds = {
    let state = state.clone();
    Callback::from(move |_| {
      let mut new_state = (*state).clone();
      new_state.show_seconds = !new_state.show_seconds;
      state.set(new_state);
    })
  };

  let on_tick = {
    let state = state.clone();
    Callback::from(move |msg: Msg| {
      let mut new_state = (*state).clone();
      update(&mut new_state, msg);
      state.set(new_state);
    })
  };

  // Wheel handler for adjusting countdown duration and target time
  let on_wheel = {
    let state = state.clone();
    Callback::from(move |(unit, delta): (String, f64)| {
      let mut new_state = (*state).clone();

      if new_state.mode == TimerMode::Countdown {
        // Countdown mode: adjust duration
        let current_duration_ms = new_state.countdown_duration_ms;
        let adjustment_ms = match unit.as_str() {
          "years" => (delta * 365.0 * 24.0 * 3600.0 * 1000.0) as i64,
          "months" => (delta * 30.0 * 24.0 * 3600.0 * 1000.0) as i64,
          "days" => (delta * 24.0 * 3600.0 * 1000.0) as i64,
          "hours" => (delta * 3600.0 * 1000.0) as i64,
          "minutes" => (delta * 60.0 * 1000.0) as i64,
          "seconds" => (delta * 1000.0) as i64,
          _ => 0,
        };

        let new_duration = (current_duration_ms as i64 + adjustment_ms).max(1000); // Min 1 second
        new_state.countdown_duration_ms = new_duration as u64;

        // Update elapsed_ms if not currently counting up
        if !new_state.is_counting_up {
          new_state.elapsed_ms = new_duration as u64;
        }

        state.set(new_state);
      } else if new_state.mode == TimerMode::CountdownTo {
        // CountdownTo mode: adjust target timestamp
        let adjustment_ms = match unit.as_str() {
          "years" => (delta * 365.0 * 24.0 * 3600.0 * 1000.0) as i64,
          "months" => (delta * 30.0 * 24.0 * 3600.0 * 1000.0) as i64,
          "days" => (delta * 24.0 * 3600.0 * 1000.0) as i64,
          "hours" => (delta * 3600.0 * 1000.0) as i64,
          "minutes" => (delta * 60.0 * 1000.0) as i64,
          "seconds" => (delta * 1000.0) as i64,
          _ => 0,
        };

        console::log_1(&format!("CountdownTo scroll: unit={}, delta={}, adjustment={}", unit, delta, adjustment_ms).into());

        // Get current target or initialize to 1 hour from now
        let current_target = new_state.target_timestamp_ms.unwrap_or_else(|| (js_sys::Date::now() + 3600000.0) as u64);

        let new_target = (current_target as i64 + adjustment_ms).max(js_sys::Date::now() as i64);
        new_state.target_timestamp_ms = Some(new_target as u64);

        console::log_1(&format!("New target: {}", new_target).into());

        state.set(new_state);
      }
    })
  };

  // Compute display values
  let time_display = format_display(&state);

  html! {
      <div class="app-container" tabindex="0" onkeydown={on_keydown}>
          <header class="app-header">
              <h1 class="app-title">
                  <a
                      href="https://synd.hevs.io/education/infotronics.html"
                      target="_blank"
                      rel="noopener noreferrer"
                      class="app-logo-link"
                      title="Visit Infotronics"
                  >
                      <img
                          src={if state.theme == Theme::Dark {
                              "img/infotronics-dark.svg"
                          } else {
                              "img/infotronics-light.svg"
                          }}
                          alt="Infotronics"
                          class="app-logo"
                      />
                  </a>
              </h1>
              <div style="display: flex; gap: 0.5rem;">
                  <button
                      class="theme-toggle"
                      aria_label={if state.show_seconds { "Hide seconds" } else { "Show seconds" }}
                      onclick={on_toggle_seconds}
                      title={if state.show_seconds { "Hide seconds" } else { "Show seconds" }}
                  >
                      {if state.show_seconds { ":SS" } else { ":--" }}
                  </button>
                  <ThemeToggle
                      theme={state.theme}
                      on_toggle={on_toggle_theme}
                  />
              </div>
          </header>

          <ModeSwitcher
              active_mode={state.mode}
              on_switch={on_switch.clone()}
          />

          <main class="timer-display">
              <TimeDisplayComponent
                  time_display={time_display.clone()}
                  mode={state.mode}
                  show_seconds={state.show_seconds}
                  is_counting_up={state.is_counting_up}
                  on_wheel={if state.mode == TimerMode::Countdown || state.mode == TimerMode::CountdownTo {
                      Some(on_wheel)
                  } else {
                      None
                  }}
              />
          </main>

          <CountdownControls
              state={(*state).clone()}
              on_switch={on_switch}
              on_tick={on_tick}
          />

          <KeyboardShortcuts
              mode={state.mode}
          />
      </div>
  }
}
