mod config_parser;
mod classification;

use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use crate::classification::classify;
use crate::config_parser::parse_config;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let path = Path::new(&args[2]);
    let mut file = File::open(&path).expect("FILE NOT FOUND");
    let mut buf = vec![];
    file.read_to_end(&mut buf)?;
    let contents = String::from_utf8_lossy(&buf);
    let lines = contents.split("\n").collect::<Vec<&str>>();

    let config = parse_config().unwrap();
    for line in lines {
        // booking
        if line.trim() != ";;;;" && !line.trim().ends_with(";;;") {
            let result = classify(config.clone(), line.to_string());
            print!("{} {}", result.0, result.1);
        }
    }
    Ok(())
}
