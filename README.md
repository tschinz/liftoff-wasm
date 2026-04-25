# Liftoff

A polished, responsive countdown timer and clock web application built with Rust and WebAssembly. Features multiple timer modes, smooth animations, and dark/light themes with visual alerts when countdowns complete.

⚠️ Disclaimer: GenAI-Generated Code

A significant portion of the code in this repository was generated using Generative AI tools.
The project was conducted as an experiment to evaluate the capabilities of local, small, open-weight models in practical software development scenarios.

The following models were used during this process:

- Qwen 3.6
- Qwen3-Coder
- GPT-OSS
- Granite 4 (Micro and 3B variants)
- DeepSeek-R1
- GLM-4.6

The generated code has been reviewed and adapted where necessary, but may still contain inconsistencies or non-optimal patterns. It should therefore be considered experimental and not production-ready without further validation.

## Features

- **Three timer modes**
  - Current clock
  - Countdown timer (counts down, then up after hitting zero)
  - Countdown to a specific date/time
- **Dark and light themes** with smooth transitions
- **Responsive layout** — mobile-first, scales gracefully to desktop
- **Accessible** — semantic HTML, keyboard support, ARIA labels
- **Smooth animations** — CSS transitions, pulsing separators

## Tech Stack

| Layer        | Technology                              |
|-------------|-----------------------------------------|
| Language    | Rust (stable)                           |
| Framework   | Yew 0.21 (WebAssembly)                  |
| Builder     | Trunk                                   |
| Styling     | Custom CSS with CSS custom properties   |
| Timer       | `gloo_timers`                           |

## Prerequisites

- [Rust toolchain](https://rustup.rs/) (stable)
- [Trunk](https://trunkrs.dev/) (WebAssembly bundler)
- [Just](https://just.systems/) (optional, for convenience)
- A modern browser with WebAssembly support

## Quick Start

```bash
# 1. Install Trunk
cargo install trunk --locked

# 2. Enter the project directory
cd liftoff

# 3. Run the dev server
trunk serve --port 8080 --open
```

The app will open at `http://localhost:8080`.

### Using Just (Optional)

If you have Just installed:

```bash
# Install dependencies
just install-deps

# Run dev server
just run

# Run tests
just test

# Format and lint
just fmt
just clippy
```

## Available Commands

| Command           | Description                                   |
|------------------|-----------------------------------------------|
| `just`            | Show help (default)                           |
| `just install-deps` | Install Trunk and fetch dependencies       |
| `just run`        | Start dev server (opens browser)              |
| `just run-no-open` | Start dev server (manual browser)            |
| `just build`      | Build release bundle to `dist/`               |
| `just test`       | Run all tests (unit + integration)            |
| `just check`      | Check compilation (no build artifacts)        |
| `just fmt`        | Format all code                               |
| `just clippy`     | Run Clippy lints (fail on warnings)           |
| `just clean`      | Remove build artifacts                        |
| `just update-deps` | Update Cargo dependencies                    |

## Project Structure

```
liftoff/
├── Cargo.toml          # Dependencies
├── Cargo.lock
├── Justfile            # Build automation
├── index.html          # HTML entry point
├── styles.css          # All styles (dark/light themes)
├── tests/
│   └── integration.rs  # Integration tests
└── src/
    ├── lib.rs          # WASM entry point
    ├── app.rs          # Root App component + timer loop
    ├── model.rs        # Data types (TimerMode, Theme, etc.)
    ├── update.rs       # State update logic
    ├── view.rs         # UI components (display, switcher, controls)
    ├── components/     # Re-exports from view.rs
    └── timer/          # Timer service + per-mode logic
```

## Testing

```bash
# Run all tests
just test

# Run tests in release mode
just test-optimized
```

## Key Features

- **Three Timer Modes**: Clock, Countdown Timer, and Countdown To specific date/time
- **Keyboard Controls**: Spacebar to start/pause, R to reset, arrow keys to switch modes
- **Visual Alerts**: Red color when countdown ends (T+ state)
- **Scrollable Time Units**: Adjust hours, minutes, seconds, and large units (days, months, years) by scrolling
- **Dark/Light Themes**: Smooth theme transitions with proper background colors
- **T-minus/T-plus Prefixes**: Clear indication of countdown state
- **Zero Value Display**: Shows all time units including zeros for clarity
- **Responsive Design**: Works on desktop and mobile devices

## Design Decisions

- **Rust for all logic** — no JavaScript dependencies
- **gloo_timers for intervals** — the standard Rust→JS timer bridge
- **Yew with CSR** — component-based UI, compiled to WASM
- **CSS custom properties** — clean theme switching without JS
- **Minimal dependencies** — only what's needed for the timer and UI
- **Function components** — using Yew's function component API for simplicity

## Troubleshooting

### Build fails with "main function not found"
This is a library crate. Make sure you're using `trunk serve` or `trunk build`, not `cargo run`.

### WASM doesn't load in browser
Check browser console for errors. Ensure you're using a modern browser with WebAssembly support.

### Tests fail
Run `cargo test --lib` for library tests and `cargo test --test integration` for integration tests.

## License

MIT
