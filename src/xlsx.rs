use rust_xlsxwriter::{Color, ConditionalFormatCell, ConditionalFormatCellRule, Format, Worksheet, XlsxError};

pub struct Writer {
    row_count: u32,
    greater_format: ConditionalFormatCell,
    lower_format: ConditionalFormatCell
}

impl Writer {

    /// Creates a new instance
    pub fn new() -> Self {
        let greater = ConditionalFormatCell::new()
            .set_rule(ConditionalFormatCellRule::GreaterThan(0))
            .set_format(
                Format::new()
                    .set_font_color(Color::Green)
            );
        let lower = ConditionalFormatCell::new()
            .set_rule(ConditionalFormatCellRule::LessThan(0))
            .set_format(
                Format::new()
                    .set_font_color(Color::Red)
            );
        Writer {
            row_count: 1,
            greater_format: greater,
            lower_format: lower
        }
    }

    /// Writes a specific data row to the sheet
    pub fn write_row(&mut self, data: String, classification: String, worksheet: &mut Worksheet) {
        let split: Vec<&str> = data.split(";").collect();
        let mut c: u16 = 0;
        for element in split {
            if c == 4 {
                let mut cleaned_line = element
                    .replace(".", "")
                    .replace("\"", "");
                cleaned_line = cleaned_line.replace(",", ".")
                    .trim().parse().unwrap();
                let float_value: f64 = cleaned_line.parse().unwrap();
                worksheet.write(self.row_count, c, float_value)
                    .expect("ERROR WHILE WRITING");
            } else {
                worksheet.write(self.row_count, c, element.replace("\"", "")).expect("ERROR WHILE WRITING");
            }
            c += 1;
        }
        worksheet.add_conditional_format(0, 4, 65536, 4, &self.lower_format)
            .expect("CANNOT ADD FORMATTING");
        worksheet.add_conditional_format(0, 4, 65536, 4, &self.greater_format)
            .expect("CANNOT ADD FORMATTING");
        worksheet.write(self.row_count, c, classification).expect("ERROR WHILE WRITING");
        self.row_count += 1;
    }

    /// Writes the headline to the sheet
    pub fn write_headline(&mut self, worksheet: &mut Worksheet) -> Result<(), XlsxError> {
        worksheet.write(0, 0, "Buchungstag")?;
        worksheet.write(0, 1, "Wertstellung")?;
        worksheet.write(0, 2, "Vorgang")?;
        worksheet.write(0, 3, "Buchungstext")?;
        worksheet.write(0, 4, "Umsatz in EUR")?;
        worksheet.write(0, 5, "Klassifizierung")?;
        Ok(())
    }

    /// Adjusts the width of the worksheet
    pub fn adjust_width(&mut self, worksheet: &mut Worksheet) -> Result<(), XlsxError> {
        worksheet.set_column_width(0, 14.0)?;
        worksheet.set_column_width(1, 14.0)?;
        worksheet.set_column_width(2, 19.5)?;
        worksheet.set_column_width(3, 125.5)?;
        worksheet.set_column_width(4, 9.5)?;
        worksheet.set_column_width(5, 20.0)?;
        Ok(())
    }
}