macro_rules! binary_op_scalar_prototype {
    ($function_name: ident, $input: ty, $output: ty) => {
        fn $function_name(&self, other: $input) -> $output;
    };
}

macro_rules! binary_op_scalar_inplace_prototype {
    ($function_name: ident, $input: ty, $output: ty) => {
        fn $function_name(&mut self, other: $input);
    };
}

macro_rules! binary_op_scalar_into_prototype {
    ($function_name: ident, $input: ty, $into: ty) => {
        fn $function_name(&self, other: $input, into: &mut $into);
    };
}

macro_rules! binary_op_scalar {
    ($function_name: ident, $op: expr, $input: ty, $output: ty) => {
        #[inline(always)]
        fn $function_name(&self, other: $input) -> $output {
            let mut res: [T; N] = [self[0]; N];
            for i in 0..self.len() {
                res[i] = $op(self[i], other);
            }
            res
        }
    };
}

macro_rules! binary_op_scalar_inplace {
    ($function_name: ident, $op: expr, $input: ty, $output: ty) => {
        #[inline(always)]
        fn $function_name(&mut self, other: $input) {
            for i in 0..self.len() {
                $op(&mut self[i], other);
            }
        }
    };
}

macro_rules! binary_op_scalar_into {
    ($function_name: ident, $op: expr, $input: ty, $into: ty) => {
        #[inline(always)]
        fn $function_name(&self, other: $input, into: &mut $into) {
            for i in 0..self.len() {
                into[i] = $op(self[i], other);
            }
        }
    };
}

pub trait ScalarMath {
    type Input;
    type Output;

    binary_op_scalar_prototype!(adds, Self::Input, Self::Output);
    binary_op_scalar_inplace_prototype!(adds_assign, Self::Input, Self::Output);
    binary_op_scalar_into_prototype!(adds_into, Self::Input, Self::Output);
    binary_op_scalar_prototype!(subs, Self::Input, Self::Output);
    binary_op_scalar_inplace_prototype!(subs_assign, Self::Input, Self::Output);
    binary_op_scalar_into_prototype!(subs_into, Self::Input, Self::Output);
}

impl<T, const N: usize> ScalarMath for [T; N]
where
    T: Copy
        + std::ops::Add<Output = T>
        + std::ops::AddAssign
        + std::ops::Sub<Output = T>
        + std::ops::SubAssign,
{
    type Input = T;
    type Output = [T; N];

    binary_op_scalar!(adds, |lhs: T, rhs: T| lhs + rhs, Self::Input, Self::Output);
    binary_op_scalar_inplace!(
        adds_assign,
        |lhs: &mut T, rhs: T| lhs.add_assign(rhs),
        Self::Input,
        Self::Output
    );
    binary_op_scalar_into!(
        adds_into,
        |lhs: T, rhs: T| lhs + rhs,
        Self::Input,
        Self::Output
    );
    binary_op_scalar!(subs, |lhs: T, rhs: T| lhs - rhs, Self::Input, Self::Output);
    binary_op_scalar_inplace!(
        subs_assign,
        |lhs: &mut T, rhs: T| lhs.sub_assign(rhs),
        Self::Input,
        Self::Output
    );
    binary_op_scalar_into!(
        subs_into,
        |lhs: T, rhs: T| lhs - rhs,
        Self::Input,
        Self::Output
    );
}

#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;

    use super::*;

    #[test]
    fn test_adds() {
        let a = [1.2, 1.3, 1.4];
        let b = 2.1;
        let c = a.adds(b);
        let c_expected: [f64; 3] = [3.3, 3.4, 3.5];
        for i in 0..c.len() {
            assert_approx_eq!(c[i], c_expected[i]);
        }
    }

    #[test]
    fn test_adds_assign() {
        let mut a = [1.2, 1.3, 1.4];
        let b = 2.1;
        a.adds_assign(b);
        let a_expected: [f64; 3] = [3.3, 3.4, 3.5];
        for i in 0..a.len() {
            assert_approx_eq!(a[i], a_expected[i]);
        }
    }

    #[test]
    fn test_adds_into() {
        let a = [1.2, 1.3, 1.4];
        let b = 2.1;
        let mut c = [99.0, 99.0, 99.0];
        a.adds_into(b, &mut c);
        let c_expected: [f64; 3] = [3.3, 3.4, 3.5];
        for i in 0..a.len() {
            assert_approx_eq!(c[i], c_expected[i]);
        }
    }

    #[test]
    fn test_subs() {
        let a = [1.2, 1.3, 1.4];
        let b = 2.1;
        let c = a.subs(b);
        let c_expected: [f64; 3] = [-0.9, -0.8, -0.7];
        for i in 0..c.len() {
            assert_approx_eq!(c[i], c_expected[i]);
        }
    }

    #[test]
    fn test_subs_assign() {
        let mut a = [1.2, 1.3, 1.4];
        let b = 2.1;
        a.subs_assign(b);
        let a_expected: [f64; 3] = [-0.9, -0.8, -0.7];
        for i in 0..a.len() {
            assert_approx_eq!(a[i], a_expected[i]);
        }
    }

    #[test]
    fn test_subs_into() {
        let a = [1.2, 1.3, 1.4];
        let b = 2.1;
        let mut c = [99.0, 99.0, 99.0];
        a.subs_into(b, &mut c);
        let c_expected: [f64; 3] = [-0.9, -0.8, -0.7];
        for i in 0..a.len() {
            assert_approx_eq!(c[i], c_expected[i]);
        }
    }
}
