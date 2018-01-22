use std::convert::TryFrom;
use ::buffer::control::ControlBuffer;
use ::buffer::audio::AudioBuffer;

#[derive(Debug, Clone, PartialEq)]
/// Represents the kind of an IO buffer.
pub enum PluginIoKind {
    /// The buffer is a [`ControlBuffer`](../buffer/struct.ControlBuffer.html).
    Control,

    /// The buffer is an [`AudioBuffer`](../buffer/struct.AudioBuffer.html).
    Audio,

    /// The buffer can be either of last two buffers and the type is handled
    /// on-demand.
    Either
}

#[derive(Debug, Clone, PartialEq)]
/// Represents IO configuration in [`PluginIoMode::Inplace`][0] mode.
/// [0]: enum.PluginIoMode.html#variant.Inplace
pub struct PluginInplaceIo {
    /// list of buffers that used as both input and output.
    pub buffers: Box<[PluginIoKind]>,

    /// a mapping that defines which output belongs to which input. must contain
    /// valid indexes to `buffers` and no duplicates. can be checked through
    /// [`is_valid()`](#method.is_valid).
    pub mapping: Box<[usize]>
}

impl PluginInplaceIo {
    /// Check whether this inplace mode is valid or not.
    ///
    /// # Examples
    /// ```
    /// use overlib::plugins::PluginIoKind;
    /// use overlib::plugins::PluginInplaceIo;
    ///
    /// let inplace = PluginInplaceIo {
    ///     buffers: Box::new([]),
    ///     mapping: Box::new([0])
    /// };
    /// assert_eq!(inplace.is_valid(), false);
    ///
    /// let inplace = PluginInplaceIo {
    ///     buffers: Box::new([PluginIoKind::Control]),
    ///     mapping: Box::new([0])
    /// };
    /// assert_eq!(inplace.is_valid(), true);
    /// ```
    pub fn is_valid(&self) -> bool {
        let mut duplicates = Vec::new();
        for index in self.mapping.iter() {
            if *index >= self.buffers.len() { return false; }

            if duplicates.contains(index) { return false; }
            else { duplicates.push(*index) }
        }
        true
    }
}

impl<'a> TryFrom<&'a str> for PluginInplaceIo {
    type Error = ();

    fn try_from(_s: &'a str) -> Result<Self, ()> { Err(()) }
}

#[derive(Debug, Clone, PartialEq)]
/// Represents IO configuration in [`PluginIoMode::Complex`] mode.
/// [`PluginIoMode::Complex`]: enum.PluginIoMode.html#variant.Complex
pub struct PluginComplexIo {
    /// list of input buffers kind.
    pub inputs: Box<[PluginIoKind]>,

    /// list of outputt buffers kind.
    pub outputs: Box<[PluginIoKind]>
}

#[derive(Debug, Clone, PartialEq)]
/// Represents IO mode for a plugin.
pub enum PluginIoMode {
    /// In this mode, plugin receives all buffers as it's `outputs` argument
    /// (which is mutable) and should read all of input frames from them and
    /// also write the result frames to those buffers. For more details, look
    /// at [`PluginInplaceIo`]. `inputs` argument on [`Plugin::process()`]
    /// would be an empty boxed slice.
    /// [`PluginInplaceIo`]: struct.PluginInplaceIo.html
    /// [`Plugin::process()`]: trait.Plugin.html#tymethod.process
    Inplace(PluginInplaceIo),

    /// In this mode, plugin is simply reading frames from `inputs` and
    /// writing the results to `outputs`.
    Complex(PluginComplexIo)
}

impl PluginIoMode {
    /// Checks whether this mode is valid or not. See [`PluginInplaceIo`]
    /// for more details.
    /// [`PluginInplaceIo`]: struct.PluginInplaceIo.html
    ///
    /// # Examples
    /// ```
    /// use overlib::plugins::{PluginInplaceIo, PluginComplexIo, PluginIoMode};
    ///
    /// let mode = PluginIoMode::Complex(PluginComplexIo {
    ///     inputs: Box::new([]), outputs: Box::new([])
    /// });
    /// assert_eq!(mode.is_valid(), true);
    ///
    /// let mode = PluginIoMode::Inplace(PluginInplaceIo {
    ///     buffers: Box::new([]), mapping: Box::new([0])
    /// });
    /// assert_eq!(mode.is_valid(), false);
    /// ```
    pub fn is_valid(&self) -> bool {
        match *self { PluginIoMode::Inplace(ref m) => m.is_valid(), _ => true }
    }
}

#[derive(Clone)]
/// Descriptor structure for plugin IO.
pub struct PluginIoDesc {
    /// The mode which plugin processes on.
    pub mode: PluginIoMode,

    /// Name of each input buffer. Length of this list must be equal to the
    /// length of input buffers depending on `mode`.
    pub inputs: Box<[String]>,

    /// Name of each output buffer. Length of this list must be equal to the
    /// length of output buffers depending on `mode`.
    pub outputs: Box<[String]>
}

impl PluginIoDesc {
    /// Checks whether mode is valid or not; and if it is, also checks the
    /// length of `inputs` and `outputs`.
    pub fn is_valid(&self) -> bool { false }
}

impl<'a> TryFrom<&'a str> for PluginIoDesc {
    type Error = ();

    fn try_from(_s: &'a str) -> Result<Self, ()> { Err(()) }
}

#[derive(Debug, Clone)]
/// A holder to pass the buffer to the plugin.
pub enum PluginIoBuffer {
    /// Holds a control buffer.
    Control(ControlBuffer),

    /// Holds an audio buffer.
    Audio(AudioBuffer),

    /// Indicates that the buffer is *disconnected* (i.e. no path is feeding
    /// it if it's an input or no one is using it if it's output).
    Disconnected
}

/* TODO: fix macro for this section
macro_rules! fn_is {
    ($name:ident, $p:tt) => (
        pub fn $name(&self) -> bool {
            if let &PluginIoBuffer::$p = self { true } else { false }
        }
    )
}

macro_rules! fn_as {
    ($as_ref:ident, $as_mut:ident, $p:tt, $b:ty) => (
        #[inline]
        /// Tries to get a reference to internal buffer as a `$buffer`.
        /// if this buffer is not one, returns `None`.
        pub fn $as_ref(&self) -> Option<&$b> {
            if let &PluginIoBuffer::$p(ref buffer) = self {
                Some(buffer)
            } else {
                None
            }
        }

        #[inline]
        /// Tries to get a mutable reference to internal buffer as a
        /// `$buffer`.  if this buffer is not one, returns `None`.
        pub fn $as_mut(&mut self) -> PluginResult<&mut $b> {
            if let &mut PluginIoBuffer::$p(ref mut buffer) = self {
                Ok(buffer)
            } else {
                None
            }
        }
    )
}

impl PluginIoBuffer {
    #[inline]
    /// Checks whether this buffer is disconnected from another plugin or not.
    fn_is!(is_disconnected, Disconnected);

    #[inline]
    /// Checks whether this buffer is a control buffer or not.
    fn_is!(is_control, Control(x));

    #[inline]
    /// Checks whether this buffer is an audio buffer or not.
    fn_is!(is_audio, Audio(x));

    fn_as!(as_control, as_mut_control, Control, ControlBuffer);
    fn_as!(as_audio, as_mut_audio, Audio, AudioBuffer);

    #[inline]
    /// Checks whether this buffer is connected to another plugin or not.
    pub fn is_connected(&self) -> bool { !self.is_disconnected() }
}
*/

/// List of buffers. Used by [`Plugin::process()`].
/// [`Plugin::process()`]: trait.Plugin.html#tymethod.process
pub type PluginIo = Box<[PluginIoBuffer]>;

#[derive(Clone)]
/// Represents a change in plugin's input or output buffers status.
pub struct PluginIoChange {
    /// Indicates whether the change is for an output buffer or not.
    is_output: bool,

    /// The index of corresponding buffer.
    index: usize,

    /// The new status for buffer.
    status: Option<PluginIoKind>
}
