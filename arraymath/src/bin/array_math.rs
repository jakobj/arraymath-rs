trait ArrayOps {
    type InputInner;
    type Output;

    fn add(&self, other: &[Self::InputInner]) -> Self::Output;
    fn add_assign(&mut self, other: &[Self::InputInner]);
    fn sub(&self, other: &[Self::InputInner]) -> Self::Output;
}

impl<T, const N: usize> ArrayOps for [T; N]
where
    T: Copy + std::ops::AddAssign + std::ops::SubAssign,
{
    type InputInner = T;
    type Output = [T; N];

    fn add(&self, other: &[Self::InputInner]) -> Self::Output {
        let mut res = self.clone();
        for i in 0..self.len() {
            res[i] += other[i];
        }
        res
    }

    fn add_assign(&mut self, other: &[Self::InputInner]) {
        for i in 0..self.len() {
            self[i] += other[i];
        }
    }

    fn sub(&self, other: &[Self::InputInner]) -> Self::Output {
        let mut res = self.clone();
        for i in 0..self.len() {
            res[i] -= other[i];
        }
        res
    }
}

fn main() {
    let mut a = [1.2, 1.3, 1.4];
    let b = [1.3, 1.5, 2.4];
    println!("{:?}", a.add(&b));
    println!("{:?}", a.sub(&b));
    a.add_assign(&b);
    println!("{:?}", a);
}
