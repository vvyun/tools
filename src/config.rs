use std::collections::HashMap;
use std::fs;

struct Config {
    info: HashMap<String, String>,
}

impl Config {
    fn new() -> Self {
        let result = fs::read_to_string("./config.json");
        let result1: HashMap<String, String> = serde_json::from_str(result.unwrap().as_str()).unwrap();

        Config { info: HashMap::new() }
    }

    fn read_info(&mut self, key: &str) -> String {
        self.info.get(key).expect("").to_string()
    }
}