use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct StringPool {
    map: HashMap<String, usize>,
    buf: Vec<String>,
}

impl StringPool {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            buf: Vec::new(),
        }
    }

    pub fn get_index(&mut self, s: &str) -> usize {
        if let Some(&i) = self.map.get(s) {
            i
        } else {
            let old_len = self.buf.len();
            self.map.insert(s.into(), old_len);
            self.buf.push(s.into());
            old_len
        }
    }

    pub fn get_str(&self, index: usize) -> &str {
        &self.buf[index]
    }
}
