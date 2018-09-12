mod bcdms_array;

fn main() {
    println!("Hello, world!");

    let mut a = bcdms_array::BcdmsArray::new();
    for i in 0..1000 {
        a.push(i);
        println!("{}", a.read(i).unwrap());
    }

    for _ in 0..1000 {
        println!("{:?}", a.pop());
    }
}
