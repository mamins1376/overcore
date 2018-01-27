use std::str::FromStr;
use ::license;

// a workaround for kind - shouldn't it simply impl License???
fn kind_as_license(kind: &license::Kind) -> &license::License {
    match kind {
        &license::Kind::Agpl3(ref license) => license,
        &license::Kind::Apache2(ref license) => license,
        &license::Kind::Cc01(ref license) => license,
        &license::Kind::Gpl3(ref license) => license,
        &license::Kind::Lgpl3(ref license) => license,
        &license::Kind::Mit(ref license) => license,
        &license::Kind::Mpl2(ref license) => license,
        &license::Kind::Unlicense(ref license) => license
    }
}

/// Represents a license.
#[derive(Clone)]
pub enum License<'a> {
    Known(license::Kind),
    Unknown(&'a str)
}

impl<'a> License<'a> {
    /// Check whether the license is known or not.
    ///
    /// # Examples
    /// ```
    /// use overcore::meta::License;
    ///
    /// let license = License::from("MIT");
    /// assert_eq!(license.is_known(), true);
    ///
    /// let license = License::from("a custom license");
    /// assert_eq!(license.is_known(), false);
    /// ```
    pub fn is_known(&self) -> bool {
        if let &License::Known(_) = self { true } else { false }
    }

    /// Unwrap if it is a known license.
    ///
    /// # Examples
    /// ```
    /// use overcore::meta::License;
    ///
    /// let license = License::from("MIT");
    /// assert_eq!(license.as_known().is_some(), true);
    ///
    /// let license = License::from("a custom license");
    /// assert_eq!(license.as_known().is_none(), true);
    /// ```
    pub fn as_known(&'a self) -> Option<&'a license::License> {
        match self {
            &License::Known(ref kind) => Some(kind_as_license(kind)),
            &License::Unknown(_) => None
        }
    }

    /// Get license's text.
    ///
    /// # Examples
    /// ```
    /// use overcore::meta::License;
    ///
    /// let license = License::from("MIT");
    /// assert_eq!(license.text().starts_with("The MIT License"), true);
    ///
    /// let license = License::from("a custom license");
    /// assert_eq!(license.text(), "a custom license");
    /// ```
    pub fn text(&'a self) -> &'a str {
        match self {
            &License::Known(ref kind) => kind_as_license(kind).text(),
            &License::Unknown(ref text) => text
        }
    }
}

impl<'a> From<&'a str> for License<'a> {
    /// Return corresponding license from it's name. If the name was not a
    /// known license, it considered an unkown license with text `s`.
    fn from(s: &'a str) -> Self {
        match license::Kind::from_str(s) {
            Ok(kind) => License::Known(kind),
            Err(_) => License::Unknown(s)
        }
    }
}
