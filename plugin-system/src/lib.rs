use directories::ProjectDirs;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Extension {
    pub name: String,
    /// A reverse DNS id like com.projectflara.Dresslive
    pub id: String,
    /// Fiana Fortressia <fiana@projectflara.com>
    pub authors: Vec<String>,
    /// An SPDX lice
    pub license: String,
    ///
    pub website: String,
    pub issues: String,
    pub screenshots: Vec<String>,
}

pub enum StartupError {

}

pub fn startup() -> Result<Vec<Extension>, StartupError> {
    let mut app_id: Vec<&str> = flara_utils::APP_ID.split(".").collect();
    let dirs = ProjectDirs::from(&app_id.remove(0), &app_id.remove(0), &app_id.join("."));

    todo!()
}
