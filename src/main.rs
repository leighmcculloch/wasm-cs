//! Read and write custom sections in WASM modules.
//!
//! ## Install
//!
//! ```console
//! cargo install --locked wasm-cs
//! ```
//!
//! ## Usage
//!
//! ### List custom sections
//!
//! ```console
//! $ wasm-cs file.wasm ls
//! hello (6 bytes)
//! test (47 bytes)
//! ```
//!
//! ### Read a custom section
//!
//! ```console
//! $ wasm-cs file.wasm read SECTION_NAME
//! Length: 47 (0x2f) bytes
//! 0000:   54 68 65 20  71 75 69 63  6b 20 62 72  6f 77 6e 20   The quick brown
//! 0010:   66 6f 78 20  6a 75 6d 70  73 20 6f 76  65 72 20 74   fox jumps over t
//! 0020:   68 65 20 6c  61 7a 79 20  64 6f 67 2e  2e 2e 0a      he lazy dog....
//! ```
//!
//! ```console
//! $ wasm-cs file.wasm read SECTION_NAME -f hex
//! 54686520717569636b2062726f776e20666f78206a756d7073206f76657220746865206c617a7920646f672e2e2e0a
//! ```
//!
//! ```console
//! $ wasm-cs file.wasm read SECTION_NAME -f base64
//! VGhlIHF1aWNrIGJyb3duIGZveCBqdW1wcyBvdmVyIHRoZSBsYXp5IGRvZy4uLgo=
//! ```
//!
//! ```console
//! $ wasm-cs file.wasm read SECTION_NAME -f binary
//! The quick brown fox jumps over the lazy dog...
//! ```
//!
//! ### Write a custom section
//!
//! ```console
//! $ wasm-cs file.wasm write SECTION_NAME < FILE
//! ```
//!
//! ### Thanks
//!
//! wasm-cs is a fork of [wasm-custom-sections], by Sven Sauleau.
//!
//! [wasm-custom-sections]: https://docs.rs/wasm-custom-section

use clap::{Parser, Subcommand};
use std::{error::Error, path::PathBuf};

mod ls;
mod read;
mod write;

/// Read and write custom sections in WASM modules.
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Root {
    /// WASM file to read and write.
    #[arg(required = true)]
    wasm: PathBuf,

    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand, Debug)]
enum Cmd {
    Ls(ls::Cmd),
    Read(read::Cmd),
    Write(write::Cmd),
}

impl Root {
    fn run(&self) -> Result<(), Box<dyn Error>> {
        match &self.cmd {
            Cmd::Ls(c) => c.run(&self.wasm),
            Cmd::Read(c) => c.run(&self.wasm),
            Cmd::Write(c) => c.run(&self.wasm),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    Root::parse().run()
}
