use arraymath::ScalarMath;
use arraymath::VectorMath;

fn main() {
    let a = [1.2, 1.3, 1.4];
    let b = [1.3, 1.5, 2.4];
    // add two arrays
    println!("{:?}", a.addv(&b));
    // substract two arrays
    println!("{:?}", a.subv(&b));
    let mut a = [1.2, 1.3, 1.4];
    // add two arrays inplace (modifies a)
    a.addv_assign(&b);
    println!("{:?}", a);
    // add an array and a scalar
    println!("{:?}", a.adds(2.0));
}
