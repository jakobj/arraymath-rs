use arraymath::ScalarMath;
use arraymath::VectorMath;

fn main() {
    let a = [1.2, 1.3, 1.4];
    let b = [1.3, 1.5, 2.4];
    println!("{:?}", a.addv(&b));
    println!("{:?}", a.subv(&b));
    let mut a = [1.2, 1.3, 1.4];
    a.addv_assign(&b);
    println!("{:?}", a);
    println!("{:?}", a.adds(2.0));
}
