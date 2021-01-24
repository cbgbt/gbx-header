use std::{convert::TryFrom, str::FromStr};

use xml::{reader::XmlEvent, EventReader};

use crate::gbx::{GBXVersion, ReplayXMLHeader};

use super::ParseError;

/// Parses the xml included in GBX replay
pub(crate) fn parse_replay_xml<'a>(buf: &[u8]) -> Result<ReplayXMLHeader, ParseError> {
    let xmlp = EventReader::new(buf);

    let mut header = ReplayXMLHeader::default();
    let mut is_replay = false;

    for e in xmlp {
        match e {
            Ok(XmlEvent::StartElement {
                name, attributes, ..
            }) => match name.local_name.as_str() {
                "header" => {
                    for attr in attributes {
                        match attr.name.local_name.as_str() {
                            "type" => match attr.value.as_ref() {
                                "replay" => is_replay = true,
                                _ => continue,
                            },
                            "version" => {
                                header.version = GBXVersion::try_from(attr.value.as_ref())
                                    .map_err(|p| ParseError::HeaderTryIntoEnumError(p))?
                            }
                            "exever" => {
                                header.exever = attr.value;
                            }
                            _ => println!("Unkown header attribute: {}", attr.name.local_name),
                        }
                    }
                }
                "challenge" => {
                    for attr in attributes {
                        match attr.name.local_name.as_str() {
                            "uid" => header.challenge_uid = attr.value,
                            _ => println!("Unknown challenge attribute: {}", attr.name.local_name),
                        }
                    }
                }
                "times" => {
                    for attr in attributes {
                        match attr.name.local_name.as_str() {
                            "best" => {
                                header.score.best = u32::from_str(attr.value.as_str())
                                    .map_err(|e| ParseError::HeaderValueError(e))?
                            }
                            "respawns" => {
                                header.score.respawns = u32::from_str(attr.value.as_str())
                                    .map_err(|e| ParseError::HeaderValueError(e))?
                            }
                            "stuntscore" => {
                                header.score.stuntscore = u32::from_str(attr.value.as_str())
                                    .map_err(|e| ParseError::HeaderValueError(e))?
                            }
                            "validable" => {
                                header.score.validable = 0
                                    != u32::from_str(attr.value.as_str())
                                        .map_err(|e| ParseError::HeaderValueError(e))?
                            }
                            _ => println!("Unknown times attribute: {}", attr.name.local_name),
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

    if is_replay {
        Ok(header)
    } else {
        Err(ParseError::HeaderNotFound)
    }
}
