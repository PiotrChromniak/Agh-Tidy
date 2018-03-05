extern crate regex;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use regex::Regex;

fn main() {
    if let Some(first_arg) = env::args().nth(1) {
        println!("input file: {}", &first_arg);
        let dict = [
            ("Zaawansowane techniki internetowe", "ZTI"),
            ("Techniki mikroprocesorowe", "Mikroprocesory"),
            ("Eksploracja danych", "ED"),
            ("Modelowanie procesów fizycznych", "MPF"),
            ("Systemy równoległe i rozproszone", "SRiR"),
            ("Fizykochemia procesów", "Fizykochemia"),
            ("Fizyka III", "Fizyka")
        ];
        let path = Path::new(&first_arg);
        let mut file = File::open(&path).expect("couldn't open imput file");
        let mut content = String::new();
        file.read_to_string(&mut content).expect("couldn't read from input file");

        for &(key, rep) in dict.into_iter() {
            content = content.replace(key, rep);
        }

        let trim_unnecessary_word = Regex::new(r"Sala:\s?").unwrap();
        let trim_teacher = Regex::new(r"Prowadzący: (?:mgr |inż. |prof. |dr |hab. )*(.*?)").unwrap();
        let reformat_location = Regex::new(r"(?P<building>[a-zA-Z]-\d{1,2}) - s. (?P<class_num>\d{1,3}\w?)").unwrap();

        let formatted = Some(content)
        .map(|text| trim_teacher.replace_all(&text, "").into_owned())
        .map(|text| reformat_location.replace_all(&text, "$building $class_num").into_owned())
        .map(|text| trim_unnecessary_word.replace_all(&text, "").into_owned());

        let formatted_content = formatted.expect("file formatting failed");

        let new_path = Path::new("plan_zajec_plus.ics");
        let mut new_file = File::create(&new_path).expect("couldn't create output file");
        new_file.write_all(formatted_content.as_bytes())
        .expect("couldn't write to output file");
        
        println!("File succesfuly reformated");
    } else {
        println!("No file specificated");
    }
}