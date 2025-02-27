use assert_cmd::Command;

const HELP_STR: &str = "Shows the contribution for each partonic channel

Usage: pineappl channels [OPTIONS] <INPUT> <PDFSET>

Arguments:
  <INPUT>   Path to the input grid
  <PDFSET>  LHAPDF id or name of the PDF set

Options:
  -a, --absolute          Show absolute numbers of each contribution
  -l, --limit <LIMIT>     The maximum number of channels displayed [default: 10]
  -i, --integrated        Show integrated numbers (without bin widths) instead of differential ones
      --lumis <LUMIS>     Show only the listed channels
  -o, --orders <ORDERS>   Select orders manually
      --digits-abs <ABS>  Set the number of fractional digits shown for absolute numbers [default: 7]
      --digits-rel <REL>  Set the number of fractional digits shown for relative numbers [default: 2]
  -h, --help              Print help information
";

const DEFAULT_STR: &str = "b   etal    l  size  l  size  l size  l size l size
     []        [%]      [%]      [%]    [%]    [%] 
-+----+----+-+------+-+------+-+-----+-+----+-+----
0    2 2.25 0 111.42 3  -8.10 1 -3.35 4 0.02 2 0.01
1 2.25  2.5 0 112.20 3  -8.85 1 -3.38 4 0.02 2 0.01
2  2.5 2.75 0 113.09 3  -9.59 1 -3.51 4 0.01 2 0.01
3 2.75    3 0 113.99 3 -10.16 1 -3.85 4 0.01 2 0.01
4    3 3.25 0 114.81 3 -10.58 1 -4.25 4 0.01 2 0.01
5 3.25  3.5 0 115.57 3 -10.79 1 -4.80 2 0.02 4 0.01
6  3.5    4 0 116.24 3 -10.48 1 -5.78 4 0.01 2 0.01
7    4  4.5 0 115.79 3  -8.59 1 -7.23 4 0.03 2 0.01
";

const ABSOLUTE_STR: &str =
    "b   etal    l disg/detal  l  disg/detal  l  disg/detal   l  disg/detal  l  disg/detal 
     []          [pb]           [pb]           [pb]            [pb]           [pb]    
-+----+----+-+-----------+-+------------+-+-------------+-+------------+-+------------
0    2 2.25 0 4.1812079e2 3 -3.0404615e1 1  -1.2554096e1 4 7.3283017e-2 2 4.0843648e-2
1 2.25  2.5 0 3.8732921e2 3 -3.0544941e1 1  -1.1664958e1 4 7.1405161e-2 2 2.4809894e-2
2  2.5 2.75 0 3.3927446e2 3 -2.8777182e1 1  -1.0542665e1 4 3.9504114e-2 2 1.9941837e-2
3 2.75    3 0 2.7651168e2 3 -2.4643233e1 1  -9.3423343e0 4 3.6066338e-2 2 1.4446490e-2
4    3 3.25 0 2.0772858e2 3 -1.9135032e1 1  -7.6928937e0 4 2.2005096e-2 2 1.0776479e-2
5 3.25  3.5 0 1.4204593e2 3 -1.3267050e1 1  -5.9020586e0 2 2.1802965e-2 4 1.2525547e-2
6  3.5    4 0 6.7246107e1 3 -6.0617011e0 1  -3.3444315e0 4 8.0948775e-3 2 2.9487321e-3
7    4  4.5 0 1.5946839e1 3 -1.1836652e0 1 -9.9587535e-1 4 3.9519852e-3 2 7.7841269e-4
";

const ABSOLUTE_INTEGRATED_STR: &str =
    "b   etal    l    integ    l     integ     l     integ     l    integ     l    integ    
     []           []             []              []              []             []     
-+----+----+-+-----------+-+-------------+-+-------------+-+------------+-+------------
0    2 2.25 0 1.0453020e2 3  -7.6011537e0 1  -3.1385240e0 4 1.8320754e-2 2 1.0210912e-2
1 2.25  2.5 0 9.6832303e1 3  -7.6362352e0 1  -2.9162394e0 4 1.7851290e-2 2 6.2024736e-3
2  2.5 2.75 0 8.4818615e1 3  -7.1942956e0 1  -2.6356663e0 4 9.8760284e-3 2 4.9854592e-3
3 2.75    3 0 6.9127920e1 3  -6.1608082e0 1  -2.3355836e0 4 9.0165844e-3 2 3.6116225e-3
4    3 3.25 0 5.1932144e1 3  -4.7837579e0 1  -1.9232234e0 4 5.5012740e-3 2 2.6941197e-3
5 3.25  3.5 0 3.5511483e1 3  -3.3167626e0 1  -1.4755146e0 2 5.4507411e-3 4 3.1313867e-3
6  3.5    4 0 3.3623053e1 3  -3.0308506e0 1  -1.6722157e0 4 4.0474387e-3 2 1.4743661e-3
7    4  4.5 0 7.9734197e0 3 -5.9183261e-1 1 -4.9793767e-1 4 1.9759926e-3 2 3.8920635e-4
";

const LIMIT_3_STR: &str = "b   etal    l  size  l  size  l size 
     []        [%]      [%]      [%] 
-+----+----+-+------+-+------+-+-----
0    2 2.25 0 111.42 3  -8.10 1 -3.35
1 2.25  2.5 0 112.20 3  -8.85 1 -3.38
2  2.5 2.75 0 113.09 3  -9.59 1 -3.51
3 2.75    3 0 113.99 3 -10.16 1 -3.85
4    3 3.25 0 114.81 3 -10.58 1 -4.25
5 3.25  3.5 0 115.57 3 -10.79 1 -4.80
6  3.5    4 0 116.24 3 -10.48 1 -5.78
7    4  4.5 0 115.79 3  -8.59 1 -7.23
";

const BAD_LIMIT_STR: &str = "error: Invalid value '0' for '--limit <LIMIT>': 0 is not in 1..=65535

For more information try '--help'
";

const LUMIS_0123_STR: &str = "b   etal    l  size  l  size  l size  l size
     []        [%]      [%]      [%]    [%] 
-+----+----+-+------+-+------+-+-----+-+----
0    2 2.25 0 111.42 3  -8.10 1 -3.35 2 0.01
1 2.25  2.5 0 112.20 3  -8.85 1 -3.38 2 0.01
2  2.5 2.75 0 113.09 3  -9.59 1 -3.51 2 0.01
3 2.75    3 0 113.99 3 -10.16 1 -3.85 2 0.01
4    3 3.25 0 114.81 3 -10.58 1 -4.25 2 0.01
5 3.25  3.5 0 115.57 3 -10.79 1 -4.80 2 0.02
6  3.5    4 0 116.24 3 -10.48 1 -5.78 2 0.01
7    4  4.5 0 115.79 3  -8.59 1 -7.23 2 0.01
";

const ORDERS_A2_AS1A2_STR: &str = "b   etal    l  size  l  size  l size  l size l size
     []        [%]      [%]      [%]    [%]    [%] 
-+----+----+-+------+-+------+-+-----+-+----+-+----
0    2 2.25 0 111.33 3  -8.02 1 -3.31 2 0.00 4 0.00
1 2.25  2.5 0 112.11 3  -8.76 1 -3.35 2 0.00 4 0.00
2  2.5 2.75 0 112.99 3  -9.51 1 -3.48 2 0.00 4 0.00
3 2.75    3 0 113.90 3 -10.08 1 -3.82 2 0.00 4 0.00
4    3 3.25 0 114.72 3 -10.50 1 -4.22 2 0.00 4 0.00
5 3.25  3.5 0 115.49 3 -10.72 1 -4.77 2 0.00 4 0.00
6  3.5    4 0 116.14 3 -10.40 1 -5.74 2 0.00 4 0.00
7    4  4.5 0 115.70 3  -8.53 1 -7.17 2 0.00 4 0.00
";

#[test]
fn help() {
    Command::cargo_bin("pineappl")
        .unwrap()
        .args(&["channels", "--help"])
        .assert()
        .success()
        .stdout(HELP_STR);
}

#[test]
fn default() {
    Command::cargo_bin("pineappl")
        .unwrap()
        .args(&[
            "--silence-lhapdf",
            "channels",
            "data/LHCB_WP_7TEV.pineappl.lz4",
            "NNPDF31_nlo_as_0118_luxqed",
        ])
        .assert()
        .success()
        .stdout(DEFAULT_STR);
}

#[test]
fn absolute() {
    Command::cargo_bin("pineappl")
        .unwrap()
        .args(&[
            "--silence-lhapdf",
            "channels",
            "--absolute",
            "data/LHCB_WP_7TEV.pineappl.lz4",
            "NNPDF31_nlo_as_0118_luxqed",
        ])
        .assert()
        .success()
        .stdout(ABSOLUTE_STR);
}

#[test]
fn absolute_integrated() {
    Command::cargo_bin("pineappl")
        .unwrap()
        .args(&[
            "--silence-lhapdf",
            "channels",
            "--absolute",
            "--integrated",
            "data/LHCB_WP_7TEV.pineappl.lz4",
            "NNPDF31_nlo_as_0118_luxqed",
        ])
        .assert()
        .success()
        .stdout(ABSOLUTE_INTEGRATED_STR);
}

#[test]
fn limit_3() {
    Command::cargo_bin("pineappl")
        .unwrap()
        .args(&[
            "--silence-lhapdf",
            "channels",
            "--limit=3",
            "data/LHCB_WP_7TEV.pineappl.lz4",
            "NNPDF31_nlo_as_0118_luxqed",
        ])
        .assert()
        .success()
        .stdout(LIMIT_3_STR);
}

#[test]
fn bad_limit() {
    Command::cargo_bin("pineappl")
        .unwrap()
        .args(&[
            "--silence-lhapdf",
            "channels",
            "--limit=0",
            "data/LHCB_WP_7TEV.pineappl.lz4",
            "NNPDF31_nlo_as_0118_luxqed",
        ])
        .assert()
        .failure()
        .stderr(BAD_LIMIT_STR);
}

#[test]
fn lumis_0123() {
    Command::cargo_bin("pineappl")
        .unwrap()
        .args(&[
            "--silence-lhapdf",
            "channels",
            "--lumis=0-3",
            "data/LHCB_WP_7TEV.pineappl.lz4",
            "NNPDF31_nlo_as_0118_luxqed",
        ])
        .assert()
        .success()
        .stdout(LUMIS_0123_STR);
}

#[test]
fn orders_a2_as1a2() {
    Command::cargo_bin("pineappl")
        .unwrap()
        .args(&[
            "--silence-lhapdf",
            "channels",
            "--orders=a2,as1a2",
            "data/LHCB_WP_7TEV.pineappl.lz4",
            "NNPDF31_nlo_as_0118_luxqed",
        ])
        .assert()
        .success()
        .stdout(ORDERS_A2_AS1A2_STR);
}
