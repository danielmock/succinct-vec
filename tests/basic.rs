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
    // TODO: make this pass
    for _ in 0..10000 {
        let idx = rng.gen_range(0, bcdms.len());
        assert_eq!(bcdms[idx], vec[idx]);
    }
}
