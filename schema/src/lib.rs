use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Schema {
    pub on: String,
    pub copy: Option<Copy>,
    pub replace: Option<Replace>,
    pub ignored_dir: Option<Vec<String>>,
}

impl Schema {
    pub fn new() -> Schema {
        Schema {
            on: String::from("."),
            copy: Some(Copy::new()),
            replace: Some(Replace::new()),
            ignored_dir: Some(vec![
                String::from("/.git/"),
                String::from("/node_modules/"),
                String::from("/vendor/"),
                String::from("/target/"),
            ]),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Copy {
    pub files: Vec<String>,
}

impl Copy {
    fn new() -> Copy {
        Copy {
            files: vec![String::from("hello.txt.example:hello.txt")],
        }
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Replace {
    pub global: Vec<String>,
}

impl Replace {
    fn new() -> Replace {
        Replace {
            global: vec![String::from("hello world:hello mom")],
        }
    }
}
