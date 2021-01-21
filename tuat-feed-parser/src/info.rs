use serde::Serialize;
use std::collections::HashMap;

/// holds the information id and the information as a hashmap
#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct Info {
    /// the id of the information. found in the tuat feed.
    pub id: u32,
    /// the actual data. key is from the table on the tuat feed.
    pub data: HashMap<String, String>,
}

impl Info {
    /// creates a new `Info`
    pub fn new(id: u32) -> Self {
        Self {
            id,
            data: HashMap::new(),
        }
    }
}
