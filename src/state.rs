use anyhow::Result;
use log::info;
use std::time::Instant;
use tokio::sync::RwLock;
use tokio::try_join;

use tuat_feed_parser::{get_academic_feed, get_campus_feed, Info};

use crate::info_section::InfoSection;
use crate::INTERVAL;

/// State of the server.
/// contains data for both academic and campus information.
pub struct State {
    /// academic information.
    academic: RwLock<InfoSection>,
    /// campus information.
    campus: RwLock<InfoSection>,
}

impl State {
    /// initializes the state.
    /// fetches the data from tuat feed and stores it.
    pub async fn init() -> Result<Self> {
        info!("initializing state");
        let (academic, campus) = try_join!(get_academic_feed(), get_campus_feed())?;
        // let academic = get_academic_feed().await.context("academic")?;
        // let campus = get_campus_feed().await.context("campus")?;

        Ok(Self {
            academic: RwLock::new(InfoSection::new(academic)),
            campus: RwLock::new(InfoSection::new(campus)),
        })
    }

    /// updates and gets academic info
    pub async fn get_academic(&self) -> Result<Vec<Info>> {
        let update_academic = Instant::now() > self.academic.read().await.last_checked + INTERVAL;

        if update_academic {
            self.academic.write().await.set(get_academic_feed().await?);
        }

        let info = self.academic.read().await.info.clone();

        Ok(info)
    }

    /// updates and gets capmus info
    pub async fn get_campus(&self) -> Result<Vec<Info>> {
        let update_campus = Instant::now() > self.campus.read().await.last_checked + INTERVAL;

        if update_campus {
            self.campus.write().await.set(get_academic_feed().await?);
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
}
