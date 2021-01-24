use std::{convert::TryFrom, str::FromStr};

use xml::{reader::XmlEvent, EventReader};

use crate::gbx::{GBXVersion, ReplayXMLHeader};

use super::ParseError;

/// Parses the xml included in GBX replay
pub(crate) fn parse_replay_xml(buf: &[u8]) -> Result<ReplayXMLHeader, ParseError> {
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
                                    .map_err(ParseError::HeaderTryIntoEnumError)?
                            }
                            "exever" => {
                                header.exever = attr.value;
                            }
                            _ => (),
                        }
                    }
                }
                "challenge" => {
                    for attr in attributes {
                        if let "uid" = attr.name.local_name.as_str() {
                            header.challenge_uid = attr.value;
                        }
                    }
                }
                "times" => {
                    for attr in attributes {
                        match attr.name.local_name.as_str() {
                            "best" => {
                                header.score.best = u32::from_str(attr.value.as_str())
                                    .map_err(ParseError::HeaderValueError)?
                            }
                            "respawns" => {
                                header.score.respawns = u32::from_str(attr.value.as_str())
                                    .map_err(ParseError::HeaderValueError)?
                            }
                            "stuntscore" => {
                                header.score.stuntscore = u32::from_str(attr.value.as_str())
                                    .map_err(ParseError::HeaderValueError)?
                            }
                            "validable" => {
                                header.score.validable = 0
                                    != u32::from_str(attr.value.as_str())
                                        .map_err(ParseError::HeaderValueError)?
                            }
                            _ => (),
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

    if is_replay {
        Ok(header)
    } else {
        Err(ParseError::HeaderNotFound)
    }
}

#[cfg(test)]
mod tests {
    use crate::gbx::parser::ParseError;

    use super::parse_replay_xml;

    #[test]
    fn successfull_parse() {}

    #[test]
    fn unuccessfull_parse() {
        let pairs: &[(&[u8], ParseError)] = &[
            (b"<header></header>", ParseError::HeaderNotFound),
            (b"", ParseError::HeaderNotFound),
        ];

        for p in pairs {
            match parse_replay_xml(p.0) {
                Err(e) => assert_eq!(
                    std::mem::discriminant(&e),
                    std::mem::discriminant(&p.1),
                    "Wrong error returned: {:?}!={:?}",
                    e,
                    p.1
                ),
                Ok(_) => panic!(
                    "{} should fail with {:?} but didn't",
                    std::str::from_utf8(p.0).unwrap_or("<utf8 decode error>"),
                    p.1
                ),
            }
        }
    }
}
