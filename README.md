# arraymath-rs

Math on arrays. Fast and simple.

This crate aims to simplify using math operations on arrays. It does so by providing a trait which implements common operations (addition, substraction, ...) on plain arrays, allowing you to write code like this:

``` rust
use arraymath::ScalarMath;
use arraymath::VectorMath;

fn main() {
    let a = [1.2, 1.3, 1.4];
    let b = [1.3, 1.5, 2.4];
    // add two arrays
    println!("{:?}", a.addv(&b)); // -> [2.5, 2.8, 3.8]
    // substract two arrays
    println!("{:?}", a.subv(&b)); // -> [-0.10000000000000009, -0.19999999999999996, -1.0]

    let mut a = [1.2, 1.3, 1.4];
    // add two arrays inplace (modifies a)
    a.addv_assign(&b); // -> [2.5, 2.8, 3.8]
    println!("{:?}", a);
    // add an array and a scalar
    println!("{:?}", a.adds(2.0)); // -> [4.5, 4.8, 5.8]

    let a = [1.2, 1.3, 1.4];
    let mut b = [99.0, 99.0, 99.0];
    // add an array and a scalar and store result in existing array (modifies b)
    a.adds_into(2.0, &mut b);
    println!("{:?}", b); // -> [3.2, 3.3, 3.4]
}
```

 We aim for speed at least as fast as element-wise for loops, i.e., zero-cost abstractions, and simple usage with minimal impact on existing code.
