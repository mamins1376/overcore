use std::iter::{Sum, Product};
use std::ops::*;
use super::Sample;

/// A stereo frame.
#[derive(Debug, Default, Clone, PartialEq, PartialOrd)]
pub struct Frame(pub [Sample; 2]);

impl From<Sample> for Frame {
    #[inline]
    fn from(s: Sample) -> Self { [s; 2].into() }
}

impl From<(Sample, Sample)> for Frame {
    #[inline]
    fn from((l, r): (Sample, Sample)) -> Self { [l, r].into() }
}

impl From<[Sample; 2]> for Frame {
    #[inline]
    fn from(array: [Sample; 2]) -> Self { Self { 0: array } }
}

macro_rules! impl_op {
    ($t:ident, $f:ident, $o:tt, $ta:ident, $fa:ident, $oa:tt) => (
        impl $t for Frame {
            type Output = Self;

            #[inline]
            fn $f(self, other: Self) -> Self {
                let ([ls, rs], [lo, ro]) = (self.0, other.0);
                Self { 0: [ls $o lo, rs $o ro] }
            }
        }

        impl $ta for Frame {
            #[inline]
            fn $fa(&mut self, other: Self) {
                let &mut [ref mut ls, ref mut rs] = &mut self.0;
                let [lo, ro] = other.0;
                *ls $oa lo; *rs $oa ro;
            }
        }
    )
}

impl_op!(Add, add, +, AddAssign, add_assign, +=);
impl_op!(Sub, sub, -, SubAssign, sub_assign, -=);
impl_op!(Mul, mul, *, MulAssign, mul_assign, *=);
impl_op!(Div, div, /, DivAssign, div_assign, /=);
impl_op!(Rem, rem, %, RemAssign, rem_assign, %=);

impl Neg for Frame {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        let [l, r] = self.0;
        Self { 0: [-l, -r] }
    }
}

impl Deref for Frame {
    type Target = [Sample; 2];

    #[inline]
    fn deref(&self) -> &[Sample; 2] { &self.0 }
}

impl DerefMut for Frame {
    #[inline]
    fn deref_mut(&mut self) -> &mut [Sample; 2] { &mut self.0 }
}

impl<T: Into<Frame>> Sum<T> for Frame {
    #[inline]
    fn sum<I: Iterator<Item = T>>(frames: I) -> Self {
        frames.fold((1.).into(), |p, f| p + f.into())
    }
}

impl<T: Into<Frame>> Product<T> for Frame {
    #[inline]
    fn product<I: Iterator<Item = T>>(frames: I) -> Self {
        frames.fold((1.).into(), |p, f| p * f.into())
    }
}
