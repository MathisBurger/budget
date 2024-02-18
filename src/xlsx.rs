use rust_xlsxwriter::{Color, ConditionalFormatCell, ConditionalFormatCellRule, Format, Worksheet};

pub struct Writer {
    row_count: u32,
    greater_format: ConditionalFormatCell,
    lower_format: ConditionalFormatCell
}

impl Writer {

    pub fn new() -> Self {
        let greater = ConditionalFormatCell::new()
            .set_rule(ConditionalFormatCellRule::GreaterThan(0))
            .set_format(Format::new().set_background_color(Color::Green));
        let lower = ConditionalFormatCell::new()
            .set_rule(ConditionalFormatCellRule::LessThan(0))
            .set_format(Format::new().set_background_color(Color::Red));
        Writer {
            row_count: 0,
            greater_format: greater,
            lower_format: lower
        }
    }

    pub fn write_row(&mut self, data: String, classification: String, worksheet: &mut Worksheet) {
        let split: Vec<&str> = data.split(";").collect();
        let mut c: u16 = 0;
        for element in split {
            worksheet.write(self.row_count, c, element).expect("ERROR WHILE WRITING");
            c += 1;
        }
        worksheet.add_conditional_format(0, 4, 65536, 4, &self.lower_format)
            .expect("CANNOT ADD FORMATTING");
        worksheet.add_conditional_format(0, 4, 65536, 4, &self.lower_format)
            .expect("CANNOT ADD FORMATTING");
        worksheet.write(self.row_count, c, classification).expect("ERROR WHILE WRITING");
        self.row_count += 1;
    }
}