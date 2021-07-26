use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;

#[derive(Debug, Deserialize, Serialize)]
pub struct File {
    pub path: Vec<String>,
    pub length: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Info {
    pub name: String,
    pub pieces: ByteBuf,
    #[serde(rename = "piece length")]
    pub piece_length: i64,
    pub files: Vec<File>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Torrent {
    info: Info,
    announce: String,
    #[serde(default)]
    encoding: String,
    #[serde(rename = "creation date")]
    creation_date: i64,
    // #[serde(rename = "comment")]
    // comment: Option<String>,
    // #[serde(default, rename = "created by")]
    // created_by: String,
}

impl Torrent {
    pub fn new(announce: String, creation_date: i64, info: Info) -> Self {
        Torrent {
            info,
            announce,
            encoding: String::from("UTF-8"),
            creation_date,
        }
    }
}
