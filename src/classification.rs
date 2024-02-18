use toml::Table;

pub fn classify(config: Table, line: String) -> (String, f64) {
    let classifications = &config["classifications"].as_table().unwrap();
    let line_split = line.split(";").collect::<Vec<&str>>();
    let mut cleaned_line = line_split[4]
        .replace(".", "");
    cleaned_line = cleaned_line.replace(",", ".")
        .trim().parse().unwrap();
    let float_value: f64 = cleaned_line.parse().unwrap();
    for classification in classifications.keys() {
        let classification_elements  = classifications
            .get(classification)
            .unwrap()
            .as_array()
            .unwrap();
        for value in  classification_elements {
            if line_split[3].contains(value.as_str().unwrap()) {
                return (classification.clone(), float_value);
            }
        }
    }

    // Dynamic classification
    (String::new(), float_value)
}