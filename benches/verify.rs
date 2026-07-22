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

mod numeric {
    use super::*;
    pub static SAMPLES: LazyLock<Vec<String>> = LazyLock::new(|| gen_samples(b"0123456789"));
}

mod alphabetic {
    use super::*;
    pub static SAMPLES: LazyLock<Vec<String>> =
        LazyLock::new(|| gen_samples(b"ABCDEFGHIJKLMNOPQRSTUVWXYZ"));
}

mod alphanumeric {
    use super::*;
    pub static SAMPLES: LazyLock<Vec<String>> =
        LazyLock::new(|| gen_samples(b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ"));
}

macro_rules! gen_benches {
    ($system:ident, $charset:ident, $type_x:ident, $type_y:ident) => {
        mod $system {
            use super::*;
            use iso_iec_7064::System as _;

            #[test]
            fn compare_results() {
                let samples = &*$charset::SAMPLES;
                for s in samples {
                    let x = iso_7064::$type_x.verify(s).unwrap_or(false);
                    let y = iso_iec_7064::$type_y.validate_string(s);
                    assert_eq!(x, y);
                }
            }

            #[bench]
            fn bench_iso_7064(b: &mut test::Bencher) {
                let samples = &*$charset::SAMPLES;
                b.iter(|| {
                    samples.iter().fold(false, |acc, s| {
                        acc ^ iso_7064::$type_x.verify(s).unwrap_or(false)
                    })
                });
            }

            #[bench]
            fn bench_iso_7064_over_bytes(b: &mut test::Bencher) {
                let samples = &*$charset::SAMPLES;
                b.iter(|| {
                    samples.iter().fold(false, |acc, s| {
                        acc ^ iso_7064::$type_x
                            .verify_from_chars(s.bytes().map(char::from))
                            .unwrap_or(false)
                    })
                });
            }

            #[bench]
            fn bench_iso_iec_7064(b: &mut test::Bencher) {
                let samples = &*$charset::SAMPLES;
                b.iter(|| {
                    samples.iter().fold(false, |acc, s| {
                        acc ^ iso_iec_7064::$type_y.validate_string(s)
                    })
                });
            }
        }
    };
}

gen_benches!(pure1_mod11_2, numeric, MOD11_2, MOD_11_2);
gen_benches!(pure1_mod37_2, alphanumeric, MOD37_2, MOD_37_2);
gen_benches!(pure2_mod97_10, numeric, MOD97_10, MOD_97_10);
gen_benches!(pure2_mod661_26, alphabetic, MOD661_26, MOD_661_26);
gen_benches!(pure2_mod1271_36, alphanumeric, MOD1271_36, MOD_1271_36);
gen_benches!(hybrid_mod11_10, numeric, MOD11_10, MOD_11_10);
gen_benches!(hybrid_mod27_26, alphabetic, MOD27_26, MOD_27_26);
gen_benches!(hybrid_mod37_36, alphanumeric, MOD37_36, MOD_37_36);
