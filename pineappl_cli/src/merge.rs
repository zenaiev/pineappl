use super::helpers::{self, GlobalConfiguration, Subcommand};
use anyhow::Result;
use clap::{Parser, ValueHint};
use std::path::PathBuf;
use std::process::ExitCode;

/// Merges one or more PineAPPL grids together.
#[derive(Parser)]
pub struct Opts {
    /// Path of the merged PineAPPL file.
    #[arg(value_hint = ValueHint::FilePath)]
    output: PathBuf,
    /// Path(s) of the files that should be merged.
    #[arg(required = true, value_hint = ValueHint::FilePath)]
    input: Vec<PathBuf>,
    /// Scales all grids with the given factor.
    #[arg(long, short)]
    scale: Option<f64>,
    /// Scales all grids with order-dependent factors.
    #[arg(
        conflicts_with = "scale",
        long,
        num_args = 1,
        value_delimiter = ',',
        value_name = "ALPHAS,ALPHA,LOGXIR,LOGXIF,GLOBAL"
    )]
    scale_by_order: Vec<f64>,
}

impl Subcommand for Opts {
    fn run(&self, _: &GlobalConfiguration) -> Result<ExitCode> {
        let (input0, input_rest) = self.input.split_first().unwrap();
        let mut grid0 = helpers::read_grid(input0)?;

        for i in input_rest {
            grid0.merge(helpers::read_grid(i)?)?;
        }

        if let Some(scale) = self.scale {
            grid0.scale(scale);
        } else if !self.scale_by_order.is_empty() {
            grid0.scale_by_order(
                self.scale_by_order[0],
                self.scale_by_order[1],
                self.scale_by_order[2],
                self.scale_by_order[3],
                self.scale_by_order[4],
            );
        }

        helpers::write_grid(&self.output, &grid0)
    }
}
