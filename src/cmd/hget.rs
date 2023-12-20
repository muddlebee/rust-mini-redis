use bytes::Bytes;
use tracing::debug;
use crate::db::Db;
use crate::{Connection, Frame};
use crate::parse::Parse;

#[derive(Debug)]
pub struct HGet {
    key: String,
    field: String,
}

impl HGet {
    pub fn new(key: String, field: String) -> Self {
        Self { key, field }
    }


    pub fn get_key(&self) -> &String {
        &self.key
    }

    pub fn get_field(&self) -> &String {
        &self.field
    }

    pub fn parse_frames(parse: &mut Parse) -> crate::Result<HGet> {
        let key = parse.next_string()?;
        let field = parse.next_string()?;

        Ok(HGet { key, field })
    }

    pub async fn apply(self, db: &Db, dst: &mut Connection) -> crate::Result<()> {
        // get the value from the shared database state.
        let response = match db.hget(&self.key, &self.field) {
            Some(value) => Frame::Bulk(value),
            None => Frame::Null,
        };

        debug!(?response);

        dst.write_frame(&response).await?;

        Ok(())
    }


    // implement into_frame
    pub fn into_frame(self) -> Frame {
        let mut frame = Frame::array();
        frame.push_bulk(Bytes::from("hget".as_bytes()));
        frame.push_bulk(Bytes::from(self.key.into_bytes()));
        frame.push_bulk(Bytes::from(self.field.into_bytes()));
        frame
    }
}