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
pub trait ScalarMath {
    type Input;
    type Output;

    binary_op_scalar_prototype!(adds, Self::Input, Self::Output);
    binary_op_scalar_inplace_prototype!(adds_assign, Self::Input, Self::Output);
    binary_op_scalar_prototype!(subs, Self::Input, Self::Output);
    binary_op_scalar_inplace_prototype!(subs_assign, Self::Input, Self::Output);
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
    binary_op_scalar!(subs, |lhs: T, rhs: T| lhs - rhs, Self::Input, Self::Output);
    binary_op_scalar_inplace!(
        subs_assign,
        |lhs: &mut T, rhs: T| lhs.sub_assign(rhs),
        Self::Input,
        Self::Output
    );
}
