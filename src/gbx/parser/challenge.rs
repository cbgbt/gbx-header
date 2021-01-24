use std::convert::TryFrom;

use xml::{reader::XmlEvent, EventReader};

use crate::gbx::*;

use super::ParseError;

/// Parses the xml included in GBX file for challenges
pub(crate) fn parse_challenge_header_xml(buf: &[u8]) -> Result<ChallengeXMLHeader, ParseError> {
    let xmlp = EventReader::new(buf);

    let mut header = ChallengeXMLHeader::default();

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
                                    .map_err(ParseError::HeaderTryIntoEnumError)?
                            }
                            "version" => {
                                header.mapversion = GBXVersion::try_from(attr.value.as_str())
                                    .map_err(ParseError::HeaderTryIntoEnumError)?
                            }
                            "exever" => header.exever = attr.value,
                            _ => (),
                        }
                    }
                }
                "ident" => {
                    for attr in attributes {
                        match attr.name.local_name.as_str() {
                            "uid" => header.uid = attr.value,
                            "name" => header.name = attr.value,
                            "author" => header.author = attr.value,
                            _ => (),
                        }
                    }
                }
                "desc" => {
                    for attr in attributes {
                        match attr.name.local_name.as_str() {
                            "envir" => {
                                header.envir = Environment::try_from(attr.value.as_str())
                                    .map_err(ParseError::HeaderTryIntoEnumError)?
                            }
                            "mood" => {
                                header.mood = Mood::try_from(attr.value.as_str())
                                    .map_err(ParseError::HeaderTryIntoEnumError)?
                            }
                            "type" => {
                                header.desctype = DescType::try_from(attr.value.as_str())
                                    .map_err(ParseError::HeaderTryIntoEnumError)?
                            }
                            "nblaps" => {
                                header.nblaps =
                                    attr.value.parse().map_err(ParseError::HeaderValueError)?
                            }
                            "price" => {
                                header.price =
                                    attr.value.parse().map_err(ParseError::HeaderValueError)?
                            }
                            _ => (),
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
                            _ => (),
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
                _ => (),
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
