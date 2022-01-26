macro_rules! binary_op_vector_prototype {
    ($function_name: ident, $input_inner: ty, $output: ty) => {
        fn $function_name(&self, other: &[$input_inner]) -> $output;
    };
}

macro_rules! binary_op_vector_inplace_prototype {
    ($function_name: ident, $input_inner: ty, $output: ty) => {
        fn $function_name(&mut self, other: &[$input_inner]);
    };
}

macro_rules! binary_op_vector_into_prototype {
    ($function_name: ident, $input: ty, $into: ty) => {
        fn $function_name(&self, other: &[$input], into: &mut $into);
    };
}

macro_rules! binary_op_vector {
    ($function_name: ident, $op: expr, $input_inner: ty, $output: ty) => {
        #[inline(always)]
        fn $function_name(&self, other: &[$input_inner]) -> $output {
            let mut res: [T; N] = [self[0]; N];
            for i in 0..self.len() {
                res[i] = $op(self[i], other[i]);
            }
            res
        }
    };
}

macro_rules! binary_op_vector_inplace {
    ($function_name: ident, $op: expr, $input_inner: ty, $output: ty) => {
        #[inline(always)]
        fn $function_name(&mut self, other: &[$input_inner]) {
            for i in 0..self.len() {
                $op(&mut self[i], other[i]);
            }
        }
    };
}

macro_rules! binary_op_vector_into {
    ($function_name: ident, $op: expr, $input: ty, $into: ty) => {
        #[inline(always)]
        fn $function_name(&self, other: &[$input], into: &mut $into) {
            for i in 0..self.len() {
                into[i] = $op(self[i], other[i]);
            }
        }
    };
}

pub trait VectorMath {
    type InputInner;
    type Output;

    binary_op_vector_prototype!(addv, Self::InputInner, Self::Output);
    binary_op_vector_inplace_prototype!(addv_assign, Self::InputInner, Self::Output);
    binary_op_vector_into_prototype!(addv_into, Self::InputInner, Self::Output);
    binary_op_vector_prototype!(subv, Self::InputInner, Self::Output);
    binary_op_vector_inplace_prototype!(subv_assign, Self::InputInner, Self::Output);
    binary_op_vector_into_prototype!(subv_into, Self::InputInner, Self::Output);
}

impl<T, const N: usize> VectorMath for [T; N]
where
    T: Copy
        + std::ops::Add<Output = T>
        + std::ops::AddAssign
        + std::ops::Sub<Output = T>
        + std::ops::SubAssign,
{
    type InputInner = T;
    type Output = [T; N];

    binary_op_vector!(
        addv,
        |lhs: T, rhs: T| lhs + rhs,
        Self::InputInner,
        Self::Output
    );
    binary_op_vector_inplace!(
        addv_assign,
        |lhs: &mut T, rhs: T| lhs.add_assign(rhs),
        Self::InputInner,
        Self::Output
    );
    binary_op_vector_into!(
        addv_into,
        |lhs: T, rhs: T| lhs + rhs,
        Self::InputInner,
        Self::Output
    );
    binary_op_vector!(
        subv,
        |lhs: T, rhs: T| lhs - rhs,
        Self::InputInner,
        Self::Output
    );
    binary_op_vector_inplace!(
        subv_assign,
        |lhs: &mut T, rhs: T| lhs.sub_assign(rhs),
        Self::InputInner,
        Self::Output
    );
    binary_op_vector_into!(
        subv_into,
        |lhs: T, rhs: T| lhs - rhs,
        Self::InputInner,
        Self::Output
    );
}

#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;

    use super::*;

    #[test]
    fn test_addv() {
        let a = [1.2, 1.3, 1.4];
        let b = [1.3, 1.5, 2.4];
        let c = a.addv(&b);
        let c_expected: [f64; 3] = [2.5, 2.8, 3.8];
        for i in 0..c.len() {
            assert_approx_eq!(c[i], c_expected[i]);
        }
    }

    #[test]
    fn test_addv_assign() {
        let mut a = [1.2, 1.3, 1.4];
        let b = [1.3, 1.5, 2.4];
        a.addv_assign(&b);
        let a_expected: [f64; 3] = [2.5, 2.8, 3.8];
        for i in 0..a.len() {
            assert_approx_eq!(a[i], a_expected[i]);
        }
    }

    #[test]
    fn test_addv_into() {
        let a = [1.2, 1.3, 1.4];
        let b = [1.3, 1.5, 2.4];
        let mut c = [99.0, 99.0, 99.0];
        a.addv_into(&b, &mut c);
        let c_expected: [f64; 3] = [2.5, 2.8, 3.8];
        for i in 0..c.len() {
            assert_approx_eq!(c[i], c_expected[i]);
        }
    }

    #[test]
    fn test_subv() {
        let a = [1.2, 1.3, 1.4];
        let b = [1.3, 1.5, 2.4];
        let c = a.subv(&b);
        let c_expected: [f64; 3] = [-0.1, -0.2, -1.0];
        for i in 0..c.len() {
            assert_approx_eq!(c[i], c_expected[i]);
        }
    }

    #[test]
    fn test_subv_assign() {
        let mut a = [1.2, 1.3, 1.4];
        let b = [1.3, 1.5, 2.4];
        a.subv_assign(&b);
        let a_expected: [f64; 3] = [-0.1, -0.2, -1.0];
        for i in 0..a.len() {
            assert_approx_eq!(a[i], a_expected[i]);
        }
    }

    #[test]
    fn test_subv_into() {
        let a = [1.2, 1.3, 1.4];
        let b = [1.3, 1.5, 2.4];
        let mut c = [99.0, 99.0, 99.0];
        a.subv_into(&b, &mut c);
        let c_expected: [f64; 3] = [-0.1, -0.2, -1.0];
        for i in 0..c.len() {
            assert_approx_eq!(c[i], c_expected[i]);
        }
    }
}
