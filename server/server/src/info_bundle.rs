use std::{fmt, time::Instant};

use tuat_feed_scraper::post::Post;

/// This struct holds the information and when it was last checked.
#[derive(Clone)]
pub struct InfoBundle {
    /// the time the information was last checked.
    pub last_checked: Instant,
    /// actual information.
    pub post: Vec<Post>,
}

impl fmt::Debug for InfoBundle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("InfoBundle").finish_non_exhaustive()
    }
}

impl InfoBundle {
    /// creates a new `InfoBundle` from a `Vec<Info>`.
    pub fn new(post: Vec<Post>, last_checked: Instant) -> Self {
        InfoBundle { last_checked, post }
    }

    /// set a new state.
    /// (used for updating the information)
    pub fn update(&mut self, post: Vec<Post>) {
        self.post = post;
        self.last_checked = Instant::now();
    }
}

#[cfg(test)]
mod tests {
    use std::time::Instant;
    use tuat_feed_scraper::post::Post;

    use super::InfoBundle;

    #[test]
    fn test_info_bundle_update() {
        let mut section = InfoBundle::new(vec![Post::new(0)], Instant::now());
        let InfoBundle {
            post: info,
            last_checked,
        } = section.clone();
        section.update(vec![Post::new(1)]);
        assert!(section.last_checked > last_checked);
        assert_ne!(section.post[0], info[0]);
    }
}
