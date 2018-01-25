use std::convert::TryFrom;
use ::buffer::prelude::Frame;
use ::hardconf::TWO_POW_ONE_TWELFTH;

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
/// Alphabets of a [`NoteName`] (e.g. E in "E3").
pub enum NoteAlphabet { C, Cs, D, Ds, E, F, Fs, G, Gs, A, As, B }

impl NoteAlphabet {
    #[inline]
    /// Get number of possible alphabets.
    pub fn len() -> usize { 12 }

    #[inline]
    /// Get index of this variant.
    ///
    /// # Examples
    /// ```
    /// use overlib::buffer::control::NoteAlphabet;
    /// assert_eq!(NoteAlphabet::C.index(), 0);
    /// assert_eq!(NoteAlphabet::A.index(), 9);
    /// ```
    pub fn index(&self) -> usize {
        match self {
            &NoteAlphabet::C  => 0,
            &NoteAlphabet::Cs => 1,
            &NoteAlphabet::D  => 2,
            &NoteAlphabet::Ds => 3,
            &NoteAlphabet::E  => 4,
            &NoteAlphabet::F  => 5,
            &NoteAlphabet::Fs => 6,
            &NoteAlphabet::G  => 7,
            &NoteAlphabet::Gs => 8,
            &NoteAlphabet::A  => 9,
            &NoteAlphabet::As => 10,
            &NoteAlphabet::B  => 11,
        }
    }

    #[inline]
    /// Get equivalent frequency of a note in octave 4 with alphabet `self`.
    ///
    /// # Examples
    /// ```
    /// use overlib::buffer::control::NoteAlphabet;
    /// assert_eq!(NoteAlphabet::A.freq(), 440.);
    /// ```
    pub fn freq(&self) -> f64 { self.detune(0.) }

    #[inline]
    /// Detune frequency by `cents`.
    ///
    /// # Examples
    /// ```
    /// use overlib::buffer::control::NoteAlphabet;
    /// assert_eq!(NoteAlphabet::A.detune(0.), NoteAlphabet::A.freq());
    /// assert_eq!(NoteAlphabet::A.detune(100.), NoteAlphabet::As.freq());
    /// assert_eq!(NoteAlphabet::A.detune(-900.), NoteAlphabet::C.freq());
    /// ```
    pub fn detune<T: Into<f64>>(&self, cents: T) -> f64 {
        let mut half_steps = cents.into() / 100.;
        half_steps += self.index() as f64 - NoteAlphabet::A.index() as f64;
        440. * TWO_POW_ONE_TWELFTH.powf(half_steps)
    }

    #[inline]
    /// Transpose a new note from self, `n` half steps higher (or lower).
    ///
    /// # Examples
    /// ```
    /// use overlib::buffer::control::NoteAlphabet;
    /// assert_eq!(NoteAlphabet::A.transpose(0), NoteAlphabet::A);
    /// assert_eq!(NoteAlphabet::A.transpose(5), NoteAlphabet::D);
    /// assert_eq!(NoteAlphabet::A.transpose(-3), NoteAlphabet::Fs);
    /// assert_eq!(NoteAlphabet::A.transpose(11), NoteAlphabet::Gs);
    /// ```
    pub fn transpose(&self, n: isize) -> Self {
        From::from(self.index() as isize + n)
    }

    #[inline]
    /// Get a note with one half step lower than `self`.
    ///
    /// # Examples
    /// ```
    /// use overlib::buffer::control::NoteAlphabet;
    /// assert_eq!(NoteAlphabet::A.flatten(), NoteAlphabet::Gs);
    /// assert_eq!(NoteAlphabet::C.flatten(), NoteAlphabet::B);
    /// ```
    pub fn flatten(&self) -> Self { self.transpose(-1) }

    #[inline]
    /// Get a note with one half step higher than `self`.
    ///
    /// # Examples
    /// ```
    /// use overlib::buffer::control::NoteAlphabet;
    /// assert_eq!(NoteAlphabet::A.sharpen(), NoteAlphabet::As);
    /// assert_eq!(NoteAlphabet::B.sharpen(), NoteAlphabet::C);
    /// ```
    pub fn sharpen(&self) -> Self { self.transpose(1) }
}

impl From<isize> for NoteAlphabet {
    #[inline]
    /// Get equivalent note with index `i`, starting at [`NoteAlphabet::C`].
    /// [`NoteAlphabet::C`]: #variant.C
    ///
    /// # Examples
    /// ```
    /// use overlib::buffer::control::NoteAlphabet;
    /// assert_eq!(NoteAlphabet::from(0), NoteAlphabet::C);
    /// assert_eq!(NoteAlphabet::from(7), NoteAlphabet::G);
    /// assert_eq!(NoteAlphabet::from(16), NoteAlphabet::E);
    /// assert_eq!(NoteAlphabet::from(-17), NoteAlphabet::G);
    /// ```
    fn from(mut i: isize) -> Self {
        let len = Self::len();
        if i >= 0 {
            match i as usize % len {
                0  => NoteAlphabet::C,
                1  => NoteAlphabet::Cs,
                2  => NoteAlphabet::D,
                3  => NoteAlphabet::Ds,
                4  => NoteAlphabet::E,
                5  => NoteAlphabet::F,
                6  => NoteAlphabet::Fs,
                7  => NoteAlphabet::G,
                8  => NoteAlphabet::Gs,
                9  => NoteAlphabet::A,
                10 => NoteAlphabet::As,
                11 => NoteAlphabet::B,
                _  => unreachable!("limited to 0..len")
            }
        } else {
            let len = len as isize;
            while i < 0 { i += len; }
            From::from(i)
        }
    }
}

macro_rules! ensure (($cond:expr) => (if !$cond { return Err(()); }));

impl<'a> TryFrom<&'a str> for NoteAlphabet {
    type Error = ();

    /// Get equivalent alphabet from a string.
    ///
    /// # Examples
    /// ```
    /// #![feature(try_from)]
    ///
    /// use std::convert::TryFrom;
    /// use overlib::buffer::control::NoteAlphabet;
    ///
    /// assert_eq!(NoteAlphabet::try_from("C"), Ok(NoteAlphabet::C));
    /// assert_eq!(NoteAlphabet::try_from("g#"), Ok(NoteAlphabet::Gs));
    /// assert_eq!(NoteAlphabet::try_from("Cb"), Ok(NoteAlphabet::B));
    /// assert_eq!(NoteAlphabet::try_from(""), Err(()));
    /// assert_eq!(NoteAlphabet::try_from("H"), Err(()));
    /// assert_eq!(NoteAlphabet::try_from("Dx"), Err(()));
    /// assert_eq!(NoteAlphabet::try_from("nonsense"), Err(()));
    /// ```
    fn try_from(alphabet: &'a str) -> Result<Self, Self::Error> {
        let len = alphabet.len();
        ensure!(len == 1 || len == 2);

        let mut alphabet = alphabet.chars();

        let mut index: isize = {
            let mut first = alphabet.next().ok_or(())?.to_uppercase();
            let upper = first.next().ok_or(())?;
            ensure!(first.next().is_none());
            match upper {
                'C' => 0,
                'D' => 2,
                'E' => 4,
                'F' => 5,
                'G' => 7,
                'A' => 9,
                'B' => 11,
                _ => { return Err(()); }
            }
        };

        if let Some(c) = alphabet.next() {
            index += match c {
                'b' => -1,
                '#' => 1,
                _ => { return Err(()) }
            };
        }

        Ok(From::from(index))
    }
}

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
/// Name of a note (e.g. "E6", "A42" "B-21").
pub struct NoteName(pub NoteAlphabet, pub i8);

impl NoteName {
    #[inline]
    /// Get the note's frequency.
    ///
    /// # Examples
    /// ```
    /// use overlib::buffer::control::{NoteAlphabet, NoteName};
    /// let name = NoteName { 0: NoteAlphabet::A, 1: 5 };
    /// assert_eq!(name.freq().round(), 880.);
    /// ```
    pub fn freq(&self) -> f64 { self.detune(0.) }

    #[inline]
    /// Detune frequency by `cents`.
    ///
    /// # Examples
    /// ```
    /// use overlib::buffer::control::NoteAlphabet;
    /// assert_eq!(NoteAlphabet::A.detune(0.), NoteAlphabet::A.freq());
    /// assert_eq!(NoteAlphabet::A.detune(100.), NoteAlphabet::As.freq());
    /// assert_eq!(NoteAlphabet::A.detune(-900.), NoteAlphabet::C.freq());
    /// ```
    pub fn detune<T: Into<f64>>(&self, cents: T) -> f64 {
        let half_steps = (self.1 as isize - 4) * NoteAlphabet::len() as isize;
        let cents = cents.into() + (half_steps * 100) as f64;
        self.0.detune(cents)
    }
}

/// Reference for comparing notes.
pub type NoteRef = NoteName;

#[derive(Debug, Clone, PartialEq)]
/// Value for a specific parameter of a note.
pub enum NoteParam {
    /// Note's amplitude.
    Velocity(f64),

    /// Note's channel amplification.
    Panning(f64),

    /// Note's detuning.
    Cents(f64)
}

#[derive(Debug, Clone, PartialEq)]
/// All parameters of a note.
pub struct NoteParams {
    /// Velocity. 0 means silence and 100 means 0dB.
    pub velocity: f64,

    /// Panning. -100 means only channel and 100 means only right.
    pub panning: f64,

    /// Cents in unit cents. 0 means no detuning, 1200 means one octave
    /// higher.
    pub cents: f64
}

impl NoteParams {
    /// Create from left velocity, right velocity and cents.
    ///
    /// # Examples
    /// ```
    /// use overlib::buffer::control::NoteParams;
    /// let params = NoteParams::from_velocities(100., 100., 0.);
    /// assert_eq!(params, NoteParams { velocity: 100., panning: 0., cents: 0. });
    ///
    /// let params = NoteParams::from_velocities(0., 100., 0.);
    /// assert_eq!(params, NoteParams { velocity: 100., panning: -100., cents: 0. });
    /// ```
    pub fn from_velocities(frame: Frame, cents: f64) -> Self {
        let [left, right]: [f64; 2] = frame.into();

        let (velocity, panning) = if left == right {
            (left, 0.)
        } else if right == 0. {
            (left, 100.)
        } else if left == 0. {
            (right, -100.)
        } else {
            unimplemented!()
        };

        Self { velocity, panning, cents }
    }

    #[inline]
    /// Apply given `param` change.
    ///
    /// # Examples
    /// ```
    /// use overlib::buffer::control::{NoteParam, NoteParams};
    /// let mut params = NoteParams { velocity: 100., panning: 0., cents: 0., };
    /// params.apply(&NoteParam::Velocity(50.));
    /// params.apply(&NoteParam::Panning(-30.));
    /// params.apply(&NoteParam::Cents(100.));
    /// assert_eq!(params, NoteParams { velocity: 50., panning: -30., cents: 100. });
    pub fn apply(&mut self, param: &NoteParam) {
        match param {
            &NoteParam::Velocity(v) => { self.velocity = v },
            &NoteParam::Panning(p) => { self.panning = p },
            &NoteParam::Cents(c) => { self.cents = c }
        }
    }

    #[inline]
    /// Multiply `self.velocity` by `value`.
    ///
    /// # Examples
    /// ```
    /// use overlib::buffer::control::{NoteParam, NoteParams};
    /// let mut params = NoteParams { velocity: 100., panning: 0., cents: 0., };
    /// params.gain(2);
    /// assert_eq!(params, NoteParams { velocity: 200., panning: 0., cents: 0. });
    pub fn gain<T: Into<f64>>(&mut self, gain: T) {
        let amplitude = (self.velocity as f64) * gain.into();
        self.velocity = amplitude as f64;
    }

    #[inline]
    /// Velocity of note on each channel.
    ///
    /// # Examples
    /// ```
    /// use overlib::buffer::control::NoteParams;
    /// let params = NoteParams { velocity: 80., panning: 0., cents: 0. };
    /// assert_eq!(params.velocities(), [80.; 2].into());
    ///
    /// let params = NoteParams { velocity: 100., panning: 100., cents: 0. };
    /// let velocities: [f64; 2] = params.velocities().round().into();
    /// assert_eq!(velocities, [100., 0.]);
    /// ```
    pub fn velocities(&self) -> Frame {
        let v = self.velocity as f64;

        if self.panning == 0. {
            return v.into();
        }

        use std::f64::consts::{FRAC_PI_4, SQRT_2};
        let t = self.panning.abs() as f64 * (FRAC_PI_4 / 100.);
        let (c, s) = (SQRT_2 * t.cos() / 2., SQRT_2 * t.sin() / 2.);
        let (h, l) = (c + s, c - s);

        if self.panning.is_sign_positive() {
            (h * v, l * v).into()
        } else {
            (l * v, h * v).into()
        }
    }
}

impl Default for NoteParams {
    #[inline]
    fn default() -> Self {
        Self { velocity: 100., panning: 0., cents: 0. }
    }
}

#[derive(Debug, Clone, PartialEq)]
/// A note with it's parameters.
pub struct Note {
    pub name: NoteName,
    pub params: NoteParams
}

impl Note {
    #[inline]
    /// Get frequency of the note.
    pub fn freq(&self) -> f64 {
        self.detune(0.)
    }

    #[inline]
    /// Get frequency of the note, detuning `cents` cents.
    pub fn detune<T: Into<f64>>(&self, cents: T) -> f64 {
        let cents = self.params.cents as f64 + cents.into();
        self.name.detune(cents)
    }

    #[inline]
    /// Gain velocity of the note.
    pub fn gain<T: Into<f64>>(&mut self, value: T) {
        self.params.gain(value)
    }
}
