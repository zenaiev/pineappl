#ifndef APPLGRID_HPP
#define APPLGRID_HPP

#include "pineappl_applgrid/src/lib.rs.h"
#include "rust/cxx.h"

#include <appl_grid/appl_grid.h>
#include <appl_grid/lumi_pdf.h>
#include <appl_igrid.h>
#include <memory>

std::unique_ptr<appl::grid> make_grid(rust::Str filename);

rust::Vec<int> grid_combine(appl::grid const& grid);

rust::Vec<double> grid_convolute(
    appl::grid& grid,
    rust::Str pdfset,
    int member,
    int nloops,
    double rscale,
    double fscale,
    double escale
);

double sparse_matrix_get(SparseMatrix3d const& matrix, int x, int y, int z);

double weightfun(double x);

bool igrid_m_reweight(appl::igrid const& igrid);

#endif
