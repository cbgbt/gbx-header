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
                .help("File to write thumbnail data to (jpg)")
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
    println!("{}", gbx);

    if matches.is_present("thumbnail") {
        let thumbnail_file = matches.value_of("thumbnail").unwrap();
        match write(thumbnail_file, gbx.thumbnail.0) {
            Ok(_) => {
                println!("Successfully written thumbnail to {}", thumbnail_file)
            }
            Err(e) => {
                println!("Writing thumbnail to {} failed with {}", thumbnail_file, e)
            }
        }
    }
}
