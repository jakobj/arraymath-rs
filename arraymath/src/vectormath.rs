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

pub trait VectorMath {
    type InputInner;
    type Output;

    binary_op_vector_prototype!(addv, Self::InputInner, Self::Output);
    binary_op_vector_inplace_prototype!(addv_assign, Self::InputInner, Self::Output);
    binary_op_vector_prototype!(subv, Self::InputInner, Self::Output);
    binary_op_vector_inplace_prototype!(subv_assign, Self::InputInner, Self::Output);
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
}
