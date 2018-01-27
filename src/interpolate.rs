use std::ops::{Index, Deref};
use buffer::audio::Sample;

const TWO_PI: f64 = 2. * ::std::f64::consts::PI;

pub enum Interpolation { Floor, Nearest, Linear, Poly, Spline }

pub struct Interpolator(Interpolation, Box<[Sample]>);

impl Interpolator {
    /// Create new function interpolator.
    ///
    /// `f` is called exactly `n` times to map each `x` in range [0, 2pi)
    /// to a [`Sample`]. later on [`f`], interpolation is done according to
    /// `int`.
    ///
    /// [`Sample`]: ../buffer/audio/type.Sample.html
    /// [`f`]: #method.f
    ///
    /// # Panics
    /// Panics if n == 0.
    pub fn new<F>(i: Interpolation, n: usize, f: F) -> Self
        where F: FnMut(f64) -> Sample {
        assert_ne!(n, 0);

        let known = (0..n).map(|i| i as f64 * TWO_PI / n as f64)
            .map(f).collect::<Vec<_>>().into_boxed_slice();

        Self { 0: i, 1: known }
    }

    /// Interpolate f(x).
    ///
    /// # Examples
    /// ```
    /// use std::f64::consts::FRAC_PI_4;
    /// use overcore::interpolate::{Interpolation, Interpolator};
    ///
    /// let interpolator = Interpolator::new(Interpolation::Linear, 4, |x| x.sin());
    ///
    /// assert_eq!(interpolator.f(0.), 0.);
    /// assert_eq!(interpolator.f(FRAC_PI_4), 0.5);
    /// ```
    pub fn f(&self, x: f64) -> Sample {
        let x = x * self.1.len() as f64 / TWO_PI;

        match self.0 {
            Interpolation::Floor => self[x.floor() as isize],
            Interpolation::Nearest => self[x.round() as isize],
            Interpolation::Linear => {
                let x_l = x.floor();
                let y_l = self[x_l as isize];
                let y_h = self[x_l as isize + 1];
                let (x_d, y_d) = (x - x_l, y_h - y_l);
                y_l + x_d * y_d
            },
            _ => unimplemented!()
        }
    }
}

impl Index<isize> for Interpolator {
    type Output = Sample;

    /// Get f(i) like f is a periodic function.
    fn index(&self, i: isize) -> &Sample {
        let (mut i, len) = (i, self.1.len() as isize);
        loop {
            if i >= len {
                i -= len;
            } else if i < 0 {
                i += len;
            } else {
                break &self.1[i as usize];
            }
        }
    }
}

impl Deref for Interpolator {
    type Target = Box<[Sample]>;

    fn deref(&self) -> &Self::Target { &self.1 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interpolator_new() {
        let int = Interpolator::new(Interpolation::Floor, 4, |x| x.sin());

        assert_eq!(int.len(), 4);

        assert_eq!(int[0], 0.);
        assert_eq!(int[1], 1.);
        assert!(int[2] - 0. < 1e-10);
        assert_eq!(int[3], -1.);
    }

    #[test]
    fn test_interpolator_index() {
        let int = Interpolator::new(Interpolation::Floor, 4, |x| x.sin());

        assert_eq!(int[1], 1.);
        assert_eq!(int[7], -1.);
        assert_eq!(int[-3], 1.);
        assert_eq!(int[-13], -1.);
    }

    #[test]
    fn test_interpolator_f_floor() {
        use std::f64::consts::FRAC_PI_2;
        let int = Interpolator::new(Interpolation::Floor, 4, |x| x.sin());

        assert_eq!(int.f(FRAC_PI_2), 1.);
        assert_eq!(int.f(FRAC_PI_2 + 1e-3), 1.);
        assert_eq!(int.f(FRAC_PI_2 - 1e-3), 0.);
    }

    #[test]
    fn test_interpolator_f_nearest() {
        use std::f64::consts::FRAC_PI_2;
        let int = Interpolator::new(Interpolation::Nearest, 4, |x| x.sin());

        assert_eq!(int.f(FRAC_PI_2), 1.);
        assert_eq!(int.f(FRAC_PI_2 / 2. + 1e-3), 1.);
        assert_eq!(int.f(FRAC_PI_2 / 2. - 1e-3), 0.);
    }

    #[test]
    fn test_interpolator_f_linear() {
        use std::f64::consts::FRAC_PI_4;
        let int = Interpolator::new(Interpolation::Linear, 4, |x| x.sin());

        assert_eq!(int.f(2. * FRAC_PI_4), 1.);
        assert_eq!(int.f(FRAC_PI_4), 0.5);
        assert_eq!(int.f(-FRAC_PI_4), -0.5);
        assert_eq!(int.f(5. * FRAC_PI_4), -0.5);
    }
}
