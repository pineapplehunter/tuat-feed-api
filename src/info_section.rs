use std::time::Instant;
use tuat_feed_parser::Info;

/// InfoSection.
/// This struct holds the information and when it was last checked.
#[derive(Debug, Clone)]
pub struct InfoBundle {
    /// the time the information was last checked.
    pub last_checked: Instant,
    /// actual information.
    pub info: Vec<Info>,
}

impl InfoBundle {
    /// creates a new InfoSection from a `Vec<Info>`.
    pub fn new(info: Vec<Info>, last_checked: Instant) -> Self {
        InfoBundle { last_checked, info }
    }

    /// set a new state.
    /// (used for updating the information)
    pub fn update(&mut self, info: Vec<Info>) {
        self.info = info;
        self.last_checked = Instant::now();
    }
}

#[cfg(test)]
mod tests {
    use std::time::Instant;
    use tuat_feed_parser::Info;

    use super::InfoBundle;

    #[test]
    fn infosection_update() {
        let mut section = InfoBundle::new(vec![Info::new(0)], Instant::now());
        let InfoBundle { info, last_checked } = section.clone();
        section.update(vec![Info::new(1)]);
        assert!(section.last_checked > last_checked);
        assert_ne!(section.info[0], info[0]);
    }
}
