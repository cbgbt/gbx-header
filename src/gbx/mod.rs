pub mod parser;

use fmt::{Debug, Display};
use serde::{Deserialize, Serialize};
use std::{convert::TryFrom, fmt};

use enum_repr::EnumRepr;

/// Container for any data extracted from a GBX file.
///
/// See [parse_from_buffer](parser::parse_from_buffer).
/// Serde is not used internally to Deserialize, but added to enable easier integration of these datatypes in other applications.
#[derive(Debug, Serialize, Deserialize)]
pub struct GBX {
    pub origin: GBXOrigin,
    pub filesize: usize,
    header_start: usize,
    header_length: usize,
    pub bin_header: GBXBinaryHeader,
    pub replay_header: Option<ReplayXMLHeader>,
    pub header_xml: String,
}

impl Display for GBX {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn unoption<T: Display>(o: &Option<&T>) -> String {
            o.map(|x| format!("{}", x))
                .unwrap_or_else(|| "Not present".to_owned())
        }
        write!(
            f,
            "GBX Info Dump (Size={}B)\nFrom file={}\n\nMagic\n=====\n{}\n\nReplay\n======\n{}",
            self.filesize,
            self.origin,
            self.bin_header,
            unoption(&self.replay_header.as_ref())
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

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct ReplayXMLHeader {
    /// Version of the replay file format
    pub version: GBXVersion,
    /// Version on executable player used to make the replay
    pub exever: String,
    /// UID of Map
    pub map_uid: String,
    /// Name of Map
    pub map_name: String,
    /// Score and time
    pub score: ReplayScore,
}

impl Display for ReplayXMLHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Version: {:?}\nExever.: {}\nMap: {}\nScore: {}",
            self.version, self.exever, self.map_name, self.score
        )
    }
}

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct ReplayScore {
    /// Best time in ms
    pub best: u32,
    /// Number of respawns in attempt
    pub respawns: i32,
    pub stuntscore: u32,
    pub validable: bool,
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

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum GBXVersion {
    /// Unknown Type/Version
    Unknown,
    /// Challenge v6
    TMc6,
    /// Replay v7
    TMr7,
}

impl TryFrom<&str> for GBXVersion {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "tmc.6" => Ok(GBXVersion::TMc6),
            "tmr.7" => Ok(GBXVersion::TMr7),
            _ => Err(format!("Unknown GBX file version: {}", value)),
        }
    }
}

impl GBXVersion {
    /// Converts specific GBX file version and type (GBXVersion) into more generic GBXType.
    pub fn content_type(&self) -> GBXType {
        match self {
            GBXVersion::TMc6 => GBXType::Challenge,
            GBXVersion::TMr7 => GBXType::Replay,
            GBXVersion::Unknown => GBXType::Unknown,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum GBXType {
    Challenge,
    Replay,
    /// GBX files can be many types of objects, this repo doesn't aim to implement parsing for most of them.
    Unknown,
}

impl Default for GBXVersion {
    fn default() -> Self {
        GBXVersion::Unknown
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
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

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
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

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
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
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
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
