use std::collections::HashMap;
use bytes::Bytes;
use tracing::{debug, instrument};
use crate::db::Db;
use crate::{Connection, Frame};
use crate::Stream;

/// XADD stream entry [entry ...]
/// Appends the specified stream entry to the stream at the specified key.
#[derive(Debug)]
pub struct XAdd {
    /// Name of the stream to set
    stream_name: String,
    /// Entries as a vector of key-value pairs
    entries: Vec<String>,
}

/// XREAD [COUNT count] [BLOCK milliseconds] STREAMS key [key ...] ID [ID ...]
/// Read data from one or multiple streams, only returning entries with an ID greater than the last received ID reported by the caller.
#[derive(Debug)]
pub struct XRead {
    /// Name of stream to set
    stream: String,

    /// Name of entry to set
    entry: String,

    /// Value to set.
    value: Bytes,
}

/// XRANGE key start end [COUNT count]
/// Returns the stream entries matching a given range of IDs.
#[derive(Debug)]
pub struct XRange {
    /// Name of stream to set
    stream: String,

    /// Name of entry to set
    entry: String,

    /// Value to set.
    value: Bytes,
}

impl XAdd {
    /// Create a new `XAdd` command which sets `key` to `value`.
    pub fn new(stream: impl ToString, entries: Vec<String>) -> XAdd {
        XAdd {
            stream_name: stream.to_string(),
            entries,
        }
    }

    /// Get the stream
    pub fn stream(&self) -> &str {
        &self.stream_name
    }



    pub(crate) fn into_frame(self) -> Frame {
        let mut frame = Frame::array();
        frame.push_bulk(Bytes::from("xadd".as_bytes()));
        frame.push_bulk(Bytes::from(self.stream_name.into_bytes()));
        for entry in self.entries {
            frame.push_bulk(Bytes::from(entry.into_bytes()));
        }
        frame
    }

    /// Apply the `XAdd` command to the specified `Db` instance.
    ///
    /// The response is written to `dst`. This is called by the server in order
    /// to execute a received command.
    #[instrument(skip(self, db, dst))]
    pub(crate) async fn apply(self, db: &Db, dst: &mut Connection) -> crate::Result<()> {
        db.xadd(self.stream_name, self.entries);
        Ok(())
    }
}

impl XRead {
    /// Create a new `XRead` command which sets `key` to `value`.
    pub fn new(stream: impl ToString, entry: impl ToString, value: Bytes) -> XRead {
        XRead {
            stream: stream.to_string(),
            entry: entry.to_string(),
            value,
        }
    }

    /// Get the stream
    pub fn stream(&self) -> &str {
        &self.stream
    }

    /// Get the entry
    pub fn entry(&self) -> &str {
        &self.entry
    }

    /// Get the value
    pub fn value(&self) -> &Bytes {
        &self.value
    }

    pub(crate) fn into_frame(self) -> Frame {
        let mut frame = Frame::array();
        frame.push_bulk(Bytes::from("xread".as_bytes()));
        frame.push_bulk(Bytes::from(self.stream.into_bytes()));
        frame.push_bulk(Bytes::from(self.entry.into_bytes()));
        frame.push_bulk(self.value);
        frame
    }

    /// Apply the `XRead` command to the specified `Db` instance.
    ///
    /// The response is written to `dst`. This is called by the server in order
    /// to execute a received command.
    #[instrument(skip(self, db, dst))]
    pub(crate) async fn apply(self, db: &Db, dst: &mut Connection) -> crate::Result<()> {

        // Set the value in the shared database state.

        Ok(())
    }
}

impl XRange {
    /// Create a new `XRange` command which sets `key` to `value`.
    pub fn new(stream: impl ToString, entry: impl ToString, value: Bytes) -> XRange {
        XRange {
            stream: stream.to_string(),
            entry: entry.to_string(),
            value,
        }
    }

    /// Get the stream
    pub fn stream(&self) -> &str {
        &self.stream
    }

    /// Get the entry
    pub fn entry(&self) -> &str {
        &self.entry
    }

    /// Get the value
    pub fn value(&self) -> &Bytes {
        &self.value
    }

    pub(crate) fn into_frame(self) -> Frame {
        let mut frame = Frame::array();
        frame.push_bulk(Bytes::from("xrange".as_bytes()));
        frame.push_bulk(Bytes::from(self.stream.into_bytes()));
        frame.push_bulk(Bytes::from(self.entry.into_bytes()));
        frame.push_bulk(self.value);
        frame
    }

    /// Apply the `XRange` command to the specified `Db` instance.
    ///
    /// The response is written to `dst`. This is called by the server in order
    /// to execute a received command.
    #[instrument(skip(self, db, dst))]
    pub(crate) async fn apply(self, db: &Db, dst: &mut Connection) -> crate::Result<()> {
        // Set the value in the shared database state.

        Ok(())
    }
}
