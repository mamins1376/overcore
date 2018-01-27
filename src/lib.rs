//#![deny(missing_docs)]

#![feature(slice_patterns)]
#![feature(conservative_impl_trait)]
#![feature(box_syntax)]
#![feature(try_from)]

extern crate license;
extern crate sample;

pub mod hardconf;
pub mod buffer;
pub mod interpolate;
pub mod meta;
pub mod plugins;

pub mod graph;

use self::graph::Graph;

#[derive(Debug, Clone)]
pub struct CoreConfig {
    pub sample_rate: u32,
    pub buffer_size: usize,
    pub pool_preallocate: usize
}

#[derive(Debug, Clone, PartialEq)]
pub enum CoreStatus { Created, Initialized, Processing, Idle, CleaningUp }

pub struct Core {
    pub config: CoreConfig,

    pub status: CoreStatus,

    graph: Option<Graph>
}

impl Core {
    pub fn new(config: CoreConfig) -> Self {
        let (status, graph) = (CoreStatus::Created, None);
        Self { config, status, graph }
    }

    pub fn initialize(&mut self) {
        assert!(self.status == CoreStatus::Created);

        self.graph = Some(Graph::new(&self));

        self.status = CoreStatus::Initialized;
    }

    pub fn start(&self) {}
}
