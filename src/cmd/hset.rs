use bytes::Bytes;
use tracing::{debug, instrument};
use crate::db::Db;
use crate::{Connection, Frame};

#[derive(Debug)]
pub struct HSet {
    /// the lookup key
    key: String,

    /// the field to be stored
    field: String,

    /// the value to be stored
    value: Bytes,
}

impl HSet {
    /// Create a new `HSet` command which sets `key` to `value`.
    pub fn new(key: impl ToString, field: impl ToString, value: Bytes) -> HSet {
        HSet {
            key: key.to_string(),
            field: field.to_string(),
            value,
        }
    }

    /// Get the key
    pub fn key(&self) -> &str {
        &self.key
    }

    /// Get the field
    pub fn field(&self) -> &str {
        &self.field
    }

    /// Get the value
    pub fn value(&self) -> &Bytes {
        &self.value
    }

    pub(crate) fn into_frame(self) -> Frame {
        let mut frame = Frame::array();
        frame.push_bulk(Bytes::from("hset".as_bytes()));
        frame.push_bulk(Bytes::from(self.key.into_bytes()));
        frame.push_bulk(Bytes::from(self.field.into_bytes()));
        frame.push_bulk(self.value);
        frame
    }

    /// Apply the `HSet` command to the specified `Db` instance.
    ///
    /// The response is written to `dst`. This is called by the server in order
    /// to execute a received command.
    #[instrument(skip(self, db, dst))]
    pub(crate) async fn apply(self, db: &Db, dst: &mut Connection) -> crate::Result<()> {
        // Set the value in the shared database state.
        db.hset(self.key, self.field, self.value);

        // Create a success response and write it to `dst`.
        let response = Frame::Simple("OK".to_string());
        debug!(?response);
        dst.write_frame(&response).await?;

        Ok(())
    }


    /// Parse a `HSet` instance from a received frame.
    pub fn parse_frames(parse: &mut crate::Parse) -> crate::Result<HSet> {
        // The `HSET` string has already been consumed. Extract the `key`
        // and `value` values from the frame.
        //
        // The `key` must be a valid string.
        let key = parse.next_string()?;

        // The `field` must be a valid string.
        let field = parse.next_string()?;

        // The `value` is arbitrary bytes.
        let value = parse.next_bytes()?;

        Ok(HSet { key, field, value })
    }


}