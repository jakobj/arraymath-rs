use arraymath::ArrayMath;

fn main() {
    let a = [1.2, 1.3, 1.4];
    let b = [1.3, 1.5, 2.4];
    println!("{:?}", a.add(&b));
    println!("{:?}", a.sub(&b));
    let mut a = [1.2, 1.3, 1.4];
    a.add_assign(&b);
    println!("{:?}", a);
}
