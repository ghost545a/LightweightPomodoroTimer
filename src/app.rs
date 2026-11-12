use std::fmt;
use std::time::{Duration, Instant};
use slint::{SharedString, Timer};
use crate::MainWindow;

pub const POMODORO_DURATION_MS: i32 = 25 * 60 * 1000;
pub const SHORT_BREAK_DURATION_MS: i32 = 5 * 60 * 1000;
pub const LONG_BREAK_DURATION_MS: i32 = 15 * 60 * 1000;
pub const CYCLES_BEFORE_LONG_BREAK: i32 = 4;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PomodoroState {
    Work,
    ShortBreak,
    LongBreak,
}

impl PomodoroState {
    pub fn get_duration_ms(self) -> i32 {
        match self {
            PomodoroState::Work => POMODORO_DURATION_MS,
            PomodoroState::ShortBreak => SHORT_BREAK_DURATION_MS,
            PomodoroState::LongBreak => LONG_BREAK_DURATION_MS,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            PomodoroState::Work => "Work",
            PomodoroState::ShortBreak => "Short Break",
            PomodoroState::LongBreak => "Long Break",
        }
    }
}

impl fmt::Display for PomodoroState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

pub struct AppState {
    state: PomodoroState,
    pomodoro_cycles: i32,
    remaining_ms: i32,
    last_tick_time: Option<Instant>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            state: PomodoroState::Work,
            pomodoro_cycles: 0,
            remaining_ms: POMODORO_DURATION_MS,
            last_tick_time: None,
        }
    }

    pub fn remaining_seconds(&self) -> i32 {
        (self.remaining_ms.max(0) + 999) / 1000
    }

    pub fn state_label(&self) -> SharedString {
        let cycle_in_set = match self.state {
            PomodoroState::Work => (self.pomodoro_cycles % CYCLES_BEFORE_LONG_BREAK) + 1,
            PomodoroState::ShortBreak => self.pomodoro_cycles % CYCLES_BEFORE_LONG_BREAK,
            PomodoroState::LongBreak => CYCLES_BEFORE_LONG_BREAK,
        };
        format!("{} ({}/{})", self.state.as_str(), cycle_in_set, CYCLES_BEFORE_LONG_BREAK).into()
    }

    pub fn start_timer(&mut self, ui: &MainWindow) {
        ui.set_timer_active(true);
        self.last_tick_time = Some(Instant::now());
    }

    pub fn pause_timer(&mut self, ui: &MainWindow, timer: &Timer) {
        timer.stop();
        ui.set_timer_active(false);
        self.last_tick_time = None;
    }

    pub fn reset_timer(&mut self, ui: &MainWindow, timer: &Timer) {
        self.pause_timer(ui, timer);
        self.remaining_ms = self.state.get_duration_ms();
        self.update_ui(ui);
    }

    pub fn skip_state(&mut self, ui: &MainWindow, timer: &Timer) {
        self.pause_timer(ui, timer);
        self.advance_state();
        ui.set_current_state_text(self.state_label());
        self.update_ui(ui);
    }

    pub fn tick(&mut self, ui: &MainWindow, timer: &Timer) {
        let now = Instant::now();
        if self.advance_time(now) {
            self.skip_state(ui, timer);
        } else {
            self.update_ui(ui);
        }
    }

    /// Advances internal timer state to `now`. Returns `true` if the current session finished.
    fn advance_time(&mut self, now: Instant) -> bool {
        let elapsed_time = if let Some(last_tick) = self.last_tick_time {
            now.saturating_duration_since(last_tick)
        } else {
            Duration::ZERO
        };

        self.remaining_ms -= elapsed_time.as_millis() as i32;
        self.last_tick_time = Some(now);

        if self.remaining_ms <= 0 {
            self.remaining_ms = 0;
            true
        } else {
            false
        }
    }

    fn advance_state(&mut self) {
        if self.state == PomodoroState::Work {
            self.pomodoro_cycles += 1;
            if self.pomodoro_cycles % CYCLES_BEFORE_LONG_BREAK == 0 {
                self.state = PomodoroState::LongBreak;
            } else {
                self.state = PomodoroState::ShortBreak;
            }
        } else {
            self.state = PomodoroState::Work;
        }
        self.remaining_ms = self.state.get_duration_ms();
        self.last_tick_time = None;
    }

    fn update_ui(&self, ui: &MainWindow) {
        let remaining_secs = self.remaining_seconds();
        if ui.get_remaining_seconds() != remaining_secs {
            ui.set_remaining_seconds(remaining_secs);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ceiling_division_rounds_up_partial_seconds() {
        let mut app = AppState::new();
        assert_eq!(app.remaining_seconds(), 25 * 60);

        // 100ms elapsed -> still displays 25:00 (1500s) instead of jumping to 24:59 (1499s)
        app.remaining_ms = POMODORO_DURATION_MS - 100;
        assert_eq!(app.remaining_seconds(), 25 * 60);

        // Exact second boundary
        app.remaining_ms = (25 * 60 - 1) * 1000;
        assert_eq!(app.remaining_seconds(), 25 * 60 - 1);

        // Last millisecond still shows 00:01 instead of 00:00
        app.remaining_ms = 1;
        assert_eq!(app.remaining_seconds(), 1);

        // Zero shows 00:00
        app.remaining_ms = 0;
        assert_eq!(app.remaining_seconds(), 0);
    }

    #[test]
    fn pause_clears_last_tick_time_preventing_time_jump() {
        let mut app = AppState::new();
        let start = Instant::now();
        app.last_tick_time = Some(start);

        // Tick after 2 seconds of active work
        let t1 = start + Duration::from_secs(2);
        assert!(!app.advance_time(t1));
        assert_eq!(app.remaining_ms, POMODORO_DURATION_MS - 2_000);

        // Simulate pause (clears last_tick_time)
        app.last_tick_time = None;

        // Simulate resume 10 seconds later
        let t_resume = t1 + Duration::from_secs(10);
        app.last_tick_time = Some(t_resume);

        // First tick 100ms after resume should only subtract 100ms, NOT 10.1s!
        let t2 = t_resume + Duration::from_millis(100);
        assert!(!app.advance_time(t2));
        assert_eq!(app.remaining_ms, POMODORO_DURATION_MS - 2_100);
    }

    #[test]
    fn pomodoro_cycle_transitions_and_labels() {
        let mut app = AppState::new();
        assert_eq!(app.state, PomodoroState::Work);
        assert_eq!(app.state_label().as_str(), "Work (1/4)");

        // Cycle 1: Work -> Short Break
        app.advance_state();
        assert_eq!(app.state, PomodoroState::ShortBreak);
        assert_eq!(app.state_label().as_str(), "Short Break (1/4)");
        assert_eq!(app.remaining_ms, SHORT_BREAK_DURATION_MS);

        // Cycle 2: Work -> Short Break
        app.advance_state();
        assert_eq!(app.state, PomodoroState::Work);
        assert_eq!(app.state_label().as_str(), "Work (2/4)");
        app.advance_state();
        assert_eq!(app.state, PomodoroState::ShortBreak);
        assert_eq!(app.state_label().as_str(), "Short Break (2/4)");

        // Cycle 3: Work -> Short Break
        app.advance_state();
        assert_eq!(app.state_label().as_str(), "Work (3/4)");
        app.advance_state();
        assert_eq!(app.state_label().as_str(), "Short Break (3/4)");

        // Cycle 4: Work -> Long Break
        app.advance_state();
        assert_eq!(app.state, PomodoroState::Work);
        assert_eq!(app.state_label().as_str(), "Work (4/4)");
        app.advance_state();
        assert_eq!(app.state, PomodoroState::LongBreak);
        assert_eq!(app.state_label().as_str(), "Long Break (4/4)");
        assert_eq!(app.remaining_ms, LONG_BREAK_DURATION_MS);

        // Next cycle wraps back to Work (1/4)
        app.advance_state();
        assert_eq!(app.state, PomodoroState::Work);
        assert_eq!(app.state_label().as_str(), "Work (1/4)");
    }

    #[test]
    fn timer_completes_when_remaining_reaches_zero() {
        let mut app = AppState::new();
        let start = Instant::now();
        app.last_tick_time = Some(start);

        let finish = start + Duration::from_millis(POMODORO_DURATION_MS as u64 + 50);
        assert!(app.advance_time(finish));
        assert_eq!(app.remaining_ms, 0);
        assert_eq!(app.remaining_seconds(), 0);
    }
}
