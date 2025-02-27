#!/bin/bash

wget --no-verbose --no-clobber -P test-data 'https://data.nnpdf.science/pineappl/test-data/ATLASWPT11-Wplus_tot.appl'
wget --no-verbose --no-clobber -P test-data 'https://data.nnpdf.science/pineappl/test-data/E906nlo_bin_00.pineappl.lz4'
wget --no-verbose --no-clobber -P test-data 'https://data.nnpdf.science/pineappl/test-data/E906nlo_bin_00.tar'
wget --no-verbose --no-clobber -P test-data 'https://data.nnpdf.science/pineappl/test-data/FK_ATLASTTBARTOT13TEV.dat'
wget --no-verbose --no-clobber -P test-data 'https://data.nnpdf.science/pineappl/test-data/FK_POSXDQ.dat'
wget --no-verbose --no-clobber -P test-data 'https://data.nnpdf.science/pineappl/test-data/LHCBWZMU7TEV_PI_part1.appl'
wget --no-verbose --no-clobber -P test-data 'https://data.nnpdf.science/pineappl/test-data/LHCB_DY_8TEV.pineappl.lz4'
wget --no-verbose --no-clobber -P test-data 'https://data.nnpdf.science/pineappl/test-data/LHCB_DY_8TEV.tar'
wget --no-verbose --no-clobber -P test-data 'https://data.nnpdf.science/pineappl/test-data/LHCB_WP_7TEV.pineappl.lz4'
wget --no-verbose --no-clobber -P test-data 'https://data.nnpdf.science/pineappl/test-data/LHCB_WP_7TEV.tar'
wget --no-verbose --no-clobber -P test-data 'https://data.nnpdf.science/pineappl/test-data/NJetEvents_0-0-2.tab.gz'
wget --no-verbose --no-clobber -P test-data 'https://data.nnpdf.science/pineappl/test-data/NUTEV_CC_NU_FE_SIGMARED.pineappl.lz4'
wget --no-verbose --no-clobber -P test-data 'https://data.nnpdf.science/pineappl/test-data/NUTEV_CC_NU_FE_SIGMARED.tar'
wget --no-verbose --no-clobber -P test-data 'https://ploughshare.web.cern.ch/ploughshare/db/applfast/applfast-atlas-dijets-fnlo-arxiv-1312.3524/grids/applfast-atlas-dijets-fnlo-arxiv-1312.3524-xsec000.tab.gz'
wget --no-verbose --no-clobber -P test-data 'https://ploughshare.web.cern.ch/ploughshare/db/applfast/applfast-h1-dijets-appl-arxiv-0010054/grids/applfast-h1-dijets-appl-arxiv-0010054-xsec000.appl'
wget --no-verbose --no-clobber -P test-data 'https://ploughshare.web.cern.ch/ploughshare/db/applfast/applfast-h1-incjets-fnlo-arxiv-0706.3722/grids/applfast-h1-incjets-fnlo-arxiv-0706.3722-xsec000.tab.gz'
wget --no-verbose --no-clobber -P test-data 'https://ploughshare.web.cern.ch/ploughshare/db/atlas/atlas-atlas-wpm-arxiv-1109.5141/grids/atlas-atlas-wpm-arxiv-1109.5141-xsec001.appl'

export RUSTFLAGS='-Cinstrument-coverage'

cargo clean
cargo test --all-features -- --include-ignored 2> >(tee stderr 1>&2)
# from https://stackoverflow.com/a/51141872/812178
sed -i 's/\x1B\[[0-9;]\{1,\}[A-Za-z]//g' stderr

find . -name '*.profraw' -exec $(rustc --print target-libdir)/../bin/llvm-profdata merge -sparse -o pineappl.profdata {} +
sed -nE 's/  Running( unittests|) [^[:space:]]+ \(([^)]+)\)/\2/p' stderr | \
    xargs printf ' --object %s' | \
    xargs $(rustc --print target-libdir)/../bin/llvm-cov show \
        --ignore-filename-regex='/.cargo/registry' \
        --ignore-filename-regex='rustc' \
        --ignore-filename-regex='pineappl/tests' \
        --ignore-filename-regex='pineappl_capi' \
        --ignore-filename-regex='pineappl_cli/tests' \
        --instr-profile=pineappl.profdata \
        --object target/debug/pineappl \
        --format html \
        --output-dir cov \
        -Xdemangler=rustfilt

rm pineappl.profdata
find . -name '*.profraw' -delete
