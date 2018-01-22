//! The Special Plugins Module
//!
//! Some plugins have common structure and IO configuration, and so special
//! names. This module contains some traits that makes them more generic.
//!
//! These traits are supposed to *only reduce the boilerplate* and are not
//! used to categorize plugins. For instance, One can implement it's own
//! audio effect without implemeting [`Effect`] trait.
//! [`Effect`]: trait.Effect.html


/* Currently we can't do this:
 *
 *     impl<T: Manipulator> Plugin for T { ... }
 *     impl<T: Generator> Plugin for T { ... }
 *
 * due to implementation conflicts caused by implementors. Maybe specialization
 * fixes those when landed; but for now we have to use macros to implement
 * common boilerplate for special plugins and can't use traits for that.
 *
 * TODO: Replace `impl_*_for` macros with implementors.
 */



