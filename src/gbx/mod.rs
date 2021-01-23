pub mod parser;

use fmt::Display;
use serde::{Deserialize, Serialize};
use std::{convert::TryFrom, fmt};

#[derive(Serialize, Deserialize)]
pub struct JPEGData(pub Vec<u8>);

impl fmt::Display for JPEGData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "JPEG({}B)", self.0.len())
    }
}

impl fmt::Debug for JPEGData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GBX {
    pub origin: GBXOrigin,
    pub filesize: usize,
    header_start: usize,
    header_length: usize,
    thumbnail_start: usize,
    thumbnail_length: usize,
    pub thumbnail: JPEGData,
    pub header: GBXHeader,
    pub header_xml: String,
}

impl Display for GBX {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "GBX Info Dump (Size={}B)\nFrom file={}\nHeader Infos\n============\n{}",
            self.filesize, self.origin, self.header
        )
    }
}

//<header type="challenge" version="TMc.6" exever="2.11.16"><ident uid="jKazbEpo1vXYqJK25fZnoVnM6Z2" name="falch-2-dirty-jumps" author="lfalch"/><desc envir="Stadium" mood="Sunset" type="Race" nblaps="0" price="1282" /><times bronze="84000" silver="67000" gold="59000" authortime="55460" authorscore="55460"/><deps></deps></header>

#[derive(Debug, Serialize, Deserialize)]
pub enum GBXOrigin {
    File { path: String },
    Buffer,
    Unknown,
}

impl Default for GBXOrigin {
    fn default() -> Self {
        GBXOrigin::Unknown
    }
}

impl Display for GBXOrigin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GBXOrigin::File { path } => write!(f, "{}", path),
            GBXOrigin::Buffer => write!(f, "<buffer>"),
            GBXOrigin::Unknown => write!(f, "<unknown>"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct GBXHeader {
    maptype: MapType,
    mapversion: MapVersion,
    exever: String,
    uid: String,
    name: String,
    author: String,
    envir: Environment,
    mood: Mood,
    desctype: DescType,
    nblaps: u32,
    price: u32,
    times: Times,
    dependencies: Vec<Dependency>,
}

impl fmt::Display for GBXHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let dependency_files: Vec<&String> = self.dependencies.iter().map(|x| &x.file).collect();
        write!(f, "Map is {:?}/{:?} made in {:?}/{}\nUID: {}\nName: {}\nAuthor: {}\nSetting: {:?}/{:?}\nNumber of laps: {}\nPrice: {}\nTimes: {}\nDependencies[{}]: {:?}",
            self.maptype, self.desctype, self.mapversion, self.exever, self.uid, self.name, self.author, self.envir, self.mood, self.nblaps, self.price, self.times, self.dependencies.len(), dependency_files,
        )
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Times {
    bronze: u32,
    silver: u32,
    gold: u32,
    authortime: u32,
    authorscore: u32,
}

impl fmt::Display for Times {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Bronze: {}, Silver: {}, Gold: {}, Authortime: {}, Authorscore: {}",
            self.bronze, self.silver, self.gold, self.authortime, self.authorscore
        )
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Dependency {
    file: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MapType {
    Challenge,
}

impl TryFrom<&str> for MapType {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "challenge" => Ok(MapType::Challenge),
            _ => Err(format!("Unknown map type: {}", value)),
        }
    }
}

impl Default for MapType {
    fn default() -> Self {
        MapType::Challenge
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MapVersion {
    TMc6,
}

impl TryFrom<&str> for MapVersion {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "tmc.6" => Ok(MapVersion::TMc6),
            _ => Err(format!("Unknown map version: {}", value)),
        }
    }
}

impl Default for MapVersion {
    fn default() -> Self {
        MapVersion::TMc6
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Environment {
    Stadium,
}

impl TryFrom<&str> for Environment {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "stadium" => Ok(Environment::Stadium),
            _ => Err(format!("Unknown environment: {}", value)),
        }
    }
}

impl Default for Environment {
    fn default() -> Self {
        Environment::Stadium
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Mood {
    Day,
    Sunset,
    Sunrise,
    Night,
}

impl TryFrom<&str> for Mood {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "day" => Ok(Mood::Day),
            "sunset" => Ok(Mood::Sunset),
            "sunrise" => Ok(Mood::Sunrise),
            "night" => Ok(Mood::Night),
            _ => Err(format!("Unknown mood: {}", value)),
        }
    }
}

impl Default for Mood {
    fn default() -> Self {
        Mood::Day
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DescType {
    Race,
}

impl TryFrom<&str> for DescType {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "race" => Ok(DescType::Race),
            _ => Err(format!("Unknown desc.type: {}", value)),
        }
    }
}

impl Default for DescType {
    fn default() -> Self {
        DescType::Race
    }
}
