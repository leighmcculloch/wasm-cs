use base64::Engine as _;
use clap::{Args, Parser, ValueEnum};
use pretty_hex::{config_hex, HexConfig};
use std::{
    error::Error,
    fs,
    io::{stdout, Write},
    path::Path,
};

/// Read a custom section.
#[derive(Args, Debug)]
#[command()]
pub struct Cmd {
    /// Name of custom section to read.
    #[arg(required = true)]
    section_name: String,

    /// Output format.
    #[arg(short, long, value_enum, default_value_t)]
    format: OutputFormat,
}

#[derive(Default, Parser, ValueEnum, Clone, Debug)]
enum OutputFormat {
    Binary,
    Base64,
    Hex,
    #[default]
    PrettyHex,
}

impl Cmd {
    pub fn run(&self, wasm: &Path) -> Result<(), Box<dyn Error>> {
        let bytes = fs::read(wasm).map_err(|err| format!("failed to read {wasm:?}: {err}"))?;

        let parser = wasmparser::Parser::new(0);
        for payload in parser.parse_all(&bytes) {
            let wasmparser::Payload::CustomSection(r) = payload? else {
                continue;
            };
            if r.name() != self.section_name {
                continue;
            }
            match self.format {
                OutputFormat::Binary => {
                    stdout().write_all(r.data())?;
                }
                OutputFormat::Base64 => {
                    let b64 = base64::engine::general_purpose::STANDARD.encode(r.data());
                    stdout().write_all(b64.as_bytes())?;
                }
                OutputFormat::Hex => {
                    let cfg = HexConfig {
                        title: false,
                        ascii: false,
                        width: 0,
                        group: 0,
                        chunk: 0,
                        max_bytes: usize::MAX,
                    };
                    let hex = config_hex(&r.data(), cfg);
                    println!("{hex}");
                }
                OutputFormat::PrettyHex => {
                    let cfg = HexConfig {
                        title: true,
                        ascii: true,
                        width: 16,
                        group: 4,
                        chunk: 1,
                        max_bytes: usize::MAX,
                    };
                    let hex = config_hex(&r.data(), cfg);
                    println!("{hex}");
                }
            }
            return Ok(());
        }

        Err(format!("Section `{}` not found", self.section_name).into())
    }
}
