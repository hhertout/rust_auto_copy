use schema::Schema;
use std::fs;

use logger::Logger;

pub struct Copier {
    pub config_file_path: String,
}

impl Copier {
    pub fn new(config_file_path: String) -> Copier {
        Copier { config_file_path }
    }

    pub fn parse_yml(&self) -> Schema {
        let content = fs::read_to_string(&self.config_file_path);
        if content.is_err() {
            Logger::error_file_not_found();
        }
        let content_unwrap = content.unwrap();
        let parsed_content: Schema = serde_yaml::from_str(&content_unwrap).unwrap();

        return parsed_content;
    }

    pub fn run_copy(&self, from: &str, to: &str) {
        let result = fs::copy(from, to);
        match result {
            Ok(_) => Logger::copy_success(from, to),
            Err(err) => println!("{} => Copy failed : {}", from, err),
        }
    }
}
