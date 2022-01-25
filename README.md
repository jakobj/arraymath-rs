# arraymath-rs

Math on arrays. Fast and simple.

This crate aims to simplify using math operations on arrays. It does so by providing a trait which implements common operations (addition, substraction, ...) on plain arrays, allowing you to write code like this:

``` rust
use arraymath::ArrayMath;

fn main() {
    let a = [1.2, 1.3, 1.4];
    let b = [1.3, 1.5, 2.4];
    println!("{:?}", a.add(&b)); // -> [2.5, 2.8, 3.8]
    println!("{:?}", a.sub(&b)); // -> [-0.10000000000000009, -0.19999999999999996, -1.0]
    let mut a = [1.2, 1.3, 1.4];
    a.add_assign(&b);
    println!("{:?}", a); // -> [2.5, 2.8, 3.8]
}
```

 We aim for speed at least as fast as element-wise for loops, i.e., zero-cost abstractions, and simple usage with minimal impact on existing code.
