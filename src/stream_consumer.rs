use std::collections::{HashMap, VecDeque};

/// `Stream` represents a stream of entries.
///
/// Each stream has a name and a list of entries. The entries are stored in a
/// `VecDeque`, which is a queue-like data structure that allows efficient
/// insertion and removal of elements at both ends.
///
/// # Example command
///
/// ```
/// XADD race:france * rider Castilla speed 30.2 position 1 location_id 1
/// XADD race:france * rider Roberto  speed 90.2 position 3 location_id 6
/// ```
/// # Example code
///
/// ```
/// let mut stream = Stream::new("race:france".to_string());
/// let mut entry = StreamEntry {
///     id: "1692632086370-0".to_string(),
///     fields: HashMap::new(),
/// };
/// entry.fields.insert("rider".to_string(), "Castilla".to_string());
/// entry.fields.insert("speed".to_string(), "30.2".to_string());
/// entry.fields.insert("position".to_string(), "1".to_string());
/// entry.fields.insert("location_id".to_string(), "1".to_string());
/// stream.entries.push_back(entry);
/// ```
#[derive(Debug)]
pub struct Stream {
    name: String,
    entries: VecDeque<StreamEntry>,
    // Additional fields can be added for more complex features, like consumer groups.
}

#[derive(Debug)]
pub struct StreamEntry {
    id: String,                      // The ID of the entry in the stream
    fields: HashMap<String, String>, // The fields and values of the entry
}

impl Stream {
    pub fn new(name: String) -> Self {
        Stream {
            name,
            entries: VecDeque::new(),
        }
    }

    /// setter for the name and entries
    pub fn set_name_and_entries(&mut self, name: String, entries: VecDeque<StreamEntry>) {
        self.name = name;
        self.entries = entries;
    }
    
}

impl StreamEntry {
 
    //initialize with default values and generate a unique id
    pub fn new() -> Self {
        StreamEntry {
            //generate a unique id
            id: Uuid::new_v4().to_string(),
            fields: HashMap::new(),
        }
    }

    //setter for the fields
    pub fn set_field(&mut self, field: &str, value: &str) {
        self.fields.insert(field.to_string(), value.to_string());
    }


}
