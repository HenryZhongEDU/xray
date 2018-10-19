extern crate futures;
#[macro_use]
extern crate lazy_static;
#[cfg(test)]
extern crate rand;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate smallvec;

mod btree;
mod buffer;
mod epoch;
mod notify_cell;
mod operation_queue;
pub mod time;
mod tree_map;
mod work_tree;

pub use buffer::{Buffer, Point};
pub use epoch::{BufferId, Cursor, DirEntry, Epoch, FileId, FileStatus, FileType, ROOT_FILE_ID};
use std::borrow::Cow;
use std::fmt;
use std::io;
pub use work_tree::{GitProvider, Operation, WorkTree};

pub type ReplicaId = u64;
pub type UserId = u64;
pub type Oid = [u8; 20];

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    InvalidPath(Cow<'static, str>),
    InvalidOperations,
    InvalidFileId,
    InvalidBufferId,
    InvalidDirEntry,
    InvalidOperation,
    CursorExhausted,
}

impl From<Error> for String {
    fn from(error: Error) -> Self {
        format!("{:?}", error)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}
