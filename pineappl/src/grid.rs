//! Module containing all traits and supporting structures for grids.

use super::bin::BinLimits;
use super::lumi::LumiEntry;
use super::ntuple_grid::NtupleSubgrid;
use ndarray::{Array3, Dimension};
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::mem;

/// Coupling powers for each grid.
#[derive(Clone, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Order {
    /// Exponent of the strong coupling.
    pub alphas: u32,
    /// Exponent of the electromagnetic coupling.
    pub alpha: u32,
    /// Exponent of the logarithm of the scale factor of the renomalization scale.
    pub logxir: u32,
    /// Exponent of the logarithm of the scale factor of the factorization scale.
    pub logxif: u32,
}

impl Order {
    /// Constructor. This function mainly exists to have a way of constructing `Order` that is less
    /// verbose.
    #[must_use]
    pub const fn new(alphas: u32, alpha: u32, logxir: u32, logxif: u32) -> Self {
        Self {
            alphas,
            alpha,
            logxir,
            logxif,
        }
    }

    /// Compares two vectors of `Order` for equality after sorting them.
    #[must_use]
    pub fn equal_after_sort(lhs: &[Self], rhs: &[Self]) -> bool {
        let mut lhs = lhs.to_vec();
        let mut rhs = rhs.to_vec();

        lhs.sort();
        rhs.sort();

        lhs == rhs
    }
}

/// Trait each subgrid must implement.
#[typetag::serde(tag = "type")]
pub trait Subgrid {
    /// Returns `self` as an `Any` type.
    fn as_any_mut(&mut self) -> &mut dyn Any;

    /// Convolute the subgrid with a luminosity function
    fn convolute(&self, lumi: &dyn Fn(f64, f64, f64) -> f64) -> f64;

    /// Fills the subgrid with `weight` for the parton momentum fractions `x1` and `x2`, and the
    /// scale `q2`.
    fn fill(&mut self, ntuple: &Ntuple<f64>);

    /// Returns true if `fill` was never called for this grid.
    fn is_empty(&self) -> bool;

    /// Merges `other` into this subgrid.
    fn merge(&mut self, other: &mut dyn Subgrid);

    /// Scale the subgrid by `factor`.
    fn scale(&mut self, factor: f64);
}

/// Subgrid creation parameters for subgrids that perform interpolation.
#[derive(Deserialize, Serialize)]
pub struct SubgridParams {
    q2_bins: usize,
    q2_max: f64,
    q2_min: f64,
    q2_order: usize,
    reweight: bool,
    x_bins: usize,
    x_max: f64,
    x_min: f64,
    x_order: usize,
}

impl Default for SubgridParams {
    fn default() -> Self {
        Self {
            q2_bins: 30,
            q2_max: 1000000.0,
            q2_min: 100.0,
            q2_order: 3,
            reweight: false,
            x_bins: 50,
            x_max: 1.0,
            x_min: 2e-7,
            x_order: 3,
        }
    }
}

impl SubgridParams {
    /// Returns the number of bins for the $Q^2$ axis.
    #[must_use]
    pub const fn q2_bins(&self) -> usize {
        self.q2_bins
    }

    /// Returns the upper limit of the $Q^2$ axis.
    #[must_use]
    pub const fn q2_max(&self) -> f64 {
        self.q2_max
    }

    /// Returns the lower limit of the $Q^2$ axis.
    #[must_use]
    pub const fn q2_min(&self) -> f64 {
        self.q2_min
    }

    /// Returns the interpolation order for the $Q^2$ axis.
    #[must_use]
    pub const fn q2_order(&self) -> usize {
        self.q2_order
    }

    /// Returns whether reweighting is enabled or not.
    #[must_use]
    pub const fn reweight(&self) -> bool {
        self.reweight
    }

    /// Sets the number of bins for the $Q^2$ axis.
    pub fn set_q2_bins(&mut self, q2_bins: usize) {
        self.q2_bins = q2_bins
    }

    /// Sets the upper limit of the $Q^2$ axis.
    pub fn set_q2_max(&mut self, q2_max: f64) {
        self.q2_max = q2_max
    }

    /// Sets the lower limit of the $Q^2$ axis.
    pub fn set_q2_min(&mut self, q2_min: f64) {
        self.q2_min = q2_min
    }

    /// Sets the interpolation order for the $Q^2$ axis.
    pub fn set_q2_order(&mut self, q2_order: usize) {
        self.q2_order = q2_order
    }

    /// Sets the reweighting parameter.
    pub fn set_reweight(&mut self, reweight: bool) {
        self.reweight = reweight
    }

    /// Sets the number of bins for the $x$ axes.
    pub fn set_x_bins(&mut self, x_bins: usize) {
        self.x_bins = x_bins
    }

    /// Sets the upper limit of the $x$ axes.
    pub fn set_x_max(&mut self, x_max: f64) {
        self.x_max = x_max
    }

    /// Sets the lower limit of the $x$ axes.
    pub fn set_x_min(&mut self, x_min: f64) {
        self.x_min = x_min
    }

    /// Sets the interpolation order for the $x$ axes.
    pub fn set_x_order(&mut self, x_order: usize) {
        self.x_order = x_order
    }

    /// Returns the number of bins for the $x$ axes.
    #[must_use]
    pub const fn x_bins(&self) -> usize {
        self.x_bins
    }

    /// Returns the upper limit of the $x$ axes.
    #[must_use]
    pub const fn x_max(&self) -> f64 {
        self.x_max
    }

    /// Returns the lower limit of the $x$ axes.
    #[must_use]
    pub const fn x_min(&self) -> f64 {
        self.x_min
    }

    /// Returns the interpolation order for the $x$ axes.
    #[must_use]
    pub const fn x_order(&self) -> usize {
        self.x_order
    }
}

/// This structure represents a position (`x1`, `x2`, `q2`) in a `Subgrid` together with a
/// corresponding `weight`. The type `W` can either be a `f64` or `()`, which is used when multiple
/// weights should be signaled.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Ntuple<W> {
    /// Momentum fraction of the first parton.
    pub x1: f64,
    /// Momentum fraction of the second parton.
    pub x2: f64,
    /// Squared scale.
    pub q2: f64,
    /// Weight of this entry.
    pub weight: W,
}

/// Error returned when merging two grids fails.
#[derive(Debug)]
pub struct GridMergeError {}

/// Main data structure of `PineAPPL`. This structure contains a `Subgrid` for each `LumiEntry`,
/// bin, and coupling order it was created with.
#[derive(Deserialize, Serialize)]
pub struct Grid {
    subgrids: Array3<Box<dyn Subgrid>>,
    lumi: Vec<LumiEntry>,
    bin_limits: BinLimits,
    orders: Vec<Order>,
    subgrid_params: SubgridParams,
}

impl Grid {
    /// Constructor.
    #[must_use]
    pub fn new(
        lumi: Vec<LumiEntry>,
        orders: Vec<Order>,
        bin_limits: Vec<f64>,
        subgrid_params: SubgridParams,
    ) -> Self {
        Self {
            subgrids: Array3::from_shape_simple_fn(
                (orders.len(), bin_limits.len() - 1, lumi.len()),
                || Box::new(NtupleSubgrid::default()) as Box<dyn Subgrid>,
            ),
            orders,
            lumi,
            bin_limits: BinLimits::new(bin_limits),
            subgrid_params,
        }
    }

    /// Returns the bin limits of the observables.
    #[must_use]
    pub const fn bin_limits(&self) -> &BinLimits {
        &self.bin_limits
    }

    /// Performs a convolution of the contained subgrids with the given PDFs, `xfx1` for the first
    /// parton and `xfx2` for the second parton, `alphas` for the evaluation of the strong
    /// coupling. The parameters `order_mask` and `lumi_mask` can be used to selectively enable
    /// perturbative orders and luminosities; they must either be empty (everything enabled) or as
    /// large as the orders and luminosity function, respectively. If the corresponding entry is
    /// `true` the order/luminosity is enable, `false` disables the entry. The tuple `xi` can be
    /// used to independently vary the renormalization (first element) and factorization scale
    /// (second element) from their central value `(1.0, 1.0)`.
    pub fn convolute(
        &self,
        xfx1: &dyn Fn(i32, f64, f64) -> f64,
        xfx2: &dyn Fn(i32, f64, f64) -> f64,
        alphas: &dyn Fn(f64) -> f64,
        order_mask: &[bool],
        lumi_mask: &[bool],
        xi: &(f64, f64),
    ) -> Vec<f64> {
        let mut bins: Vec<f64> = vec![0.0; self.bin_limits.bins()];

        for ((i, j, k), subgrid) in self.subgrids.indexed_iter() {
            let order = &self.orders[i];

            if (!order_mask.is_empty() && !order_mask[i])
                || ((order.logxir > 0) && (xi.0 == 1.0))
                || ((order.logxif > 0) && (xi.1 == 1.0))
                || (!lumi_mask.is_empty() && !lumi_mask[k])
            {
                continue;
            }

            let lumi_entry = &self.lumi[k];

            let mut value = subgrid.convolute(&|x1, x2, q2| {
                let mut lumi = 0.0;

                for entry in lumi_entry.entry() {
                    lumi += xfx1(entry.0, x1, q2) * xfx2(entry.1, x2, q2) * entry.2 / (x1 * x2);
                }

                lumi *= alphas(q2).powi(order.alphas as i32);
                lumi
            });

            if order.logxir > 0 {
                value *= xi.0.ln().powi(order.logxir as i32);
            }

            if order.logxif > 0 {
                value *= xi.1.ln().powi(order.logxif as i32);
            }

            bins[j] += value;
        }

        bins
    }

    /// Fills the grid with an ntuple for the given `order`, `observable`, and `lumi`.
    pub fn fill(&mut self, order: usize, observable: f64, lumi: usize, ntuple: &Ntuple<f64>) {
        if let Some(bin) = self.bin_limits.index(observable) {
            self.subgrids[[order, bin, lumi]].fill(ntuple);
        }
    }

    /// Fills the grid with events for the parton momentum fractions `x1` and `x2`, the scale `q2`,
    /// and the `order` and `observable`. The events are stored in `weights` and must be ordered as
    /// the corresponding luminosity function was created.
    pub fn fill_all(&mut self, order: usize, observable: f64, ntuple: &Ntuple<()>, weights: &[f64]) {
        for (lumi, weight) in weights.iter().enumerate() {
            self.fill(
                order,
                observable,
                lumi,
                &Ntuple {
                    x1: ntuple.x1,
                    x2: ntuple.x2,
                    q2: ntuple.q2,
                    weight: *weight,
                },
            );
        }
    }

    /// Returns the luminosity function.
    #[must_use]
    pub fn lumi(&self) -> &[LumiEntry] {
        &self.lumi
    }

    /// Merges the non-empty `Subgrid`s contained in `other` into `self`. This performs one of two
    /// possible operations:
    /// 1. If the bin limits of `self` and `other` are different and can be concatenated with each
    ///    other the bins are merged. In this case both grids are assumed to have the same orders
    ///    and the same luminosity functions. If this is not the case, an error is returned.
    /// 2. If the bin limits of `self` and `other` are the same, the luminosity functions and
    ///    perturbative orders of `self` and `other` may be different, if the ones that are the
    ///    same have empty grids in at least one of the grids. Otherwise an error is returned.
    pub fn merge(&mut self, mut other: Self) -> Result<(), GridMergeError> {
        if self.bin_limits == other.bin_limits {
            let mut new_orders: Vec<Order> = Vec::new();
            let mut new_entries: Vec<LumiEntry> = Vec::new();

            for ((i, _, k), _) in other
                .subgrids
                .indexed_iter_mut()
                .filter(|((_, _, _), subgrid)| !subgrid.is_empty())
            {
                let other_order = &other.orders[i];
                let other_entry = &other.lumi[k];

                if !self
                    .orders
                    .iter()
                    .chain(new_orders.iter())
                    .any(|x| x == other_order)
                {
                    new_orders.push(other_order.clone());
                }

                if !self
                    .lumi
                    .iter()
                    .chain(new_entries.iter())
                    .any(|y| y == other_entry)
                {
                    new_entries.push(other_entry.clone());
                }
            }

            if !new_orders.is_empty() || !new_entries.is_empty() {
                self.increase_shape(&(new_orders.len(), 0, new_entries.len()));
            }

            self.orders.append(&mut new_orders);
            self.lumi.append(&mut new_entries);
        } else {
            if !Order::equal_after_sort(&self.orders, &other.orders)
                || !LumiEntry::equal_after_sort(&self.lumi, &other.lumi)
            {
                return Err(GridMergeError {});
            }

            let new_bins = other.bin_limits.bins();

            if self.bin_limits.merge(other.bin_limits).is_err() {
                return Err(GridMergeError {});
            }

            self.increase_shape(&(0, new_bins, 0));
        }

        for ((i, j, k), subgrid) in other
            .subgrids
            .indexed_iter_mut()
            .filter(|((_, _, _), subgrid)| !subgrid.is_empty())
        {
            let other_order = &other.orders[i];
            let other_entry = &other.lumi[k];

            let self_i = self.orders.iter().position(|x| x == other_order).unwrap();
            let self_k = self.lumi.iter().position(|y| y == other_entry).unwrap();

            if self.subgrids[[self_i, j, self_k]].is_empty() {
                mem::swap(&mut self.subgrids[[self_i, j, self_k]], subgrid);
            } else {
                self.subgrids[[self_i, j, self_k]].merge(&mut **subgrid);
            }
        }

        Ok(())
    }

    fn increase_shape(&mut self, new_dim: &(usize, usize, usize)) {
        let old_dim = self.subgrids.raw_dim().into_pattern();
        let mut new_subgrids = Array3::from_shape_simple_fn(
            (
                old_dim.0 + new_dim.0,
                old_dim.1 + new_dim.1,
                old_dim.2 + new_dim.2,
            ),
            || Box::new(NtupleSubgrid::default()) as Box<dyn Subgrid>,
        );

        for ((i, j, k), subgrid) in self.subgrids.indexed_iter_mut() {
            mem::swap(&mut new_subgrids[[i, j, k]], subgrid);
        }

        mem::swap(&mut self.subgrids, &mut new_subgrids);
    }

    /// Scale all subgrids by `factor`.
    pub fn scale(&mut self, factor: f64) {
        self.subgrids
            .iter_mut()
            .for_each(|subgrid| subgrid.scale(factor));
    }

    /// Returns the subgrid parameters.
    #[must_use]
    pub fn orders(&self) -> &[Order] {
        &self.orders
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lumi_entry;

    #[test]
    fn grid_merge_empty_subgrids() {
        let mut grid = Grid::new(
            vec![
                lumi_entry![2, 2, 1.0; 4, 4, 1.0],
                lumi_entry![1, 1, 1.0; 3, 3, 1.0],
            ],
            vec![Order::new(0, 2, 0, 0)],
            vec![0.0, 0.25, 0.5, 0.75, 1.0],
            SubgridParams::default(),
        );

        assert_eq!(grid.bin_limits().bins(), 4);
        assert_eq!(grid.lumi().len(), 2);
        assert_eq!(grid.orders().len(), 1);

        let other = Grid::new(
            vec![
                // differently ordered than `grid`
                lumi_entry![1, 1, 1.0; 3, 3, 1.0],
                lumi_entry![2, 2, 1.0; 4, 4, 1.0],
            ],
            vec![Order::new(1, 2, 0, 0), Order::new(1, 2, 0, 1)],
            vec![0.0, 0.25, 0.5, 0.75, 1.0],
            SubgridParams::default(),
        );

        // merging with empty subgrids should not change the grid
        assert!(grid.merge(other).is_ok());

        assert_eq!(grid.bin_limits().bins(), 4);
        assert_eq!(grid.lumi().len(), 2);
        assert_eq!(grid.orders().len(), 1);
    }

    #[test]
    fn grid_merge_orders() {
        let mut grid = Grid::new(
            vec![
                lumi_entry![2, 2, 1.0; 4, 4, 1.0],
                lumi_entry![1, 1, 1.0; 3, 3, 1.0],
            ],
            vec![Order::new(0, 2, 0, 0)],
            vec![0.0, 0.25, 0.5, 0.75, 1.0],
            SubgridParams::default(),
        );

        assert_eq!(grid.bin_limits().bins(), 4);
        assert_eq!(grid.lumi().len(), 2);
        assert_eq!(grid.orders().len(), 1);

        let mut other = Grid::new(
            vec![
                lumi_entry![2, 2, 1.0; 4, 4, 1.0],
                lumi_entry![1, 1, 1.0; 3, 3, 1.0],
            ],
            vec![
                Order::new(1, 2, 0, 0),
                Order::new(1, 2, 0, 1),
                Order::new(0, 2, 0, 0),
            ],
            vec![0.0, 0.25, 0.5, 0.75, 1.0],
            SubgridParams::default(),
        );

        other.fill_all(
            0,
            0.1,
            &Ntuple {
                x1: 0.1,
                x2: 0.2,
                q2: 90.0_f64.powi(2),
                weight: (),
            },
            &[1.0, 2.0],
        );
        other.fill_all(
            1,
            0.1,
            &Ntuple {
                x1: 0.1,
                x2: 0.2,
                q2: 90.0_f64.powi(2),
                weight: (),
            },
            &[1.0, 2.0],
        );

        // merge with four non-empty subgrids
        assert!(grid.merge(other).is_ok());

        assert_eq!(grid.bin_limits().bins(), 4);
        assert_eq!(grid.lumi().len(), 2);
        assert_eq!(grid.orders().len(), 3);
    }

    #[test]
    fn grid_merge_lumi_entries() {
        let mut grid = Grid::new(
            vec![
                lumi_entry![2, 2, 1.0; 4, 4, 1.0],
                lumi_entry![1, 1, 1.0; 3, 3, 1.0],
            ],
            vec![Order::new(0, 2, 0, 0)],
            vec![0.0, 0.25, 0.5, 0.75, 1.0],
            SubgridParams::default(),
        );

        assert_eq!(grid.bin_limits().bins(), 4);
        assert_eq!(grid.lumi().len(), 2);
        assert_eq!(grid.orders().len(), 1);

        let mut other = Grid::new(
            vec![lumi_entry![22, 22, 1.0], lumi_entry![2, 2, 1.0; 4, 4, 1.0]],
            vec![Order::new(0, 2, 0, 0)],
            vec![0.0, 0.25, 0.5, 0.75, 1.0],
            SubgridParams::default(),
        );

        // fill the photon-photon entry
        other.fill(
            0,
            0.1,
            0,
            &Ntuple {
                x1: 0.1,
                x2: 0.2,
                q2: 90.0_f64.powi(2),
                weight: 3.0,
            },
        );

        assert!(grid.merge(other).is_ok());

        assert_eq!(grid.bin_limits().bins(), 4);
        assert_eq!(grid.lumi().len(), 3);
        assert_eq!(grid.orders().len(), 1);
    }

    #[test]
    fn grid_merge_bins() {
        let mut grid = Grid::new(
            vec![
                lumi_entry![2, 2, 1.0; 4, 4, 1.0],
                lumi_entry![1, 1, 1.0; 3, 3, 1.0],
            ],
            vec![Order::new(0, 2, 0, 0)],
            vec![0.0, 0.25, 0.5],
            SubgridParams::default(),
        );

        assert_eq!(grid.bin_limits().bins(), 2);
        assert_eq!(grid.lumi().len(), 2);
        assert_eq!(grid.orders().len(), 1);

        let mut other = Grid::new(
            vec![
                // luminosity function is differently sorted
                lumi_entry![1, 1, 1.0; 3, 3, 1.0],
                lumi_entry![2, 2, 1.0; 4, 4, 1.0],
            ],
            vec![Order::new(0, 2, 0, 0)],
            vec![0.5, 0.75, 1.0],
            SubgridParams::default(),
        );

        other.fill_all(
            0,
            0.1,
            &Ntuple {
                x1: 0.1,
                x2: 0.2,
                q2: 90.0_f64.powi(2),
                weight: (),
            },
            &[2.0, 3.0],
        );

        assert!(grid.merge(other).is_ok());

        assert_eq!(grid.bin_limits().bins(), 4);
        assert_eq!(grid.lumi().len(), 2);
        assert_eq!(grid.orders().len(), 1);
    }
}
