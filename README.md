# Lightweight Pomodoro Timer

<img width="417" height="337" alt="Pomodoro Timer Screenshot" src="https://github.com/user-attachments/assets/c3f1cd2f-4114-43fc-8418-7cb15453c5a2" />

A fast, minimalist, and resource-efficient Pomodoro timer built with Rust and the [Slint](https://slint.dev/) UI toolkit, maintained by [@ghost545a](https://github.com/ghost545a).

## Features

- **Standard Pomodoro Cycles:** Work (`25:00`), Short Break (`05:00`), and Long Break (`15:00` after every 4th work session) with live cycle progress (`1/4` – `4/4`).
- **Accurate Timer Controls:** Start, pause, resume, reset, and skip sessions without timer drift or pause/resume jumps.
- **Ultra-Lightweight Footprint:** Uses Slint's CPU software renderer and compacted vector assets (`<2 KB` icon, `~2.7 MB` standalone executable) for minimal RAM and CPU usage.
- **Clean, Distraction-Free UI:** Crisp dark theme with zero-padded countdown display and tactile icon buttons.

## Downloading the Application

Download the latest pre-built Windows executable (`PomodoroTimer.exe` or `.zip`) directly from the **[Releases](https://github.com/ghost545a/LightweightPomodoroTimer/releases)** page.

> **Note for Windows users:**
> Windows SmartScreen might show a warning because the application is not digitally signed. Click **"More info"** -> **"Run anyway"** to start the application.

## Building from Source

 Make sure you have the [Rust toolchain](https://rustup.rs/) installed.

1. **Clone the repository:**

    ```bash
    git clone https://github.com/ghost545a/LightweightPomodoroTimer.git
    cd LightweightPomodoroTimer
    ```

2. **Install dependencies (Linux only):**

    ```bash
    sudo apt-get update && sudo apt-get install -y libfontconfig1-dev libgtk-3-dev
    ```

3. **Build and run the application:**

    For a quick debug build:

    ```bash
    cargo run
    ```

    For a fully optimized release build (recommended):

    ```bash
    cargo run --release
    ```

4. **Run unit tests:**

    ```bash
    cargo test
    ```

## Acknowledgments

- Original project created by **[reRumbl](https://github.com/reRumbl)** ([reRumbl/LightweightPomodoroTimer](https://github.com/reRumbl/LightweightPomodoroTimer)).
- Enhanced, debugged, and optimized in this fork by **[ghost545a](https://github.com/ghost545a)**.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
