//! Package containing the parser for GBX Files.
//! The datatypes used are defined in the [gbx](crate::gbx) module, with [GBX](crate::gbx::GBX) being the main one.

pub mod replay;

use self::replay::parse_replay_xml;

use super::*;

use std::convert::TryInto;
use std::io;
use std::io::Read;
use std::{fs::File, num::ParseIntError};

const HEADER_START_TOKEN: &[u8] = "<header ".as_bytes();
const HEADER_END_TOKEN: &[u8] = "</header>".as_bytes();

#[derive(Debug)]
pub enum ParseError {
    MissingGBXMagic,
    FileTooShort,
    HeaderNotFound,
    ThumbnailNotFound,
    XMLParseError(xml::reader::Error),
    HeaderValueError(ParseIntError),
    HeaderTryIntoEnumError(String),
    IOError(io::Error),
    Unknown,
}

/// Util. function to find first match of a sub-slice in a slice
fn find_window(buf: &[u8], needle: &[u8]) -> Option<usize> {
    buf.windows(needle.len()).position(|w| w == needle)
}

/// Reads the contents from `filename` and parses them identically to [parse_from_buffer](parse_from_buffer).
///
/// Note, that the [GBXOrigin](GBXOrigin) of the returned [GBX](GBX) will be `File{path:<filepath>}`.
pub fn parse_from_file(filename: &str) -> Result<GBX, ParseError> {
    let mut buffer = Vec::new();
    let mut f = File::open(filename).map_err(ParseError::IOError)?;
    f.read_to_end(&mut buffer).map_err(ParseError::IOError)?;
    let mut gbx = parse_from_buffer(&buffer)?;
    gbx.origin = GBXOrigin::File {
        path: String::from(filename),
    };
    Ok(gbx)
}

/// Parses the given slice of bytes as if it was a GBX file.
///
/// This function assumes the XML header included in the GBX file is valid UTF8, and will panic
/// otherwise.
/// As of now the actual map-data is not extracted.
///
/// If you want to parse a file directly see [parse_from_file](parse_from_file).
pub fn parse_from_buffer(buffer: &[u8]) -> Result<GBX, ParseError> {
    if buffer.len() < 3 {
        return Err(ParseError::FileTooShort);
    }

    if &buffer[0..3] != b"GBX" {
        return Err(ParseError::MissingGBXMagic);
    }

    let binary_header = GBXBinaryHeader {
        version: u16::from_le_bytes((&buffer[3..5]).try_into().unwrap()),
        class_id: u32::from_le_bytes((&buffer[9..13]).try_into().unwrap()),
    };

    let header_start = find_window(buffer, HEADER_START_TOKEN).ok_or(ParseError::HeaderNotFound);
    let header_end = find_window(buffer, HEADER_END_TOKEN)
        .ok_or(ParseError::HeaderNotFound)
        .map(|x| x + HEADER_END_TOKEN.len());

    let mut header_xml = Vec::new();
    let mut replay_header = Err(ParseError::HeaderNotFound);

    let hs = *header_start.as_ref().unwrap_or(&0);
    let he = *header_end.as_ref().unwrap_or(&0);

    if header_start.is_ok() && header_end.is_ok() {
        header_xml.extend_from_slice(&buffer[hs..he]);
        replay_header = parse_replay_xml(&buffer[hs..he]);
    }
    let header_xml = String::from_utf8(header_xml).unwrap();

    Ok(GBX {
        origin: GBXOrigin::Buffer,
        filesize: buffer.len(),
        header_length: he - hs,
        header_start: hs,
        replay_header: replay_header.ok(),
        header_xml,
        bin_header: binary_header,
    })
}
