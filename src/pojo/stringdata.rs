use std::collections::*;

pub struct StringData {
    data: HashMap<String, String>,
}

impl StringData {
    pub fn new() -> Self {
        StringData { data: HashMap::new() }
    }

    pub fn get(&self, key: String) -> &str {
        let x = &self.data.get(&key);
        match x {
            Some(i) => i,
            _ => "nul"
        }
    }

    pub fn set(&mut self, key: String, val: String) {
        &self.data.insert(key, val);
    }


    pub fn data(&self) -> &HashMap<String, String> {
        &self.data
    }

    pub fn set_data(&mut self, data: HashMap<String, String>) {
        self.data = data;
    }
}