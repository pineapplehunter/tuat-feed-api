use anyhow::Result;
use log::info;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tokio::try_join;

use tuat_feed_parser::{get_academic_feed, get_campus_feed, Info};

use crate::info_section::InfoSection;

/// State of the server.
/// contains data for both academic and campus information.
pub struct State {
    /// academic information.
    academic: RwLock<InfoSection>,
    /// campus information.
    campus: RwLock<InfoSection>,
    /// interval to refresh
    interval: Duration,
}

impl State {
    /// initializes the state.
    /// fetches the data from tuat feed and stores it.
    pub async fn init(interval: Duration) -> Result<Self> {
        info!("initializing state");

        Ok(Self {
            academic: RwLock::new(InfoSection::new(Vec::new(), Instant::now() - interval)),
            campus: RwLock::new(InfoSection::new(Vec::new(), Instant::now() - interval)),
            interval,
        })
    }

    /// updates and gets academic info
    pub async fn get_academic(&self) -> Result<Vec<Info>> {
        let update_academic =
            Instant::now() > self.academic.read().await.last_checked + self.interval;

        if update_academic {
            let info = get_academic_feed().await?;
            self.academic.write().await.update(info);
        }

        let info = self.academic.read().await.info.clone();

        Ok(info)
    }

    /// updates and gets capmus info
    pub async fn get_campus(&self) -> Result<Vec<Info>> {
        let update_campus = Instant::now() > self.campus.read().await.last_checked + self.interval;

        if update_campus {
            let info = get_campus_feed().await?;
            self.campus.write().await.update(info);
        }

        let info = self.campus.read().await.info.clone();

        Ok(info)
    }

    /// gets all info
    pub async fn get_all(&self) -> Result<Vec<Info>> {
        let (mut academic, campus) = try_join!(self.get_academic(), self.get_campus())?;

        academic.extend(campus);

        Ok(academic)
    }

    /// sets all values in struct.
    /// this function was made for testing and should not be used in regular code.
    #[doc(hidden)]
    pub fn set_all(academic: InfoSection, campus: InfoSection, interval: Duration) -> State {
        Self {
            academic: RwLock::new(academic),
            campus: RwLock::new(campus),
            interval,
        }
    }
}
