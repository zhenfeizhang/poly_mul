//! polynomials over integers
use std::cmp::{max, min};
use std::ops::*;
use zeroize::Zeroize;

#[derive(Clone, Zeroize, PartialEq, Debug)]
pub struct Poly<T: Sized + Add + Sub + Rem + Mul> {
    pub(crate) degree: usize,
    pub(crate) coeff: Vec<T>,
}

macro_rules! poly_z_add_impl {
    ($($t:ty)*) => ($(
        impl Add for Poly<$t> {
            type Output = Self;

            #[inline]
            fn add(self, other: Self) -> Self::Output {

                let hi = max(self.degree, other.degree);
                let lo = min(self.degree, other.degree);
                let mut res: Vec<$t> = self.coeff.iter().zip(other.coeff.iter()).map(|(x, y)| x+y).collect();
                if self.degree == hi {
                    res = [res, self.coeff[lo..hi].to_vec()].concat();
                } else {
                    res = [res, other.coeff[lo..hi].to_vec()].concat();
                }

                Self {
                    degree: self.degree,
                    coeff: res,
                } }
        }

    )*)
}

poly_z_add_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64 }
