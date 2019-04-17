#![feature(test)]
extern crate test;

use test::{black_box, Bencher};

const BENCH_LEN: usize = 1_000_000;

#[bench]
fn index_bcdms(b: &mut Bencher) {
    let mut bcdms = bcdms_array::BcdmsArray::default();

    for i in 0..BENCH_LEN {
        bcdms.push(i);
    }

    b.iter(|| {
        for i in 0..BENCH_LEN {
            black_box(bcdms[black_box(i)]);
        }
    });
}

#[bench]
fn index_vec(b: &mut Bencher) {
    let mut vec = Vec::default();

    for i in 0..BENCH_LEN {
        vec.push(i);
    }

    b.iter(|| {
        for i in 0..BENCH_LEN {
            black_box(vec[black_box(i)]);
        }
    });
}
