use super::*;
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
    FileTooShort,
    HeaderNotFound,
    ThumbnailNotFound,
    HeaderValueError(ParseIntError),
    HeaderTryIntoEnumError(String),
    IOError(io::Error),
    Unknown,
}

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

fn find_window(buf: &[u8], needle: &[u8]) -> Option<usize> {
    buf.windows(needle.len()).position(|w| w == needle)
}

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
                    for attr in attributes {
                        match attr.name.local_name.as_str() {
                            "bronze" => {
                                header.times.bronze = attr
                                    .value
                                    .parse()
                                    .map_err(|p| ParseError::HeaderValueError(p))?
                            }
                            "silver" => {
                                header.times.silver = attr
                                    .value
                                    .parse()
                                    .map_err(|p| ParseError::HeaderValueError(p))?
                            }
                            "gold" => {
                                header.times.gold = attr
                                    .value
                                    .parse()
                                    .map_err(|p| ParseError::HeaderValueError(p))?
                            }
                            "authortime" => {
                                header.times.authortime = attr
                                    .value
                                    .parse()
                                    .map_err(|p| ParseError::HeaderValueError(p))?
                            }
                            "authorscore" => {
                                header.times.authorscore = attr
                                    .value
                                    .parse()
                                    .map_err(|p| ParseError::HeaderValueError(p))?
                            }
                            _ => println!("Unkown time attribute: {}", attr.name.local_name),
                        }
                    }
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

pub fn parse_from_buffer<'a>(buffer: &'a [u8]) -> Result<GBX, ParseError> {
    let header_start = find_window(buffer, HEADER_START_TOKEN).ok_or(ParseError::HeaderNotFound)?;
    let header_end = find_window(buffer, HEADER_END_TOKEN).ok_or(ParseError::HeaderNotFound)?
        + HEADER_END_TOKEN.len();

    let thumbnail_start =
        find_window(buffer, THUMBNAIL_START_TOKEN).ok_or(ParseError::HeaderNotFound)?;
    let thumbnail_end = find_window(buffer, THUMBNAIL_END_TOKEN)
        .ok_or(ParseError::HeaderNotFound)?
        + THUMBNAIL_END_TOKEN.len();

    let mut header_xml = Vec::new();
    header_xml.extend_from_slice(&buffer[header_start..header_end]);
    let header_xml = String::from_utf8(header_xml).unwrap();

    let mut thumbnail_data = Vec::new();
    thumbnail_data.extend_from_slice(&buffer[thumbnail_start..thumbnail_end]);

    let header = parse_header_xml(&buffer[header_start..header_end])?;

    Ok(GBX {
        origin: GBXOrigin::Buffer,
        filesize: buffer.len(),
        header_start,
        header_length: header_end - header_start,
        thumbnail_start,
        thumbnail_length: thumbnail_end - thumbnail_start,
        thumbnail: JPEGData(thumbnail_data),
        header,
        header_xml,
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
