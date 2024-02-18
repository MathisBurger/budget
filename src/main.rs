mod config_parser;
mod classification;
mod xlsx;

use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use colored::Colorize;
use rust_xlsxwriter::Workbook;
use crate::classification::classify;
use crate::config_parser::parse_config;
use crate::xlsx::Writer;

/// Handles all the workflow of the application
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
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();
    let mut writer = Writer::new();
    writer.write_headline(worksheet).expect("CANNOT WRITE HEADLINE");
    writer.adjust_width(worksheet).expect("CANNOT ADJUST WIDTH");

    for line in lines {
        if line.trim() != ";;;;"
            && !line.trim().ends_with(";;;")
            && !line.starts_with("\"Neuer Kontostand")
            && !line.starts_with("Buchungstag")
            && !line.starts_with(";")
            && !line.contains("tze Girokonto\";\"Zeitraum: 30 Tage")
            && !line.starts_with("\"Buchungstag\";\"Wertstellung (Valuta)\"")
            && !line.starts_with("\"Alter Kontostand")
            && line.trim() != ""
        {
            let result = classify(config.clone(), line.to_string());
            writer.write_row(line.trim().to_string(), result.0.clone(), worksheet);
            if map.get(&result.0).is_none() {
                map.insert(result.0, result.1);
            } else {
                let mut value = map.get(&result.0).unwrap().clone();
                value += result.1;
                map.insert(result.0,  value);
            }
        }
    }
    workbook.save("household.xlsx")?;

    for key in map.keys() {
        let value = map.get(key).unwrap();
        if *value > 0.0 {
            println!("{}: {} EUR", key, map.get(key).unwrap().to_string().green());
        } else {
            println!("{}: {} EUR", key, map.get(key).unwrap().to_string().red());
        }
    }
    Ok(())
}
