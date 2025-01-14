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
                "map" => {
                    for attr in attributes {
                        match attr.name.local_name.as_str() {
                            "uid" => header.map_uid = attr.value,
                            "name" => header.map_name = attr.value,
                            _ => (),
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
                                header.score.respawns = i32::from_str(attr.value.as_str())
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
            Err(e) => return Err(ParseError::XMLParseError(e)),
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

    use crate::gbx::{parser::ParseError, ReplayXMLHeader};

    use super::parse_replay_xml;

    #[test]
    fn successfull_parse() {
        let pairs: &[(&'static [u8], Option<ReplayXMLHeader>)] = &[(
            b"<header type='replay'></header>",
            Some(ReplayXMLHeader::default()),
        )];

        for p in pairs {
            match (parse_replay_xml(p.0), p.1.as_ref()) {
                (Ok(h), Some(t)) => {
                    assert_eq!(&h, t);
                }
                (Err(e), _) => panic!("XML Parsing failed with {:?}", e),
                _ => continue,
            }
        }
    }

    #[test]
    fn unuccessfull_parse() {
        if let ParseError::XMLParseError(xml_error) =
            parse_replay_xml(b"").expect_err("Expecting xml lib to fail on empty buffer")
        {
            // If pair.1 == None any Error is accepted
            let pairs: &[(&'static [u8], Option<ParseError>)] = &[
                (b"<header></header>", Some(ParseError::HeaderNotFound)),
                (
                    b"<header type='replay'><times best='-1'></times></header>",
                    None,
                ),
                (b"", Some(ParseError::XMLParseError(xml_error))),
            ];

            for p in pairs {
                match parse_replay_xml(p.0) {
                    Err(e) => {
                        if let Some(exp) = &p.1 {
                            assert_eq!(
                                std::mem::discriminant(&e),
                                std::mem::discriminant(exp),
                                "Wrong error returned: {:?}!={:?}",
                                e,
                                exp
                            );
                        }
                    }
                    Ok(_) => panic!(
                        "{} should fail with {:?} but didn't",
                        std::str::from_utf8(p.0).unwrap_or("<utf8 decode error>"),
                        p.1
                    ),
                }
            }
        }
    }
}
