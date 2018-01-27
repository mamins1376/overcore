use Core;
use super::prelude::*;
use super::{Factory, FactoryDesc};

pub mod function_generator;
use self::function_generator::FunctionGenerator;

pub trait NativePlugin: Plugin {
    fn new(core: &Core) -> Self;

    fn get_uuid() -> &'static str;

    fn get_desc(id: usize) -> PluginDesc;
}

/// A factory which holds native plugins
pub struct NativeFactory<'a> { core: &'a Core }

impl<'a> NativeFactory<'a> {
    /// Create new `NativeFactory`.
    pub fn new(core: &'a Core) -> Self { Self { core } }
}

impl<'a> Factory for NativeFactory<'a> {
    fn get_descriptor(&self) -> FactoryDesc {
        FactoryDesc {
            uuid: "d5d0cdb6-24bd-4223-92c6-7f59ca0d9502".to_owned(),
            name: "Native Factory".to_owned(),
            description: "entry of native plugins.".to_owned()
        }
    }

    fn get_plugins(&self) -> Box<[PluginDesc]> {
        box [ FunctionGenerator::get_desc(0) ]
    }

    fn create_plugin(&mut self, id: usize) -> PluginResult<Box<Plugin>> {
        match id {
            0 => Ok(box FunctionGenerator::new(&self.core)),
            _ => Err(PluginError::InvalidArgument)
        }
    }
}
