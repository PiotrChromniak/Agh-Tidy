extern crate regex;
use std::error::Error;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use regex::Regex;

fn main() {
    if let Some(args1) = env::args().nth(1) {
        println!("The first argument is {}", &args1);
        let dict = [
            ("Zaawansowane techniki internetowe", "ZTI"),
            ("Techniki mikroprocesorowe", "Mikroprocesory"),
            ("Eksploracja danych", "ED"),
            ("Modelowanie procesów fizycznych", "MPF"),
            ("Systemy równoległe i rozproszone", "SRiR"),
            ("Fizykochemia procesów", "Fizykochemia"),
            ("Fizyka III", "Fizyka")
        ];
        let path = Path::new(&args1);
        let mut file = match File::open(&path){
            Err(why) => panic!("couldn't open {}: {}", path.display(), why.description()),
            Ok(file) => file,
        };
        let mut content = String::new();
        match file.read_to_string(&mut content) {
            Err(why) => panic!("couldn't read {}: {}", path.display(), why.description()),
            Ok(_) => println!("succesfully read content"),
        };

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

        let new_path = Path::new("plan_zajec_plus.ics");
        let mut new_file = match File::create(&new_path){
            Err(why) => panic!("couldn't create output file {} {}", new_path.display(), why.description()),
            Ok(created_file) => created_file,
        };
        match new_file.write_all(formatted.unwrap().as_bytes()) {
        Err(why) => {
            panic!("couldn't write: {}", why.description())
        },
        Ok(_) => println!("successfully wrote to" ),
    }
        println!("File succesfuly reformated");
    } else {
        println!("No file specificated");
    }
}