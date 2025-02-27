use assert_cmd::Command;

const HELP_STR: &str = "Shows the predictions for all bin for each order separately

Usage: pineappl orders [OPTIONS] <INPUT> <PDFSET>

Arguments:
  <INPUT>   Path to the input grid
  <PDFSET>  LHAPDF id or name of the PDF set

Options:
  -a, --absolute               Show absolute numbers of each perturbative order
  -i, --integrated             Show integrated numbers (without bin widths) instead of differential ones
  -n, --normalize <NORMALIZE>  Normalize contributions to the specified orders
      --digits-abs <ABS>       Set the number of fractional digits shown for absolute numbers [default: 7]
      --digits-rel <REL>       Set the number of fractional digits shown for relative numbers [default: 2]
  -h, --help                   Print help information
";

const DEFAULT_STR: &str = "b   etal    disg/detal  O(as^0 a^2) O(as^1 a^2) O(as^0 a^3)
     []        [pb]         [%]         [%]         [%]    
-+----+----+-----------+-----------+-----------+-----------
0    2 2.25 3.7527620e2      100.00       16.73       -1.20
1 2.25  2.5 3.4521553e2      100.00       17.12       -1.10
2  2.5 2.75 3.0001406e2      100.00       17.52       -1.01
3 2.75    3 2.4257663e2      100.00       17.81       -0.90
4    3 3.25 1.8093343e2      100.00       18.36       -0.85
5 3.25  3.5 1.2291115e2      100.00       19.14       -0.77
6  3.5    4 5.7851018e1      100.00       20.49       -0.92
7    4  4.5 1.3772029e1      100.00       24.10       -0.98
";

const ABSOLUTE_STR: &str = "b   etal    disg/detal  O(as^0 a^2) O(as^1 a^2)  O(as^0 a^3) 
     []        [pb]        [pb]        [pb]         [pb]     
-+----+----+-----------+-----------+-----------+-------------
0    2 2.25 3.7527620e2 3.2482657e2 5.4355679e1  -3.9060418e0
1 2.25  2.5 3.4521553e2 2.9755128e2 5.0944018e1  -3.2797697e0
2  2.5 2.75 3.0001406e2 2.5751142e2 4.5111446e1  -2.6088069e0
3 2.75    3 2.4257663e2 2.0748091e2 3.6958317e1  -1.8626008e0
4    3 3.25 1.8093343e2 1.5397599e2 2.8268620e1  -1.3111794e0
5 3.25  3.5 1.2291115e2 1.0384063e2 1.9875123e1 -8.0459807e-1
6  3.5    4 5.7851018e1 4.8383606e1 9.9120372e0 -4.4462513e-1
7    4  4.5 1.3772029e1 1.1185365e1 2.6961509e0 -1.0948700e-1
";

const ABSOLUTE_INTEGRATED_STR: &str =
    "b   etal       integ    O(as^0 a^2) O(as^1 a^2)  O(as^0 a^3) 
     []         []          []          []           []      
-+----+----+-----------+-----------+-----------+-------------
0    2 2.25 9.3819050e1 8.1206641e1 1.3588920e1 -9.7651045e-1
1 2.25  2.5 8.6303882e1 7.4387820e1 1.2736004e1 -8.1994243e-1
2  2.5 2.75 7.5003515e1 6.4377855e1 1.1277861e1 -6.5220172e-1
3 2.75    3 6.0644157e1 5.1870228e1 9.2395791e0 -4.6565019e-1
4    3 3.25 4.5233358e1 3.8493998e1 7.0671550e0 -3.2779485e-1
5 3.25  3.5 3.0727788e1 2.5960157e1 4.9687809e0 -2.0114952e-1
6  3.5    4 2.8925509e1 2.4191803e1 4.9560186e0 -2.2231257e-1
7    4  4.5 6.8860146e0 5.5926826e0 1.3480754e0 -5.4743501e-2
";

const INTEGRATED_STR: &str = "b   etal       integ    O(as^0 a^2) O(as^1 a^2) O(as^0 a^3)
     []         []          [%]         [%]         [%]    
-+----+----+-----------+-----------+-----------+-----------
0    2 2.25 9.3819050e1      100.00       16.73       -1.20
1 2.25  2.5 8.6303882e1      100.00       17.12       -1.10
2  2.5 2.75 7.5003515e1      100.00       17.52       -1.01
3 2.75    3 6.0644157e1      100.00       17.81       -0.90
4    3 3.25 4.5233358e1      100.00       18.36       -0.85
5 3.25  3.5 3.0727788e1      100.00       19.14       -0.77
6  3.5    4 2.8925509e1      100.00       20.49       -0.92
7    4  4.5 6.8860146e0      100.00       24.10       -0.98
";

const NORMALIZE_A2_AS1A2_STR: &str = "b   etal    disg/detal  O(as^0 a^2) O(as^1 a^2) O(as^0 a^3)
     []        [pb]         [%]         [%]         [%]    
-+----+----+-----------+-----------+-----------+-----------
0    2 2.25 3.7527620e2       85.67       14.33       -1.03
1 2.25  2.5 3.4521553e2       85.38       14.62       -0.94
2  2.5 2.75 3.0001406e2       85.09       14.91       -0.86
3 2.75    3 2.4257663e2       84.88       15.12       -0.76
4    3 3.25 1.8093343e2       84.49       15.51       -0.72
5 3.25  3.5 1.2291115e2       83.93       16.07       -0.65
6  3.5    4 5.7851018e1       83.00       17.00       -0.76
7    4  4.5 1.3772029e1       80.58       19.42       -0.79
";

#[test]
fn help() {
    Command::cargo_bin("pineappl")
        .unwrap()
        .args(&["orders", "--help"])
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
            "orders",
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
            "orders",
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
            "orders",
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
fn integrated() {
    Command::cargo_bin("pineappl")
        .unwrap()
        .args(&[
            "--silence-lhapdf",
            "orders",
            "--integrated",
            "data/LHCB_WP_7TEV.pineappl.lz4",
            "NNPDF31_nlo_as_0118_luxqed",
        ])
        .assert()
        .success()
        .stdout(INTEGRATED_STR);
}

#[test]
fn normalize_a2_as1a2() {
    Command::cargo_bin("pineappl")
        .unwrap()
        .args(&[
            "--silence-lhapdf",
            "orders",
            "--normalize=a2,as1a2",
            "data/LHCB_WP_7TEV.pineappl.lz4",
            "NNPDF31_nlo_as_0118_luxqed",
        ])
        .assert()
        .success()
        .stdout(NORMALIZE_A2_AS1A2_STR);
}
