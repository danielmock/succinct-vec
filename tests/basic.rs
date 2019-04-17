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
