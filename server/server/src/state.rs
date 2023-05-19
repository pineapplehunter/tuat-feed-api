use crate::info_bundle::InfoBundle;
use std::{fmt, sync::Arc, time::Instant};
use tokio::sync::{Mutex, RwLock};
use tracing::{info, info_span, warn, Instrument};
use tuat_feed_scraper::{Feed, FeedCategory, Gakubu};

/// State of the server.
/// contains data for both academic and campus information.
#[derive(Debug)]
pub struct ServerState {
    /// state for Technology Academic
    pub technology_academic: FeedState,
    /// state for Technology Campus
    pub technology_campus: FeedState,
    /// state for Agriculture Academic
    pub agriculture_academic: FeedState,
    /// state for Agriculture Campus
    pub agriculture_campus: FeedState,
}

/// Atomic shared state
pub type SharedState = Arc<ServerState>;

/// State for each feed
pub struct FeedState {
    feed: Mutex<Feed>,
    /// information from feed. rw lock for fast access.
    pub information: RwLock<InfoBundle>,
}

impl fmt::Debug for FeedState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FeedState").finish_non_exhaustive()
    }
}

impl FeedState {
    fn new(gakubu: Gakubu, category: FeedCategory) -> Self {
        Self {
            feed: Mutex::new(Feed::new(gakubu, category)),
            information: RwLock::new(InfoBundle::new(Vec::new(), Instant::now())),
        }
    }

    async fn update(&self) {
        let mut feed = self.feed.lock().await;
        let new_info = feed.fetch().await;
        if new_info.is_err() {
            warn!("encounted error in {:?}", feed);
            return;
        }
        let new_info = new_info.unwrap();
        let mut information = self.information.write().await;
        information.post = new_info;
    }
}

impl ServerState {
    /// initializes the state.
    /// fetches the data from tuat feed and stores it.
    pub fn init() -> Self {
        info!("initializing state");

        Self {
            technology_academic: FeedState::new(Gakubu::Technology, FeedCategory::Academic),
            technology_campus: FeedState::new(Gakubu::Technology, FeedCategory::Campus),
            agriculture_academic: FeedState::new(Gakubu::Agriculture, FeedCategory::Academic),
            agriculture_campus: FeedState::new(Gakubu::Agriculture, FeedCategory::Campus),
        }
    }

    /// update all feeds
    #[tracing::instrument]
    pub async fn update(&self) {
        info!("updating state");
        self.technology_academic
            .update()
            .instrument(info_span!("update technology_academic"))
            .await;
        self.technology_campus
            .update()
            .instrument(info_span!("update technology_campus"))
            .await;
        self.agriculture_academic
            .update()
            .instrument(info_span!("update agriculture_academic"))
            .await;
        self.agriculture_campus
            .update()
            .instrument(info_span!("update agriculture_campus"))
            .await;
        info!("state updated");
    }
}

#[cfg(test)]
mod tests {
    use super::ServerState;
    #[test]
    fn state_init() {
        ServerState::init();
    }
}
