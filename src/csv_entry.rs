use serde::Deserialize;


#[derive(Debug, Deserialize)]
pub struct CsvEntry {
    pub booking_day: String,
    pub valuation_day: String,
    pub progress: String,
    pub booking_text: String,
    pub amount: f64
}

