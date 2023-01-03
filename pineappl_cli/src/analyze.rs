use super::helpers::{self, ConvoluteMode, Subcommand};
use anyhow::Result;
use clap::{Parser, ValueHint};
use prettytable::{cell, Row};
use std::path::PathBuf;

/// Perform various analyses with grids.
#[derive(Parser)]
pub struct Opts {
    #[clap(subcommand)]
    subcommand: SubcommandEnum,
}

impl Subcommand for Opts {
    fn run(&self) -> Result<u8> {
        self.subcommand.run()
    }
}

#[derive(Parser)]
enum SubcommandEnum {
    Ckf(CkfOpts),
}

impl Subcommand for SubcommandEnum {
    fn run(&self) -> Result<u8> {
        match self {
            Self::Ckf(opts) => opts.run(),
        }
    }
}

/// Compare K-factors with channel K factors (ckf).
#[derive(Parser)]
pub struct CkfOpts {
    /// Path to the input grid.
    #[clap(value_parser, value_hint = ValueHint::FilePath)]
    input: PathBuf,
    /// LHAPDF id or name of the PDF set.
    #[clap(value_parser = helpers::parse_pdfset)]
    pdfset: String,
    /// Order defining the K factors.
    #[clap(value_parser = helpers::parse_order)]
    order: (u32, u32),
    /// Normalizing orders of the K factors.
    #[clap(
        require_value_delimiter = true,
        use_value_delimiter = true,
        value_parser = helpers::parse_order
    )]
    orders_den: Vec<(u32, u32)>,
    /// The maximum number of channels displayed.
    #[clap(
        default_value = "10",
        long,
        short,
        value_parser = helpers::parse_pos_non_zero::<usize>
    )]
    limit: usize,
    /// Set the number of fractional digits shown for relative numbers.
    #[clap(default_value_t = 2, long = "digits-rel", value_name = "REL")]
    digits_rel: usize,
    /// Forces negative PDF values to zero.
    #[clap(long = "force-positive")]
    force_positive: bool,
}

impl Subcommand for CkfOpts {
    fn run(&self) -> Result<u8> {
        let grid = helpers::read_grid(&self.input)?;
        let mut pdf = helpers::create_pdf(&self.pdfset)?;

        let orders_den = if self.orders_den.is_empty() {
            grid.orders()
                .iter()
                .filter_map(|order| {
                    ((order.alphas != self.order.0) && (order.alpha != self.order.1))
                        .then_some((order.alphas, order.alpha))
                })
                .collect()
        } else {
            self.orders_den.clone()
        };

        let limit = grid.lumi().len().min(self.limit);
        let limits = helpers::convolute_limits(&grid, &[], ConvoluteMode::Normal);
        let results: Vec<_> = (0..grid.lumi().len())
            .map(|lumi| {
                let mut lumi_mask = vec![false; grid.lumi().len()];
                lumi_mask[lumi] = true;
                helpers::convolute(
                    &grid,
                    &mut pdf,
                    &[self.order],
                    &[],
                    &lumi_mask,
                    1,
                    ConvoluteMode::Normal,
                    self.force_positive,
                )
            })
            .collect();
        let results_den: Vec<_> = (0..grid.lumi().len())
            .map(|lumi| {
                let mut lumi_mask = vec![false; grid.lumi().len()];
                lumi_mask[lumi] = true;
                helpers::convolute(
                    &grid,
                    &mut pdf,
                    &orders_den,
                    &[],
                    &lumi_mask,
                    1,
                    ConvoluteMode::Normal,
                    self.force_positive,
                )
            })
            .collect();

        let (x, _, _) = helpers::labels_and_units(&grid, false);
        let mut title = Row::empty();
        title.add_cell(cell!(c->"b"));
        for (x_label, x_unit) in x {
            let mut cell = cell!(c->format!("{x_label}\n[{x_unit}]"));
            cell.set_hspan(2);
            title.add_cell(cell);
        }
        title.add_cell(cell!(c->"bin-K"));
        for _ in 0..limit {
            title.add_cell(cell!(c->"l"));
            title.add_cell(cell!(c->"K"));
        }

        let mut table = helpers::create_table();
        table.set_titles(title);

        for (bin, limits) in limits.iter().enumerate() {
            let row = table.add_empty_row();

            row.add_cell(cell!(r->format!("{bin}")));

            for (left, right) in limits {
                row.add_cell(cell!(r->format!("{left}")));
                row.add_cell(cell!(r->format!("{right}")));
            }

            let mut values: Vec<_> = results
                .iter()
                .zip(results_den.iter())
                .enumerate()
                .map(|(lumi, (vec, vec_den))| (lumi, vec[bin], vec_den[bin]))
                .collect();

            // sort using the absolute value in descending order
            values.sort_unstable_by(|(_, left, left_den), (_, right, right_den)| {
                (right + right_den)
                    .abs()
                    .partial_cmp(&(left + left_den).abs())
                    .unwrap()
            });

            let (total, total_den) = values
                .iter()
                .fold((0.0, 0.0), |(nom, den), (_, add_nom, add_den)| {
                    (nom + add_nom, den + add_den)
                });

            row.add_cell(
                cell!(r->format!("{:.*}", self.digits_rel, (total + total_den) / total_den)),
            );

            for (lumi, value, value_den) in values.into_iter().take(limit) {
                row.add_cell(cell!(r->format!("{lumi}")));

                let channel_k = if value_den == 0.0 {
                    if value == 0.0 {
                        0.0
                    } else {
                        f64::INFINITY.copysign(value)
                    }
                } else {
                    (value + value_den) / value_den
                };

                row.add_cell(cell!(r->format!("{:.*}", self.digits_rel, channel_k)));
            }
        }

        table.printstd();

        Ok(0)
    }
}
