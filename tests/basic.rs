use rand::prelude::*;

#[test]
fn push_iter_pop() {
    let mut a = bcdms_array::BcdmsArray::default();

    for i in 0..10 {
        a.push(i);
        assert_eq!(a[i], i);
    }

    for element in a.iter_mut() {
        *element += 1;
    }

    a.iter()
        .enumerate()
        .for_each(|(i, x)| assert_eq!(*x, i + 1));

    for i in 0..10 {
        assert_eq!(a.pop().unwrap(), 10 - i);
    }

    assert_eq!(a.pop(), None);
}

#[test]
fn push_pop_vec_parity() {
    // We push and pop a lot of random numbers
    // and assert that our behavior is identical with Vec
    let mut bcdms = bcdms_array::BcdmsArray::default();
    let mut vec = Vec::default();
    let mut rng = thread_rng();

    for _ in 0..10000 {
        if rng.gen() {
            for _ in 0..rng.gen_range(0, 1000) {
                let n = rng.gen::<u64>();
                bcdms.push(n);
                vec.push(n);
            }
        } else {
            for _ in 0..rng.gen_range(0, 1000) {
                assert_eq!(vec.len(), bcdms.len());
                assert_eq!(vec.pop(), bcdms.pop());
            }
        }
    }

    // test random access
    for _ in 0..10000 {
        let idx = rng.gen_range(0, bcdms.len());
        assert_eq!(bcdms[idx], vec[idx]);
    }
}

#[test]
fn insert_front() {
    let mut bcdms = bcdms_array::BcdmsArray::default();
    let mut vec = Vec::default();

    for idx in 0..10000 {
        bcdms.insert(0, idx);
        vec.insert(0, idx);
    }

    
    assert_eq!(bcdms2vec(bcdms), vec);
}

#[test]
fn remove_front() {
    let mut bcdms = bcdms_array::BcdmsArray::default();
    for i in 0..1000 {
        bcdms.push(i);
    }
    for i in 0..1000 {
        assert_eq!(bcdms.remove(0), i);
        bcdms.simple_sanity_check();
    }
}

#[test]
fn rand_remove() {
    let mut bcdms = bcdms_array::BcdmsArray::default();
    let mut vec = Vec::default();
    let mut rng = thread_rng();

    for i in 0..10 {
        bcdms.push(i);
        vec.push(i);
    }
    for i in 0..10 {
        let idx = rng.gen_range(0, bcdms.len());
        assert_eq!(bcdms.remove(idx), vec.remove(idx), "in step {}", i);
        println!("{:?}", bcdms);
        bcdms.simple_sanity_check();
    }
}

#[test]
fn insert_back() {
    let mut bcdms = bcdms_array::BcdmsArray::default();
    let mut vec = Vec::default();

    for i in 0..1000 {
        vec.insert(i, i);
        bcdms.insert(i, i);
    }
}

#[test]
fn rand_insert() {
    let mut bcdms = bcdms_array::BcdmsArray::default();
    let mut vec = Vec::default();
    let mut rng = thread_rng();

    for i in 0..1000 {
        let idx = rng.gen_range(0, vec.len() + 1);
        vec.insert(idx, i);
        bcdms.insert(idx, i);
    }

    bcdms.simple_sanity_check();
}

#[test]
fn rand_insert_remove() {
    let mut bcdms = bcdms_array::BcdmsArray::default();
    let mut vec = Vec::default();
    let mut rng = thread_rng();

    bcdms.push(0);
    vec.push(0);
    for i in 1..1000 {
        let idx = rng.gen_range(0, vec.len());
        vec.insert(idx, i);
        bcdms.insert(idx, i);
    }

    for i in 0..1000 {
        if vec.is_empty() {
            bcdms.push(i);
            vec.push(i);
            continue;
        }

        let idx = rng.gen_range(0, vec.len());
        if rng.gen() {
            let r1 = bcdms.remove(idx);
            let r2 = vec.remove(idx);
            assert_eq!(r1, r2, "Index: {}; BCDMS {:?}; VEC {:?}", idx, bcdms2vec(bcdms), vec);
        } else {
            bcdms.insert(idx, i);
            vec.insert(idx, i);
        }
    }

    assert_eq!(bcdms2vec(bcdms), vec);
}

fn bcdms2vec<T: Copy>(bcdms: bcdms_array::BcdmsArray<T>) -> Vec<T> {
    let mut result = Vec::new();
    for x in bcdms.iter() {
        result.push(*x);
    }
    result
}
