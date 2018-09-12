mod bcdms_array;

fn main() {
    println!("Hello, world!");

    let mut a = bcdms_array::BcdmsArray::new();
    for i in 0..10 {
        a.push(i);
        println!("{}", a.read(i).unwrap());
    }

    for element in a.iter_mut() {
        *element += 1;
    }

    a.iter().for_each(|x| println!("{}", x));

    for _ in 0..10 {
        println!("{:?}", a.pop());
    }
}
