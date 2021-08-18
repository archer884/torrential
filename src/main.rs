use std::{
    fs::{self, File},
    io::{self, Cursor, Read},
    path::PathBuf,
};

mod model;

use catread::CatRead;
use chrono::Utc;
use clap::Clap;
use serde_bytes::ByteBuf;

use crate::model::{Info, Torrent};

const PIECE_LENGTH: usize = 0x40000; // 2 << 12;

#[derive(Clap, Clone, Debug)]
#[clap(version = clap::crate_version!(), about = clap::crate_description!())]
struct Opts {
    /// the path of either the file or folder being shared
    path: String,

    /// the name of either the file or the folder being shared;
    /// advisory; intended to be optional; I'll get around to that
    /// eventually.
    #[clap(short, long)]
    name: String,

    #[clap(short, long)]
    tracker: String,

    /// the location to save the .torrent to
    #[clap(short, long)]
    output: String,
}

fn main() {
    let opts = Opts::parse();
    if let Err(e) = run(&opts) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

fn run(opts: &Opts) -> io::Result<()> {
    let files: Vec<_> = fs::read_dir(&opts.path)?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let meta = entry.metadata().ok()?;
            if meta.is_file() {
                let full_path = entry.path();
                let path = full_path.strip_prefix(&opts.path).ok()?.to_owned();
                Some((meta.len(), full_path, path))
            } else {
                None
            }
        })
        .collect();

    let (actual_length, pieces) = hash_pieces(&files)?;

    // Convert this to an error or... something.
    {
        let expected_length: u64 = files.iter().map(|entry| entry.0).sum();
        let expected_pieces = expected_length / PIECE_LENGTH as u64
            + if expected_length % PIECE_LENGTH as u64 == 0 {
                0
            } else {
                1
            };

        assert_eq!(actual_length, expected_length, "Wrong length");
        assert_eq!(
            expected_pieces,
            (pieces.len() / 20) as u64,
            "Wrong number of piece hashes"
        );
    }

    let info = Info {
        name: opts.name.clone(),
        pieces: ByteBuf::from(pieces),
        piece_length: PIECE_LENGTH as i64,
        files: files
            .into_iter()
            .map(|(length, _, path)| model::File {
                length: length as i64,
                path: vec![path.display().to_string()],
            })
            .collect(),
    };

    let torrent = Torrent::new(opts.tracker.clone(), Utc::now().timestamp(), info);
    let buf = serde_bencode::to_bytes(&torrent).unwrap();
    fs::write(&opts.output, &buf)
}

fn hash_pieces(files: &[(u64, PathBuf, PathBuf)]) -> io::Result<(u64, Vec<u8>)> {
    let sources = files.iter().map(|cx| File::open(&cx.1));
    let mut cat = CatRead::new(sources)?;
    let mut buf = vec![0u8; PIECE_LENGTH].into_boxed_slice();
    let mut pieces = Vec::new();

    let mut total_read = 0;

    loop {
        // Using the entire buffer for the last piece
        // causes the last file to be un-download-able.
        let len = cat.read(&mut buf)?;
        if len > 0 {
            pieces.extend_from_slice(&sha1_sum(&buf[..len])?);
            total_read += len as u64;
        } else {
            return Ok((total_read, pieces));
        }
    }
}

fn sha1_sum(buf: &[u8]) -> io::Result<Vec<u8>> {
    use sha1::{Digest, Sha1};
    let mut digest = Sha1::new();
    let mut reader = Cursor::new(buf);
    io::copy(&mut reader, &mut digest)?;
    Ok(digest.finalize().as_slice().into())
}
