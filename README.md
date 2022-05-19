[![Rust](https://github.com/N3PDF/pineappl/workflows/Rust/badge.svg)](https://github.com/N3PDF/pineappl/actions?query=workflow%3ARust)
[![codecov](https://codecov.io/gh/N3PDF/pineappl/branch/master/graph/badge.svg)](https://codecov.io/gh/N3PDF/pineappl)
[![Documentation](https://docs.rs/pineappl/badge.svg)](https://docs.rs/pineappl)
[![crates.io](https://img.shields.io/crates/v/pineappl.svg)](https://crates.io/crates/pineappl)
[![Minimum cargo version](https://img.shields.io/badge/cargo-1.54+-lightgray.svg)](https://github.com/N3PDF/pineappl#installation)

# Introduction

This repository contains libraries, tools, and interfaces to read and write
`PineAPPL` interpolation grids.

Similar projects are:

- [APPLgrid](https://applgrid.hepforge.org/) and
- [fastNLO](https://fastnlo.hepforge.org/).

This repository hosts four main crates:

- [`pineappl`](https://crates.io/crates/pineappl) is the crate containing the
  main functionality,
- [`pineappl_capi`](https://crates.io/crates/pineappl_capi/) installs a library
  and a C header, to use PineAPPL from your C, C++ or Fortran programs,
- [`pineappl_cli`](https://crates.io/crates/pineappl_cli/) installs the program
  `pineappl` to use PineAPPL from the command line and
- [`pineappl_py`](https://pypi.org/project/pineappl/) is the Python interface.

# Documentation

A good starting point to learn what PineAPPL and its command-line program
`pineappl` can do is the [tutorial](docs/cli-tutorial.md)!

For the CLI also a partial reference [reference](docs/cli-reference.md) is
available.

## API documentation:

- [C](https://docs.rs/pineappl_capi/latest/pineappl_capi/)
- for Fortran there's no dedicated documentation available, because it's a
  [wrapper](examples/fortran/pineappl.f90) of the C API
- [Python](https://pineappl.readthedocs.io/en/latest/modules/pineappl/pineappl.html)
- [Rust](https://docs.rs/pineappl/latest/pineappl/)

## Code examples

Another way to learn using the APIs is to have a look/modify the
[examples](examples/).

# Installation

[![Anaconda-Server Badge](https://anaconda.org/conda-forge/pineappl/badges/installer/conda.svg)](https://anaconda.org/conda-forge/pineappl)
[![AUR](https://img.shields.io/aur/version/pineappl)](https://aur.archlinux.org/packages/pineappl)

`PineAPPL` is written in [`Rust`](https://www.rust-lang.org/) and therefore
needs the Rust compiler and its build system `cargo`. If `cargo` is already
installed, make sure it is recent enough:

    cargo --version

This should show a version that 1.54 or newer. If you do not have `cargo` or it
is too old, go to <https://www.rust-lang.org/tools/install> and follow the
instructions there.

Next, install the command-line interface (CLI) by choosing either the *release*
or *development version* below. In both cases the binary `pineappl` will be
installed user-wide, typically into `~/.cargo/bin`. You can use this binary to
perform all kinds of operations on PineAPPL grids.

For most users the release version is recommended, as we guarantee that all
grids generated with release versions will be supported in all future release
versions (backwards compatibility guarantee). The advantage of the development
version is that it typically supports more features.

## Release version (recommended)

Simply run

    cargo install pineappl_cli

anywhere and you are done; this will automatically download the most-recently
released version from [crates.io](https://crates.io).

## Development version (alternative)

To use the most recent version available run

    cargo install --git https://github.com/N3PDF/pineappl.git

Instead, if you plan to make changes to the source code it's better to checkout
this repository and run

    cargo install --path pineappl_cli

inside it.

## Optional: fastNLO converter

If you'd like to convert fastNLO tables to PineAPPL, make sure to install
[fastNLO](https://fastnlo.hepforge.org/) first and add the switch
`--features=fastnlo` during the CLI's installation, for instance for the
development version:

    cargo install --features=fastnlo --path pineappl_cli

## Optional: C interface

If you plan to use one of the supported Monte Carlo programs to *generate*
PineAPPL grids, or if you want to access the contents of grids from your own
program, you will likely need the C interface (unless you are using Python, see
below). In that case proceed by installing

- `cargo-c`, which is required for the next step:

      cargo install cargo-c

  It is possible that the installation fails if your Rust compiler is too old.
  In that case update Rust or try installing an older version of `cargo-c`:

      cargo install cargo-c --version 0.7.3

- Now install `pineappl_capi`, PineAPPL's C API:

      cd pineappl_capi
      cargo cinstall --release --prefix=${prefix}
      cd ..

  where `${prefix}` points to the desired installation directory.

- Finally, you need to set the environment variables `PKG_CONFIG_PATH` and
  `LD_LIBRARY_PATH` to the right directories. Adding

      export LD_LIBRARY_PATH=${prefix}/lib:$LD_LIBRARY_PATH
      export PKG_CONFIG_PATH=${prefix}/lib/pkgconfig:$PKG_CONFIG_PATH

  to your `~/.bashrc` should do the trick (remember to replace `${prefix}` with
  the correct directory). You can check `PKG_CONFIG_PATH` by running

      pkg-config pineappl_capi --libs

  which should print the library flags needed to link against the C API. If
  there's no output or an error, double-check that `PKG_CONFIG_PATH` is in the
  environment and that it points to a directory containing the
  `pineappl_capi.pc` file.

## Optional: Python interface

[![PyPI version](https://badge.fury.io/py/pineappl.svg)](https://badge.fury.io/py/pineappl)
[![Anaconda-Server Badge](https://anaconda.org/conda-forge/pineappl/badges/installer/conda.svg)](https://anaconda.org/conda-forge/pineappl)
[![AUR](https://img.shields.io/aur/version/pineappl)](https://aur.archlinux.org/packages/pineappl)

To install the Python interface, run

    pip install pineappl

For more documentation and more information see its
[README](pineappl_py/README.md).

# Contributions

Before submitting a pull request please read the
[contribution guidelines](CONTRIBUTING.md).

# Citation

[![arXiv](https://img.shields.io/badge/arXiv-2008.12789-b31b1b?labelColor=222222)](https://arxiv.org/abs/2008.12789)
[![DOI](https://zenodo.org/badge/248306479.svg)](https://zenodo.org/badge/latestdoi/248306479)

If you use PineAPPL, please cite

1) the zenodo DOI above and
2) the following reference:

   ```
   @article{Carrazza:2020gss,
       author = "Carrazza, S. and Nocera, E. R. and Schwan, C. and Zaro, M.",
       title = "{PineAPPL: combining EW and QCD corrections for fast evaluation of LHC processes}",
       eprint = "2008.12789",
       archivePrefix = "arXiv",
       primaryClass = "hep-ph",
       doi = "10.1007/JHEP12(2020)108",
       journal = "JHEP",
       volume = "12",
       pages = "108",
       year = "2020"
   }
   ```
