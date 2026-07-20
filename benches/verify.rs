#![feature(test)]

extern crate test;

use std::sync::LazyLock;

fn gen_samples(charset: &[u8]) -> Vec<String> {
    use rand::RngExt as _;
    let mut rng = rand::rng();

    let mut xs = Vec::new();
    for _ in 0..8 {
        let len = rng.random_range(8..32);
        for _ in 0..64 {
            let mut buf = String::with_capacity(len);
            for _ in 0..len {
                let i = rng.random_range(0..charset.len());
                buf.push(char::from(charset[i]));
            }
            xs.push(buf);
        }
    }
    xs
}

static NUMERIC: LazyLock<Vec<String>> = LazyLock::new(|| gen_samples(b"0123456789"));
static ALPHABETIC: LazyLock<Vec<String>> =
    LazyLock::new(|| gen_samples(b"ABCDEFGHIJKLMNOPQRSTUVWXYZ"));
static ALPHANUMERIC: LazyLock<Vec<String>> =
    LazyLock::new(|| gen_samples(b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ"));

macro_rules! gen_benches {
    ($system:ident, $charset:ident, $type_x:ident, $type_y:ident) => {
        mod $system {
            use super::*;
            use iso_iec_7064::System as _;

            #[test]
            fn compare_results() {
                let list = &*$charset;
                for s in list {
                    let x = iso_7064::$type_x.verify(s).unwrap_or(false);
                    let y = iso_iec_7064::$type_y.validate_string(s);
                    assert_eq!(x, y);
                }
            }

            #[bench]
            fn bench_iso_7064(b: &mut test::Bencher) {
                let list = &*$charset;
                b.iter(|| {
                    list.iter().fold(false, |acc, s| {
                        acc ^ iso_7064::$type_x.verify(s).unwrap_or(false)
                    })
                });
            }

            #[bench]
            fn bench_iso_iec_7064(b: &mut test::Bencher) {
                let list = &*$charset;
                b.iter(|| {
                    list.iter().fold(false, |acc, s| {
                        acc ^ iso_iec_7064::$type_y.validate_string(s)
                    })
                });
            }
        }
    };
}

gen_benches!(mod11_2, NUMERIC, MOD11_2, MOD_11_2);
gen_benches!(mod37_2, ALPHANUMERIC, MOD37_2, MOD_37_2);
gen_benches!(mod97_10, NUMERIC, MOD97_10, MOD_97_10);
gen_benches!(mod661_26, ALPHABETIC, MOD661_26, MOD_661_26);
gen_benches!(mod1271_36, ALPHANUMERIC, MOD1271_36, MOD_1271_36);
gen_benches!(mod11_10, NUMERIC, MOD11_10, MOD_11_10);
gen_benches!(mod27_26, ALPHABETIC, MOD27_26, MOD_27_26);
gen_benches!(mod37_36, ALPHANUMERIC, MOD37_36, MOD_37_36);
