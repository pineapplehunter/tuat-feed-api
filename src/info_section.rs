use std::time::Instant;
use tuat_feed_parser::Info;

/// InfoSection.
/// This struct holds the information and when it was last checked.
pub struct InfoSection {
    /// the time the information was last checked.
    pub last_checked: Instant,
    /// actual information.
    pub info: Vec<Info>,
}

impl InfoSection {
    /// creates a new InfoSection from a `Vec<Info>`.
    pub fn new(info: Vec<Info>) -> Self {
        InfoSection {
            info,
            last_checked: Instant::now(),
        }
    }

    /// set a new state.
    /// (used for updating the information)
    pub fn set(&mut self, info: Vec<Info>) {
        self.info = info;
        self.last_checked = Instant::now();
    }
}
