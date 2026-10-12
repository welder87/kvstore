use std::collections::HashMap;

pub struct KvStore {
    data: HashMap<String, String>,
}

impl KvStore {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.data.get(key).map(|s| s.as_str())
    }

    pub fn set(&mut self, key: impl Into<String>, value: impl Into<String>) {
        // Правило:
        // Метод сохраняет строку Into<String>.
        // Метод только читает строку &str.
        self.data.insert(key.into(), value.into());
    }

    pub fn delete(&mut self, key: &str) {
        self.data.remove(key);
    }
}

fn main() {
    let mut store = KvStore::new();
    store.set("MyKey", "Hello");
    println!("{:?}", store.get("MyKey"));
}
