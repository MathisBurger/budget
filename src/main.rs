use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;

mod csv_entry;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let path = Path::new(&args[1]);
    let mut file = File::open(&path).expect("FILE NOT FOUND");
    let mut buf = vec![];
    file.read_to_end(&mut buf)?;
    let contents = String::from_utf8_lossy(&buf);
    let lines = contents.split("\n").collect::<Vec<&str>>();
    for line in lines {
        // booking
    }
    Ok(())
}
