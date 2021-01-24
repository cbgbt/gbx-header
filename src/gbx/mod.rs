pub mod parser;

use fmt::{Debug, Display};
use serde::{Deserialize, Serialize};
use std::{convert::TryFrom, fmt};

use enum_repr::EnumRepr;

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
    thumbnail_start: Option<usize>,
    thumbnail_length: Option<usize>,
    pub thumbnail: Option<JPEGData>,
    pub bin_header: GBXBinaryHeader,
    pub challenge_header: Option<ChallengeXMLHeader>,
    pub replay_header: Option<ReplayXMLHeader>,
    pub header_xml: String,
}

impl Display for GBX {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn unoption<T: Display>(o: &Option<&T>) -> String {
            o.map(|x| format!("{}", x))
                .unwrap_or("Not present".to_owned())
        }
        write!(
                f,
                "GBX Info Dump (Size={}B)\nFrom file={}\n\nMagic\n=====\n{}\n\nChallenge\n=========\n{}\n\nReplay\n======\n{}",
                self.filesize, self.origin, self.bin_header, unoption(&self.challenge_header.as_ref()), unoption(&self.replay_header.as_ref())
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
pub struct ChallengeXMLHeader {
    pub maptype: MapType,
    pub mapversion: GBXVersion,
    pub exever: String,
    pub uid: String,
    pub name: String,
    pub author: String,
    pub envir: Environment,
    pub mood: Mood,
    pub desctype: DescType,
    pub nblaps: u32,
    pub price: u32,
    /// Completion times and scores for the challenge, None if none set.
    pub times: Option<Times>,
    pub dependencies: Vec<Dependency>,
}

impl fmt::Display for ChallengeXMLHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let dependency_files: Vec<&String> = self.dependencies.iter().map(|x| &x.file).collect();
        write!(f, "Map is {:?}/{:?} made in {:?}/{}\nUID: {}\nName: {}\nAuthor: {}\nSetting: {:?}/{:?}\nNumber of laps: {}\nPrice: {}\nTimes: {}\nDependencies[{}]: {:?}",
            self.maptype, self.desctype, self.mapversion, self.exever, self.uid, self.name, self.author, self.envir, self.mood, self.nblaps, self.price, self.times.as_ref().map_or(String::from("<not set>"), |x| format!("{}", x)), self.dependencies.len(), dependency_files,
        )
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct GBXBinaryHeader {
    pub version: u16,
    pub class_id: u32,
}

impl Display for GBXBinaryHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "v{}, class id: {:08x} ({:?})",
            self.version,
            self.class_id,
            MapClass::try_from(self.class_id).map_or("unknown".to_owned(), |c| format!("{:?}", c))
        )
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ReplayXMLHeader {
    version: GBXVersion,
    exever: String,
    challenge_uid: String,
    score: ReplayScore,
}

impl Display for ReplayXMLHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Version: {:?}\nExever.: {}\nChallenge: {}\nScore: {}",
            self.version, self.exever, self.challenge_uid, self.score
        )
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ReplayScore {
    best: u32,
    respawns: u32,
    stuntscore: u32,
    validable: bool,
}

impl Display for ReplayScore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "best={}, respawns={}, stuntscore={}, validable={}",
            self.best, self.respawns, self.stuntscore, self.validable
        )
    }
}

// Times measured in ms
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Times {
    pub bronze: Option<u32>,
    pub silver: Option<u32>,
    pub gold: Option<u32>,
    pub authortime: Option<u32>,
    pub authorscore: Option<u32>,
}

impl fmt::Display for Times {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Bronze: {}, Silver: {}, Gold: {}, Authortime: {}, Authorscore: {}",
            self.bronze
                .map_or(String::from("<not set>"), |x| format!("{}", x)),
            self.silver
                .map_or(String::from("<not set>"), |x| format!("{}", x)),
            self.gold
                .map_or(String::from("<not set>"), |x| format!("{}", x)),
            self.authortime
                .map_or(String::from("<not set>"), |x| format!("{}", x)),
            self.authorscore
                .map_or(String::from("<not set>"), |x| format!("{}", x))
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
pub enum GBXVersion {
    TMc6,
    TMr7,
}

impl TryFrom<&str> for GBXVersion {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "tmc.6" => Ok(GBXVersion::TMc6),
            "tmr.7" => Ok(GBXVersion::TMr7),
            _ => Err(format!("Unknown map version: {}", value)),
        }
    }
}

impl TryFrom<u16> for GBXVersion {
    type Error = String;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            6 => Ok(GBXVersion::TMc6),
            _ => Err(format!("Unknown map version: {}", value)),
        }
    }
}

impl Default for GBXVersion {
    fn default() -> Self {
        GBXVersion::TMc6
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

#[EnumRepr(type = "u32")]
/// IDs and names taken from [wiki.xaseco.org](https://wiki.xaseco.org/wiki/GBX).
#[derive(Debug, Serialize, Deserialize)]
pub enum MapClass {
    CGameCtnChallenge = 0x03043000,
    CGameCtnCollectorList = 0x0301B000,
    CGameCtnChallengeParameters = 0x0305B000,
    CGameCtnBlockSkin = 0x03059000,
    CGameWaypointSpecialProperty = 0x0313B000,
    CGameCtnReplayRecord = 0x03093000,
    CGameGhost = 0x0303F005,
    CGameCtnGhost = 0x03092000,
    CGameCtnCollector = 0x0301A000,
    CGameCtnObjectInfo = 0x0301C000,
    CGameCtnDecoration = 0x03038000,
    CGameCtnCollection = 0x03033000,
    CGameSkin = 0x03031000,
    CGamePlayerProfile = 0x0308C000,
    CMwNod = 0x01001000,
}

impl TryFrom<u32> for MapClass {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        MapClass::from_repr(value).ok_or(())
    }
}
