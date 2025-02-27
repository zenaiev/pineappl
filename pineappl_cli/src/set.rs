use super::helpers::{self, GlobalConfiguration, Subcommand};
use anyhow::Result;
use clap::{ArgAction, Parser, ValueHint};
use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;

/// Modifies the internal key-value storage.
#[derive(Parser)]
pub struct Opts {
    /// Path to the input grid.
    #[arg(value_hint = ValueHint::FilePath)]
    input: PathBuf,
    /// Path of the modified PineAPPL file.
    #[arg(value_hint = ValueHint::FilePath)]
    output: PathBuf,
    /// Deletes an internal key-value pair.
    #[arg(action = ArgAction::Append, long, value_name = "KEY")]
    delete: Vec<String>,
    /// Sets an internal key-value pair.
    #[arg(
        action = ArgAction::Append,
        allow_hyphen_values = true,
        long,
        num_args(2),
        value_names = &["KEY", "VALUE"]
    )]
    entry: Vec<String>,
    /// Sets an internal key-value pair, with value being read from a file.
    #[arg(
        action = ArgAction::Append,
        long,
        num_args(2),
        value_names = &["KEY", "FILE"]
    )]
    entry_from_file: Vec<String>,
}

impl Subcommand for Opts {
    fn run(&self, _: &GlobalConfiguration) -> Result<ExitCode> {
        let mut grid = helpers::read_grid(&self.input)?;

        for key_value in self.entry.chunks(2) {
            grid.set_key_value(&key_value[0], &key_value[1]);
        }

        for key_file in self.entry_from_file.chunks(2) {
            grid.set_key_value(&key_file[0], &fs::read_to_string(&key_file[1])?);
        }

        for delete in &self.delete {
            grid.key_values_mut().remove(delete);
        }

        helpers::write_grid(&self.output, &grid)
    }
}
