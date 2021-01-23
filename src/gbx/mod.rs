pub mod parser;

use fmt::Display;
use serde::{Deserialize, Serialize};
use std::{convert::TryFrom, fmt};

/// Container for raw image data (assumed to be a valid jpg)
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

/// Container for any data extracted from a GBX file.
///
/// See [parse_from_file](parser::parse_from_file) and [parse_from_buffer](parser::parse_from_buffer).
/// Serde is not used internally to Deserialize, but added to enable easier integration of these datatypes in other applications.
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

/// Stores the source of the GBX struct.
/// By default a GBX struct will be `Unknown`, the [parser](parser) methods set the
/// `origin` field of the [GBX](GBX) struct accordingly. If you don't want to expose
/// this information about your local filesystem remember to overwrite that field.
#[derive(Debug, Serialize, Deserialize)]
pub enum GBXOrigin {
    File {
        path: String,
    },
    Buffer,
    Unknown,
    /// Added field to allow hiding the origin (library will never use this)
    Hidden,
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
            GBXOrigin::Hidden => write!(f, "<hidden>"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct GBXHeader {
    pub maptype: MapType,
    pub mapversion: MapVersion,
    pub exever: String,
    pub uid: String,
    pub name: String,
    pub author: String,
    pub envir: Environment,
    pub mood: Mood,
    pub desctype: DescType,
    pub nblaps: u32,
    pub price: u32,
    pub times: Times,
    pub dependencies: Vec<Dependency>,
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
    pub bronze: u32,
    pub silver: u32,
    pub gold: u32,
    pub authortime: u32,
    pub authorscore: u32,
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
    pub file: String,
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
