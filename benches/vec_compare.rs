#![feature(test)]
extern crate test;

use test::{black_box, Bencher};

use bcdms_array::BcdmsArray;
use lazy_static::lazy_static;
use rand::prelude::*;

const BENCH_LEN: usize = 1_000_000;

lazy_static! {
    static ref TEST_BCDMS : BcdmsArray<usize> = {
        let mut bcdms = BcdmsArray::default();

        for i in 0..BENCH_LEN {
            bcdms.push(i);
        }
        bcdms
    };

    static ref TEST_VEC : Vec<usize> = {
        let mut vec = Vec::default();

        for i in 0..BENCH_LEN {
            vec.push(i);
        }
        vec
    };

    static ref RANDOM_INDEXES : Vec<usize> = {
        // deterministic seed for reproducibility
        let mut rng = SmallRng::seed_from_u64(123456);

        let mut idxs = Vec::with_capacity(BENCH_LEN);

        for _ in 0..BENCH_LEN {
            idxs.push(rng.gen_range(0, BENCH_LEN));
        }
        idxs
    };
}

#[bench]
fn clone_bcdms(b: &mut Bencher) {
    let bcdms = &*TEST_BCDMS;
    b.iter(|| black_box(black_box(bcdms).clone()));
}

#[bench]
fn clone_vec(b: &mut Bencher) {
    let vec = &*TEST_VEC;
    b.iter(|| black_box(black_box(vec).clone()));
}

#[bench]
fn iter_bcdms(b: &mut Bencher) {
    let bcdms = TEST_BCDMS.clone();
    b.iter(|| {
        for i in bcdms.iter() {
            black_box(i);
        }
    });
}

#[bench]
fn iter_vec(b: &mut Bencher) {
    let vec = TEST_VEC.clone();
    b.iter(|| {
        for i in vec.iter() {
            black_box(i);
        }
    });
}

#[bench]
fn index_seq_bcdms(b: &mut Bencher) {
    let bcdms = TEST_BCDMS.clone();

    b.iter(|| {
        for i in 0..BENCH_LEN {
            black_box(bcdms[black_box(i)]);
        }
    });
}

#[bench]
fn index_seq_vec(b: &mut Bencher) {
    let vec = TEST_VEC.clone();

    b.iter(|| {
        for i in 0..BENCH_LEN {
            black_box(vec[black_box(i)]);
        }
    });
}

#[bench]
fn index_rand_bcdms(b: &mut Bencher) {
    let idxs = &*RANDOM_INDEXES;
    let bcdms = TEST_BCDMS.clone();
    b.iter(|| {
        for i in idxs {
            black_box(bcdms[black_box(*i)]);
        }
    });
}

#[bench]
fn index_rand_vec(b: &mut Bencher) {
    let idxs = &*RANDOM_INDEXES;
    let vec = TEST_VEC.clone();
    b.iter(|| {
        for i in idxs {
            black_box(vec[black_box(*i)]);
        }
    });
}
