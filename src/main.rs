mod config_parser;
mod classification;

use std::collections::HashMap;
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
    let mut map: HashMap<String, f64> = HashMap::new();

    for line in lines {
        if line.trim() != ";;;;" && !line.trim().ends_with(";;;") {
            let result = classify(config.clone(), line.to_string());
            if map.get(&result.0).is_none() {
                map.insert(result.0, result.1);
            } else {
                let mut value = map.get(&result.0).unwrap().clone();
                value += result.1;
                map.insert(result.0,  value);
            }
        }
    }
    Ok(())
}
