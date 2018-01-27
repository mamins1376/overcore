#[derive(Debug, PartialEq, Clone)]
pub enum ParamType { Unsigned, Signed, Float, Index, Boolean }

#[derive(Debug, PartialEq, Clone)]
pub enum ParamValue {
    Unsigned(u64),
    Signed(i64),
    Float(f64),
    Index(usize),
    Boolean(bool)
}

/* TODO: fix macro
macro_rules! fn_is_as {
    ($n:ident, $i_f:ident, $a_f:ident, $p:tt, $t:ty) => (
        /// Check if `self` is ParamValue::
        #[doc = stringify!($p)]
        /// .
        ///
        /// # Examples
        /// ```
        /// use overcore::buffer::control::ParamValue;
        /// let value = ParamValue::new_$name(0 as
        #[doc = stringify!($t)]
        /// );
        /// assert_eq!(value.is_
        #[doc = stringify!($t)]
        /// (), true);
        /// ```
        pub fn $i_f(&self) -> bool {
            if let &ParamValue::$p(_) = self { true } else { false }
        }

        /// Try to unwrap `self` as ParamValue::$variant with type $type.
        ///
        /// # Panics
        /// This function panics if value of `self` is not
        /// ParamValue::$variant.
        ///
        /// # Examples
        /// ```
        /// use std::panic::catch_unwind;
        /// use overcore::buffer::control::ParamValue;
        ///
        /// let value = ParamValue::new_$name(0 as $type);
        /// assert_eq!(value.as_$name(), 0 as $type);
        ///
        /// let result = panic_unwind(|| value.as_$invalid())
        /// assert_eq!(result.is_err(), true);
        /// ```
        pub fn $a_f(&self) -> $t {
            if let &ParamValue::$p(val) = self { val } else {
                panic!("Attemped to read as {}", stringify!($p);)
            }
        }
    );
    ($n:ident, $p:tt, $t:ty) => (fn_is_as!($n, (concat_idents!(is_, $n)),
        (concat_idents!(as_, $n)), $p, $t);)
}
*/

impl ParamValue {
    /// Get type of `self`.
    ///
    /// # Examples
    /// ```
    /// use overcore::buffer::control::{ParamType, ParamValue};
    /// let value = ParamValue::Index(42);
    /// assert_eq!(value.get_type(), ParamType::Index);
    /// ```
    pub fn get_type(&self) -> ParamType {
        match self {
            &ParamValue::Unsigned(_) => ParamType::Unsigned,
            &ParamValue::Signed(_) => ParamType::Signed,
            &ParamValue::Float(_) => ParamType::Float,
            &ParamValue::Index(_) => ParamType::Index,
            &ParamValue::Boolean(_) => ParamType::Boolean
        }
    }

    /* TODO: fix macro
    fn_is_as!(unsigned, is_unsigned, as_unsigned, Unsigned, u64);
    fn_is_as!(unsigned, Unsigned, u64);
    fn_is_as!(signed, Signed, i64);
    fn_is_as!(float, Float, f64);
    fn_is_as!(index, Index, usize);
    fn_is_as!(boolean, Boolean, bool);
    */
}
