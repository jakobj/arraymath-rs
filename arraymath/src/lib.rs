macro_rules! binary_op_prototype {
    ($function_name: ident, $function_name_inplace: ident,
     $input_inner: ty, $output: ty) => {
        fn $function_name(&self, other: &[$input_inner]) -> $output;
        fn $function_name_inplace(&mut self, other: &[$input_inner]);
    };
}

macro_rules! binary_op {
    ($function_name: ident, $function_name_inplace: ident,
     $op: expr, $input_inner: ty, $output: ty) => {
        #[inline(always)]
        fn $function_name(&self, other: &[$input_inner]) -> $output {
            let mut res = self.clone();
            for i in 0..self.len() {
                $op(&mut res[i], other[i]);
            }
            res
        }

        #[inline(always)]
        fn $function_name_inplace(&mut self, other: &[$input_inner]) {
            for i in 0..self.len() {
                $op(&mut self[i], other[i]);
            }
        }
    };
}

pub trait ArrayMath {
    type InputInner;
    type Output;

    binary_op_prototype!(add, add_assign, Self::InputInner, Self::Output);
    binary_op_prototype!(sub, sub_assign, Self::InputInner, Self::Output);
}

impl<T, const N: usize> ArrayMath for [T; N]
where
    T: Copy + std::ops::AddAssign + std::ops::SubAssign,
{
    type InputInner = T;
    type Output = [T; N];

    binary_op!(
        add,
        add_assign,
        |lhs: &mut T, rhs: T| lhs.add_assign(rhs),
        Self::InputInner,
        Self::Output
    );
    binary_op!(
        sub,
        sub_assign,
        |lhs: &mut T, rhs: T| lhs.sub_assign(rhs),
        Self::InputInner,
        Self::Output
    );
}
