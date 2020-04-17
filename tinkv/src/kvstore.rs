use std::collections::HashMap;

///
#[derive(Debug)]
pub struct KvStore {
    db: HashMap<String, String>,
}

impl KvStore {
    /// 
    pub fn new() -> Self {
        KvStore {
            db: HashMap::new(),
        }
    }

    ///
    pub fn set(&mut self, key: String, value: String) {
        self.db.insert(key, value);
    }

    /// 
    pub fn get(&self, key: String) -> Option<String> {
        self.db.get(&key).map(|x| x.to_string())
    }

    /// 
    pub fn remove(&mut self, key: String) {
        self.db.remove(&key);
    }
}
