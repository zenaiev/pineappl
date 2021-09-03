use super::subgrid::PySubgridEnum;
use numpy::PyReadonlyArray3;
use pineappl::import_only_subgrid::ImportOnlySubgridV1;
use pineappl::sparse_array3::SparseArray3;
use pyo3::prelude::*;

/// PyO3 wrapper to [`pineappl::import_only_subgrid::ImportOnlySubgridV1`]
///
/// **Usage**: `yadism`
#[pyclass]
#[derive(Clone)]
#[repr(transparent)]
pub struct PyImportOnlySubgridV1 {
    pub(crate) import_only_subgrid: ImportOnlySubgridV1,
}

impl PyImportOnlySubgridV1 {
    pub(crate) fn new(import_only_subgrid: ImportOnlySubgridV1) -> Self {
        Self {
            import_only_subgrid,
        }
    }
}

#[pymethods]
impl PyImportOnlySubgridV1 {
    #[new]
    pub fn new_import_only_subgrid(
        array: PyReadonlyArray3<f64>,
        q2_grid: Vec<f64>,
        x1_grid: Vec<f64>,
        x2_grid: Vec<f64>,
    ) -> Self {
        let mut sparse_array = SparseArray3::new(q2_grid.len(), x1_grid.len(), x2_grid.len());

        for ((iq2, ix1, ix2), value) in array
            .as_array()
            .indexed_iter()
            .filter(|((_, _, _), value)| **value != 0.0)
        {
            sparse_array[[iq2, ix1, ix2]] = *value;
        }

        Self::new(ImportOnlySubgridV1::new(
            sparse_array,
            q2_grid,
            x1_grid,
            x2_grid,
        ))
    }

    /// Wrapper to match [`PyGrid::set_subgrid()`]
    pub fn into(&self) -> PySubgridEnum {
        PySubgridEnum {
            subgrid_enum: self.import_only_subgrid.clone().into(),
        }
    }
}
