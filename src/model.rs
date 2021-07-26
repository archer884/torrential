use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;

// #[derive(Debug, Deserialize, Serialize)]
// pub struct Node(String, i64);

#[derive(Debug, Deserialize, Serialize)]
pub struct File {
    pub path: Vec<String>,
    pub length: i64,
    // #[serde(default)]
    // md5sum: Option<String>,
}

// #[derive(Debug, Deserialize, Serialize)]
// pub struct Info {
//     name: String,
//     pieces: ByteBuf,
//     #[serde(rename = "piece length")]
//     piece_length: i64,
//     #[serde(default)]
//     md5sum: Option<String>,
//     #[serde(default)]
//     length: Option<i64>,
//     #[serde(default)]
//     files: Option<Vec<File>>,
//     #[serde(default)]
//     private: Option<u8>,
//     #[serde(default)]
//     path: Option<Vec<String>>,
//     #[serde(default, rename = "root hash")]
//     root_hash: Option<String>,
// }

#[derive(Debug, Deserialize, Serialize)]
pub struct Info {
    pub name: String,
    pub pieces: ByteBuf,
    #[serde(rename = "piece length")]
    pub piece_length: i64,
    pub files: Vec<File>,
}



// #[derive(Debug, Deserialize, Serialize)]
// pub struct Torrent {
//     info: Info,
//     #[serde(default)]
//     announce: Option<String>,
//     #[serde(default)]
//     nodes: Option<Vec<Node>>,
//     #[serde(default)]
//     encoding: Option<String>,
//     #[serde(default)]
//     httpseeds: Option<Vec<String>>,
//     #[serde(default, rename = "announce-list")]
//     announce_list: Option<Vec<Vec<String>>>,
//     #[serde(default, rename = "creation date")]
//     creation_date: Option<i64>,
//     #[serde(rename = "comment")]
//     comment: Option<String>,
//     #[serde(default, rename = "created by")]
//     created_by: Option<String>,
// }

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
