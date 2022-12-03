use directories::ProjectDirs;
use serde_derive::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::Read,
    path::PathBuf,
};

#[derive(Serialize, Deserialize)]
pub struct Plugin {
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
    DecodeError(PathBuf, toml::de::Error),
    IoError(std::io::Error),
}

pub struct StartupErrors {
    ///
    pub errors: Vec<StartupError>,
    /// Plugins that we have succesfully managed to load
    pub okay_plugins: Vec<Plugin>,
}

pub fn startup() -> Result<Vec<Plugin>, StartupErrors> {
    let mut app_id: Vec<&str> = flara_utils::APP_ID.split(".").collect();
    let dirs = ProjectDirs::from(
        &app_id.remove(0),
        &app_id.remove(0),
        &app_id.join("."),
    )
    .unwrap();
    let mut errors = Vec::new();
    let mut plugins: Vec<Plugin> = Vec::new();
    for dir in fs::read_dir(dirs.cache_dir()).unwrap() {
        match dir {
            Ok(dir) => {
                let manifest_path = dir.path().join("ProjectFlara.toml");

                if manifest_path.exists() {
                    match File::open(&manifest_path) {
                        Ok(mut file) => {
                            let mut string = String::new();
                            match file.read_to_string(&mut string) {
                                Ok(_) => {
                                    match toml::de::from_str(&string) {
                                        Ok(plugin) => plugins.push(plugin),
                                        Err(err) => errors.push(
                                            StartupError::DecodeError(
                                                manifest_path,
                                                err,
                                            ),
                                        ),
                                    };
                                }
                                Err(err) => {
                                    errors.push(StartupError::IoError(err))
                                }
                            }
                        }
                        Err(err) => errors.push(StartupError::IoError(err)),
                    }
                }
            }
            Err(err) => errors.push(StartupError::IoError(err)),
        }
    }
    if errors.len() == 0 {
        Ok(plugins)
    } else {
        Err(StartupErrors {
            errors,
            okay_plugins: plugins,
        })
    }
}
