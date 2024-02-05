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

    // Additional methods for managing the stream and its entries
    pub fn xadd(&mut self, entry_fields: HashMap<String, String>) -> crate::Result<()> {
        //TODO: implement this function

        print!("XADD command called with entry fields: {:?}", entry_fields);

        //  unimplemented!()
        return Ok(());
    }

    //xread with self
    pub fn xread(&mut self, id: &str) -> crate::Result<()> {
     return Ok(());
    }

    //xrange with self
    pub fn xrange(&mut self, start: &str, end: &str) -> crate::Result<()> {
        return Ok(());
    }

}
