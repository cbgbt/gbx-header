use std::fs::write;

use ansi_term::Color::Red;
use ansi_term::Style;
use clap::{App, Arg};
use gbx_header::*;

fn main() {
    let error_style = Style::new().bold().fg(Red);

    let matches = App::new("gbx-info")
        .version("0.1.0")
        .author("Markus Becker")
        .arg(
            Arg::with_name("thumbnail")
                .short("t")
                .long("thumbnail")
                .help("Path to write thumbnail data to (jpg)")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::with_name("quiet")
                .long("quiet")
                .short("q")
                .required(false)
                .help("Reduce info output"),
        )
        .arg(
            Arg::with_name("xml_out")
                .long("xml")
                .help("Path to write internal XML data to")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::with_name("json_out")
                .long("json")
                .help("Path to serialize GBX file as json to")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::with_name("file")
                .required(true)
                .help("Gbx file path to analyze"),
        )
        .get_matches();

    let filename = matches.value_of("file").unwrap(); // Safe bc required

    let gbx = parse_from_file(filename);
    if let Err(p) = gbx {
        println!("{}", error_style.paint(format!("{:?}", p)));
        return;
    }
    let gbx = gbx.unwrap();
    if !matches.is_present("quiet") {
        println!("{}", gbx);
    }

    if matches.is_present("thumbnail") {
        let thumbnail_file = matches.value_of("thumbnail").unwrap();
        if let Some(data) = &gbx.thumbnail {
            match write(thumbnail_file, &data.0) {
                Ok(_) => {
                    println!("Successfully written thumbnail to {}", thumbnail_file)
                }
                Err(e) => {
                    println!("Writing thumbnail to {} failed with {}", thumbnail_file, e)
                }
            }
        } else {
            println!("No thumbnail present");
        }
    }

    if matches.is_present("xml_out") {
        let xml_file = matches.value_of("xml_out").unwrap();
        match write(xml_file, &gbx.header_xml) {
            Ok(_) => {
                println!("Successfully written xml to {}", xml_file)
            }
            Err(e) => {
                println!("Writing xml to {} failed with {}", xml_file, e)
            }
        }
    }

    if matches.is_present("json_out") {
        let json_file = matches.value_of("json_out").unwrap();
        match write(json_file, serde_json::to_string(&gbx).unwrap()) {
            Ok(_) => {
                println!("Successfully written json to {}", json_file)
            }
            Err(e) => {
                println!("Writing json to {} failed with {}", json_file, e)
            }
        }
    }
}
