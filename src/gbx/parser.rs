//! Package containing the parser for GBX Files.
//! The datatypes used are defined in the [gbx](crate::gbx) module, with [GBX](crate::gbx::GBX) being the main one.
use super::*;
use std::convert::TryInto;
use std::io;
use std::io::Read;
use std::{fs::File, num::ParseIntError};

use xml::reader::{EventReader, XmlEvent};

const HEADER_START_TOKEN: &[u8] = "<header ".as_bytes();
const HEADER_END_TOKEN: &[u8] = "</header>".as_bytes();

const THUMBNAIL_START_TOKEN: &[u8] = &[0xFF, 0xD8, 0xFF];
const THUMBNAIL_END_TOKEN: &[u8] = &[0xFF, 0xD9];

#[derive(Debug)]
pub enum ParseError {
    MissingGBXMagic,
    FileTooShort,
    HeaderNotFound,
    ThumbnailNotFound,
    HeaderValueError(ParseIntError),
    HeaderTryIntoEnumError(String),
    IOError(io::Error),
    Unknown,
}

/// Reads the contents from `filename` and parses them identically to [parse_from_buffer](parse_from_buffer).
///
/// Note, that the [GBXOrigin](GBXOrigin) of the returned [GBX](GBX).[GBXHeader](GBXHeader) will be `File{path:<filepath>}`.
pub fn parse_from_file(filename: &str) -> Result<GBX, ParseError> {
    let mut buffer = Vec::new();
    let mut f = File::open(filename).map_err(|x| ParseError::IOError(x))?;
    f.read_to_end(&mut buffer)
        .map_err(|x| ParseError::IOError(x))?;
    let mut gbx = parse_from_buffer(&buffer)?;
    gbx.origin = GBXOrigin::File {
        path: String::from(filename),
    };
    Ok(gbx)
}

/// Util. function to find first match of a sub-slice in a slice
fn find_window(buf: &[u8], needle: &[u8]) -> Option<usize> {
    buf.windows(needle.len()).position(|w| w == needle)
}

/// Parses the xml included in GBX file (maybe there's a more elegant way to do this)
fn parse_header_xml<'a>(buf: &[u8]) -> Result<GBXHeader, ParseError> {
    let xmlp = EventReader::new(buf);

    let mut header = GBXHeader::default();

    for e in xmlp {
        match e {
            Ok(XmlEvent::StartElement {
                name, attributes, ..
            }) => match name.local_name.as_str() {
                "header" => {
                    for attr in attributes {
                        match attr.name.local_name.as_str() {
                            "type" => {
                                header.maptype = MapType::try_from(attr.value.as_str())
                                    .map_err(|e| ParseError::HeaderTryIntoEnumError(e))?
                            }
                            "version" => {
                                header.mapversion = MapVersion::try_from(attr.value.as_str())
                                    .map_err(|e| ParseError::HeaderTryIntoEnumError(e))?
                            }
                            "exever" => header.exever = String::from(attr.value),
                            _ => println!("Unkown header attribute: {}", attr.name.local_name),
                        }
                    }
                }
                "ident" => {
                    for attr in attributes {
                        match attr.name.local_name.as_str() {
                            "uid" => header.uid = attr.value,
                            "name" => header.name = attr.value,
                            "author" => header.author = attr.value,
                            _ => println!("Unknown ident attribute: {}", attr.name.local_name),
                        }
                    }
                }
                "desc" => {
                    for attr in attributes {
                        match attr.name.local_name.as_str() {
                            "envir" => {
                                header.envir = Environment::try_from(attr.value.as_str())
                                    .map_err(|e| ParseError::HeaderTryIntoEnumError(e))?
                            }
                            "mood" => {
                                header.mood = Mood::try_from(attr.value.as_str())
                                    .map_err(|e| ParseError::HeaderTryIntoEnumError(e))?
                            }
                            "type" => {
                                header.desctype = DescType::try_from(attr.value.as_str())
                                    .map_err(|e| ParseError::HeaderTryIntoEnumError(e))?
                            }
                            "nblaps" => {
                                header.nblaps = attr
                                    .value
                                    .parse()
                                    .map_err(|p| ParseError::HeaderValueError(p))?
                            }
                            "price" => {
                                header.price = attr
                                    .value
                                    .parse()
                                    .map_err(|p| ParseError::HeaderValueError(p))?
                            }
                            _ => println!("Unknown desc attribute: {}", attr.name.local_name),
                        }
                    }
                }
                "times" => {
                    let mut times = Times::default();
                    for attr in attributes {
                        match attr.name.local_name.as_str() {
                            "bronze" => {
                                if let Ok(s) = attr.value.parse() {
                                    times.bronze = Some(s)
                                } else if attr.value == "-1" {
                                    times.bronze = None
                                }
                            }
                            "silver" => {
                                if let Ok(s) = attr.value.parse() {
                                    times.silver = Some(s)
                                } else if attr.value == "-1" {
                                    times.silver = None
                                }
                            }
                            "gold" => {
                                if let Ok(s) = attr.value.parse() {
                                    times.gold = Some(s)
                                } else if attr.value == "-1" {
                                    times.gold = None
                                }
                            }
                            "authortime" => {
                                if let Ok(s) = attr.value.parse() {
                                    times.authortime = Some(s)
                                } else if attr.value == "-1" {
                                    times.authortime = None
                                }
                            }
                            "authorscore" => {
                                if let Ok(s) = attr.value.parse() {
                                    times.authorscore = Some(s)
                                } else if attr.value == "-1" {
                                    times.authorscore = None
                                }
                            }
                            _ => println!("Unkown time attribute: {}", attr.name.local_name),
                        }
                    }
                    header.times = Some(times)
                }
                "deps" => continue,
                "dep" => {
                    for attr in attributes {
                        if attr.name.local_name == "file" {
                            header.dependencies.push(Dependency { file: attr.value })
                        } else {
                            println!(
                                "Encountered unknown deps field {}={}",
                                attr.name.local_name, attr.value
                            )
                        }
                    }
                }
                _ => println!("Unknown name: {} {:?}", name.local_name, attributes),
            },
            Err(e) => {
                println!("error {}", e);
                break;
            }
            _ => {}
        }
    }

    Ok(header)
}

/// Parses the given slice of bytes as if it was a GBX file.
///
/// This function assumes the XML header included in the GBX file is valid UTF8, and will panic
/// otherwise.
/// As of now the actual map-data is not extracted.
///
/// If you want to parse a file directly see [parse_from_file](parse_from_file).
pub fn parse_from_buffer<'a>(buffer: &'a [u8]) -> Result<GBX, ParseError> {
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

    let thumbnail_start =
        find_window(buffer, THUMBNAIL_START_TOKEN).ok_or(ParseError::ThumbnailNotFound);
    let thumbnail_end = find_window(buffer, THUMBNAIL_END_TOKEN)
        .ok_or(ParseError::ThumbnailNotFound)
        .map(|x| x + THUMBNAIL_END_TOKEN.len());

    let mut header_xml = Vec::new();
    let mut header = Err(ParseError::HeaderNotFound);

    let hs = *header_start.as_ref().unwrap_or(&0);
    let he = *header_end.as_ref().unwrap_or(&0);
    if header_start.is_ok() && header_end.is_ok() {
        header_xml.extend_from_slice(&buffer[hs..he]);
        header = parse_header_xml(&buffer[hs..he]);
    }
    let header_xml = String::from_utf8(header_xml).unwrap();

    let thumbnail = if let (Ok(ts), Ok(te)) = (&thumbnail_start, &thumbnail_end) {
        let mut thumbnail_data = Vec::new();
        thumbnail_data.extend_from_slice(&buffer[*ts..*te]);
        Some(JPEGData(thumbnail_data))
    } else {
        None
    };

    Ok(GBX {
        origin: GBXOrigin::Buffer,
        filesize: buffer.len(),
        header_length: he - hs,
        header_start: hs,
        thumbnail_length: if let (Ok(te), Ok(ts)) = (&thumbnail_end, &thumbnail_start) {
            Some(*te - *ts)
        } else {
            None
        },
        thumbnail_start: thumbnail_start.ok(),
        thumbnail: thumbnail,
        header: header.ok(),
        header_xml,
        bin_header: binary_header,
    })
}

#[cfg(test)]
mod tests {

    #[test]
    fn parse_buf() {
        assert_eq!(1, 1);
    }

    #[test]
    fn parse_file() {
        assert_eq!(1, 1);
    }
}
