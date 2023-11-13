use clap::{Args, Parser, ValueEnum};
use std::{error::Error, fs, path::Path};

/// List custom sections.
#[derive(Args, Debug)]
#[command()]
pub struct Cmd {
    /// Output format.
    #[arg(short, long, value_enum, default_value_t)]
    format: OutputFormat,
}

#[derive(Default, Parser, ValueEnum, Clone, Debug)]
enum OutputFormat {
    /// Output the name only.
    NameOnly,
    /// Output the name and size.
    #[default]
    NameSize,
}

impl Cmd {
    pub fn run(&self, wasm: &Path) -> Result<(), Box<dyn Error>> {
        let bytes = fs::read(wasm).map_err(|err| format!("failed to read {wasm:?}: {err}"))?;

        let parser = wasmparser::Parser::new(0);
        for payload in parser.parse_all(&bytes) {
            if let wasmparser::Payload::CustomSection(r) = payload? {
                match self.format {
                    OutputFormat::NameOnly => println!("{}", r.name()),
                    OutputFormat::NameSize => println!("{} ({} bytes)", r.name(), r.data().len()),
                }
            }
        }

        Ok(())
    }
}
