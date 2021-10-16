use crate::info_section::InfoBundle;
use color_eyre::eyre::Result;
use log::{info, warn};
use rocket::tokio::sync::{Mutex, RwLock};
use std::time::{Duration, Instant};
use tuat_feed_parser::{Feed, Info, ACADEMIC_FEED_URL, CAMPUS_FEED_URL};

/// State of the server.
/// contains data for both academic and campus information.
pub struct InformationState {
    academic_state: Mutex<Feed>,
    /// academic information.
    academic_info: RwLock<InfoBundle>,
    campus_state: Mutex<Feed>,
    /// campus information.
    campus_info: RwLock<InfoBundle>,
    /// interval to refresh
    interval: Duration,
}

impl InformationState {
    /// initializes the state.
    /// fetches the data from tuat feed and stores it.
    pub fn init(interval: Duration) -> Self {
        info!("initializing state");

        Self {
            academic_state: Mutex::new(Feed::new(ACADEMIC_FEED_URL)),
            academic_info: RwLock::new(InfoBundle::new(Vec::new(), Instant::now() - interval)),
            campus_state: Mutex::new(Feed::new(CAMPUS_FEED_URL)),
            campus_info: RwLock::new(InfoBundle::new(Vec::new(), Instant::now() - interval)),
            interval,
        }
    }

    /// updates and gets academic info
    pub async fn academic(&self) -> Result<Vec<Info>> {
        let update_academic =
            Instant::now() > self.academic_info.read().await.last_checked + self.interval;

        if update_academic {
            let mut lock = self.academic_info.write().await;
            let update_academic = Instant::now() > lock.last_checked + self.interval;
            if update_academic {
                match self.academic_state.lock().await.get().await {
                    Ok(info) => lock.update(info),
                    Err(e) => warn!("academic {:?}", e),
                }
            }
        }

        let info = self.academic_info.read().await.info.clone();

        Ok(info)
    }

    /// updates and gets capmus info
    pub async fn campus(&self) -> Result<Vec<Info>> {
        let update_campus =
            Instant::now() > self.campus_info.read().await.last_checked + self.interval;

        if update_campus {
            let mut lock = self.campus_info.write().await;
            let update_campus = Instant::now() > lock.last_checked + self.interval;
            if update_campus {
                match self.campus_state.lock().await.get().await {
                    Ok(info) => lock.update(info),
                    Err(e) => warn!("campus {:?}", e),
                }
            }
        }

        let info = self.campus_info.read().await.info.clone();

        Ok(info)
    }

    /// gets all info
    pub async fn all(&self) -> Result<Vec<Info>> {
        let mut academic = self.academic().await?;
        let campus = self.campus().await?;

        academic.extend(campus);

        Ok(academic)
    }

    /// sets all values in struct.
    /// this function was made for testing and should not be used in regular code.
    #[doc(hidden)]
    pub fn __set_all(
        academic: InfoBundle,
        campus: InfoBundle,
        interval: Duration,
    ) -> InformationState {
        Self {
            academic_state: Mutex::new(Feed::new(ACADEMIC_FEED_URL)),
            academic_info: RwLock::new(academic),
            campus_state: Mutex::new(Feed::new(CAMPUS_FEED_URL)),
            campus_info: RwLock::new(campus),
            interval,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::InformationState;
    use std::time::Duration;
    #[test]
    fn state_init() {
        InformationState::init(Duration::from_secs(1));
    }
}
