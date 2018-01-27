//! The Plugins Module
//!
//! Holds data structures and traits related to plugin and
//! factory standard.

use std::collections::HashMap;
use super::hardconf;

pub mod prelude {
    //! The Plugins Prelude
    //!
    //! `use` this module to import common things used when implementing a
    //! plugin.
    //!
    //! # Examples
    //! ```
    //! use overcore::plugins::prelude::*;
    //! ```

    pub use super::{PluginError, PluginResult};
    pub use super::{PluginParamKind, PluginParamDesc};
    pub use super::{PluginIoKind, PluginInplaceIo, PluginComplexIo};
    pub use super::{PluginIoMode, PluginIoDesc};
    pub use super::{PluginIoBuffer, PluginIo, PluginIoChange};
    pub use super::{PluginDesc, Plugin};
}

mod io;
pub use self::io::*;

pub mod core;

#[derive(Debug, Clone, PartialEq)]
/// Represents an error in some operation done by a plugin or factory.
pub enum PluginError {
    /// Given argument does not follow the specified conditions.
    InvalidArgument,

    /// Something went wrong with the plugin or factory itself.
    InternalError,

    /// A factory failed to load a plugin.
    LoadFailure,

    /// A catch-all for other errors that don't fit in previous variants.
    UnknownError
}

/// A result which plugins and factories return on some operations.
pub type PluginResult<T> = Result<T, PluginError>;

#[derive(Debug, Clone, PartialEq)]
/// Represents the value type of a parameter.
pub enum PluginParamKind {
    /// It's an [`u64`] with specified minimum and maximum, respectively.
    Unsigned(u64, u64),

    /// It's an [`i64`] with specified minimum and maximum, respectively.
    Signed(i64, i64),

    /// It's an [`f64`] with specified minimum and maximum, respectively.
    Float(f64, f64),

    /// It's an enum with possible values of given names.
    Enum(Box<[String]>),

    /// It's a [`bool`].
    Boolean
}

#[derive(Debug, Clone, PartialEq)]
/// Descriptor for a parameter.
pub struct PluginParamDesc {
    /// Name of the parameter.
    pub name: String,

    /// Type of parameter value.
    pub kind: PluginParamKind
}

/// List of parameter descriptors. Returned by [`Plugin::get_params()`][0].
/// [0]: trait.Plugin.html#tymethod.get_params
pub type PluginParamsDesc = Box<[PluginParamDesc]>;

/// A trait that every plugin implements.
pub trait Plugin {
    /// Initialize the plugin. In this method, the plugin should do it's
    /// allocations, set parameter default values, do start-up calculations,
    /// etc.
    ///
    /// # Return Value
    /// The return value indicates that the initialization was successful or
    /// not. If not, the plugin will be destroyed, **without calling
    /// plugin's [`terminate()`]**.
    /// [`terminate()`]: trait.Plugin.html#tymethod.terminate
    fn initialize(&mut self) -> PluginResult<()> { Ok(()) }

    /// This method is called when some core context is changed such as
    /// sample rate. The plugin should handle the event by updating it's
    /// internal state if needed.
    fn core_changed(&mut self) {}

    /// The plugin should return description of it's parameters.
    fn get_params(&self) -> PluginParamsDesc { box [] }

    /// Called when some IO buffer's status is changed. If the returning
    /// result was [`PluginError::InvalidArgument`][0], the core would call
    /// the [`get_io_descriptor()`][1] method to know the correct buffer
    /// configuration.
    /// [0]: enum.PluginError.html#variant.InvalidArgument
    /// [1]: trait.Plugin.html#tymethod.get_io_descriptor
    fn io_changed(&mut self, _change: PluginIoChange) -> PluginResult<()> {
        Ok(())
    }

    /// This method is called when the core needs to know the plugin's
    /// buffer configuration. After calling this method, plugin's
    /// [`io_changed()`][0] is called for each buffer to inform the
    /// IO buffers status.
    fn get_io_descriptor(&self) -> PluginIoDesc;

    /// This method is called on each render cycle, even if no buffers are
    /// connected to the plugin.
    ///
    /// # Parameters
    /// `inputs` is list of input buffers in complex mode, and an empty
    /// list in inplace mode. `outputs` is list of output buffers in
    /// complex mode, and list of all buffers in inplace mode.
    ///
    /// For more details about operation modes, take a look at
    /// [`PluginIoMode`](struct.PluginIoMode.html).
    ///
    /// # Return Value
    /// If the return value indicates an error, depending on error value
    /// different things would happen:
    /// - [`InvalidArgument`][0] means the IO description is changed and
    ///   should be updated.
    /// - [`InternalError`][1] is ignored until it's frequently returned
    ///   and after passing a rate treshold, the plugin whould be destroyed.
    /// - [`LoadFailure`][2] would indicate that plugin should be destroyed
    ///   immediately.
    /// - [`UnknownError`][3] would be ignored. the plugin's output buffers
    ///   are cleared (if any) to prevent unwanted partially incorrect results.
    /// [0]: enum.PluginError.html#variant.InvalidArgument
    /// [1]: enum.PluginError.html#variant.InternalError
    /// [2]: enum.PluginError.html#variant.LoadFailure
    /// [3]: enum.PluginError.html#variant.UnknownError
    fn process(&mut self, inputs: &PluginIo, outputs: &mut PluginIo)
        -> PluginResult<()>;

    /// Called before plugin's deallocation if it was initialized
    /// successfully.
    fn terminate(&mut self) {}
}

/// A plugin that is waitable would implement this. The engine would call the
/// [`wait()`] to fill the time gap between render cycles. This trait is
/// usually used by playback backends to keep the engine in sync with the
/// playback hardware.
pub trait WaitablePlugin: Plugin {
    /// Is called between render cycles on chosen plugin. In an engine
    /// instance only one plugin would be the plugin which it's wait method
    /// is called. Should block the caller until it's time to render the new
    /// buffer. On non-realtime backends (e.g. rendering to a file), this
    /// would do nothing to render as fast as possible.
    fn wait(&mut self) -> PluginResult<()> { Ok(()) }
}


#[derive(Clone)]
/// Descriptor for a factory.
pub struct FactoryDesc {
    /// Factories UUID. Used to reference the factory in a unique manner.
    pub uuid: String,

    /// Factory's name.
    pub name: String,

    /// Factory's description.
    pub description: String
}

#[derive(Clone)]
/// Descriptor for a plugin which is returned by it's factory.
pub struct PluginDesc {
    /// Plugin's id local to it's factory.
    pub id: usize,

    /// Plugin's UUID. Used to reference the plugin in a unique manner.
    pub uuid: String,

    /// Plugin's name.
    pub name: String,

    /// The category that plugin would fit to. subcategories are seperated
    /// by a dot ('.').
    pub category: String,

    /// Plugin's description.
    pub description: String,

    /// The url that plugin developer provides to get more information about
    /// the plugin.
    pub url: String,

    /// Plugin's developer name.
    pub developer: String,

    /// Plugin's version.
    pub version: String,

    /// Plugin's license. Converted to actual license using [`License`].
    /// [`License`]: ../meta/struct.License.html
    pub license: String,

    /// Additional custom properties of a plugin if any.
    pub extra: Option<HashMap<String, String>>
}

impl Default for PluginDesc {
    fn default() -> Self {
        let empty = || "".to_owned();
        Self {
            id: 0,
            uuid: empty(),
            name: "[unknown]".to_owned(),
            category: "General".to_owned(),
            description: empty(),
            url: hardconf::OVERDAW_URL.to_owned(),
            developer: "Overdaw Developers".to_owned(),
            version: "0.1".to_owned(),
            license: "MIT".to_owned(),
            extra: None
        }
    }
}

macro_rules! with_field {
    ($n:ident, $f:ident) => (
        /*
        #[doc = concat!("Set `self.", $sf, "` to `")]
        #[doc = $sf]
        #[doc = "`.\n\n# Examples\n```\nuse overcore::plugins::PluginDesc;"]
        #[doc = "\nlet desc = PluginDesc::default()."]
        #[doc = $sn]
        #[doc = "(\"field value\");\nassert_eq!(desc."]
        #[doc = $sf]
        #[doc = ", \"field value\");\n```"]
        */
        /// Set a specific field.
        pub fn $n(mut self, $f: &str) -> Self {
            self.$f.clear(); self.$f.push_str($f); self
        }
    )
}

impl PluginDesc {
    /// Set [`self.id`](#structfield.id) to `id`.
    ///
    /// # Examples
    /// ```
    /// use overcore::plugins::PluginDesc;
    /// let desc = PluginDesc::default().with_id(42);
    /// assert_eq!(desc.id, 42);
    /// ```
    pub fn with_id(mut self, id: usize) -> Self { self.id = id; self }

    with_field!(with_uuid, uuid);
    with_field!(with_name, name);
    with_field!(with_category, category);
    with_field!(with_description, description);
    with_field!(with_url, url);
    with_field!(with_developer, developer);
    with_field!(with_version, version);
    with_field!(with_license, license);
}

/// A trait that each factory implements.
pub trait Factory {
    /// Initialize and return a trait object of subfactories.
    fn get_subfactories(&self) -> Box<[Box<Factory>]> { box [] }

    /// Return factory's descriptor.
    fn get_descriptor(&self) -> FactoryDesc;

    /// Return a list of factory's available plugin's descriptors.
    fn get_plugins(&self) -> Box<[PluginDesc]>;

    /// Create an instance of the plugin specified by it's factory `id`.
    fn create_plugin(&mut self, id: usize) -> PluginResult<Box<Plugin>>;
}
