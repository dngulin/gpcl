use chrono::Timelike;
use std::io::Write;

#[derive(Clone)]
pub struct ClockTracker {
    hh: u32,
    mm: u32,
    buffer: Vec<u8>,
}

const PLACEHOLDER: &str = "??:??";

impl ClockTracker {
    pub fn new() -> Self {
        let mut tracker = Self {
            hh: 0,
            mm: 0,
            buffer: Vec::with_capacity(PLACEHOLDER.len()),
        };
        tracker.update();
        tracker
    }

    pub fn update(&mut self) -> bool {
        let time = chrono::Local::now().time();
        let hh = time.hour();
        let mm = time.minute();

        if self.hh != hh || self.mm != mm {
            self.hh = hh;
            self.mm = mm;
            return true;
        }

        false
    }

    pub fn time_str(&mut self) -> &str {
        self.buffer.clear();
        let _ = write!(self.buffer, "{:02}:{:02}", self.hh, self.mm);
        std::str::from_utf8(&self.buffer).unwrap_or(PLACEHOLDER)
    }
}
