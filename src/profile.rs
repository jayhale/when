use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Serialize, Deserialize, Clone)]
pub struct Profile {
    #[serde(skip)]
    path: PathBuf,

    pub version: ProfileVersion,
    pub calendars: HashMap<String, Calendar>,
}

#[derive(Serialize_repr, Deserialize_repr, Clone)]
#[repr(u8)]
pub enum ProfileVersion {
    V1 = 1,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Calendar {
    pub enabled: bool,
    pub connection: Connection,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum Connection {
    #[serde(rename = "ics")]
    Ics(IcsConnection),
}

#[derive(Serialize, Deserialize, Clone)]
pub struct IcsConnection {
    url: String,
}

impl Profile {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Profile {
            path: path.as_ref().to_path_buf(),
            version: ProfileVersion::V1,
            calendars: HashMap::new(),
        }
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Self {
        // TODO: Handle errors

        let p = path.as_ref();

        if !p.exists() {
            let profile = Profile::new(p);
            let contents = toml::to_string(&profile).unwrap();
            fs::write(p, contents).unwrap();
            return profile;
        }

        let contents = fs::read_to_string(p).unwrap();
        let mut profile: Profile = toml::from_str(&contents).unwrap();
        profile.path = p.to_path_buf();
        return profile;
    }

    pub fn save(&self) {
        // TODO: Handle errors
        let contents = toml::to_string(self).unwrap();
        fs::write(&self.path, contents).unwrap();
    }

    pub fn add_ics_calendar(&mut self, name: String, url: String) {
        self.calendars.insert(
            name,
            Calendar {
                enabled: true,
                connection: Connection::Ics(IcsConnection { url }),
            },
        );
    }
}
