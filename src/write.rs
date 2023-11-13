use clap::{Args, Parser, ValueEnum};
use std::{
    error::Error,
    fs,
    io::{stdin, Read},
    path::Path,
};

/// Write a custom section.
#[derive(Args, Debug)]
#[command()]
pub struct Cmd {
    /// Name of custom section to write.
    #[arg(required = true)]
    section_name: String,

    /// Input format.
    #[arg(short, long, value_enum, default_value_t)]
    format: InputFormat,
}

#[derive(Default, Parser, ValueEnum, Clone, Debug)]
enum InputFormat {
    #[default]
    Binary,
}

impl Cmd {
    pub fn run(&self, wasm: &Path) -> Result<(), Box<dyn Error>> {
        let mut bytes = fs::read(wasm).map_err(|err| format!("failed to read {wasm:?}: {err}"))?;

        let mut input_buffer = Vec::new();
        stdin().read_to_end(&mut input_buffer)?;

        wasm_gen::write_custom_section(&mut bytes, &self.section_name, &input_buffer);

        fs::write(wasm, bytes)?;

        Ok(())
    }
}
