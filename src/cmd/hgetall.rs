//implement HGetAll

use bytes::Bytes;
use tracing::debug;
use crate::db::Db;
use crate::{Connection, Frame};
use crate::parse::Parse;

#[derive(Debug)]
pub struct HGetAll {
    key: String,
}

impl HGetAll {
    pub fn new(key: String) -> Self {
        Self { key }
    }

    pub fn get_key(&self) -> &String {
        &self.key
    }

    pub fn parse_frames(parse: &mut Parse) -> crate::Result<HGetAll> {
        let key = parse.next_string()?;
        Ok(HGetAll { key })
    }

    pub async fn apply(self, db: &Db, dst: &mut Connection) -> crate::Result<()> {
        // get the value from the shared database state.
        let response = match db.hgetall(&self.key) {
            Some(hash_map) => {
                let mut frame = Frame::Array(Vec::with_capacity(hash_map.len() * 2));
                for (key, value) in hash_map {
                    // For each key-value pair, push the key and value as bulk strings
                    frame.push_bulk(Bytes::from(key.into_bytes()));
                    frame.push_bulk(value);
                }
                frame
            }
            None => Frame::Null,
        };

        debug!(?response);

        dst.write_frame(&response).await?;

        Ok(())
    }

    // implement into_frame
    pub fn into_frame(self) -> Frame {
        let mut frame = Frame::array();
        frame.push_bulk(Bytes::from("hgetall".as_bytes()));
        frame.push_bulk(Bytes::from(self.key.into_bytes()));
        frame
    }
}