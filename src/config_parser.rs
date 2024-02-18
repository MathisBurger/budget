use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use toml::Table;

pub fn parse_config() -> Result<Table, Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let path = Path::new(&args[1]);
    let mut file = File::open(&path).expect("FILE NOT FOUND");
    let mut data = String::new();
    file.read_to_string(&mut data)?;
    return Ok(toml::from_str(data.as_str()).expect("Data"));
}