use toml::Table;
use dialoguer::Select;

pub fn classify(config: Table, line: String) -> (String, f64) {
    let classifications = &config["classifications"].as_table().unwrap();
    let line_split = line.split(";").collect::<Vec<&str>>();
    let mut cleaned_line = line_split[4]
        .replace(".", "")
        .replace("\"", "");
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
    println!("What is the category of this transaction?");
    let possible_classifications = classifications.keys()
        .map(|x|x).collect::<Vec<&String>>();
    let selection = Select::new()
        .with_prompt(line_split[3])
        .items(&*possible_classifications)
        .interact()
        .unwrap();
    (possible_classifications.get(selection).unwrap().to_string(), float_value)
}