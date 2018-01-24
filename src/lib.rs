//#![deny(missing_docs)]

#![feature(slice_patterns)]
#![feature(conservative_impl_trait)]
#![feature(box_syntax)]
#![feature(try_from)]

extern crate license;
extern crate sample;

pub mod hardconf;
pub mod buffer;
pub mod meta;
pub mod plugins;

pub mod graph;

use std::sync::{Arc, Weak};
use std::ops::Deref;
use self::buffer::BufferPool;
use self::buffer::prelude::{ControlBuffer, AudioBuffer};
use self::graph::Graph;

pub struct Config {
    pub sample_rate: u32,
    pub buffer_size: usize,
    pub pool_preallocate: usize
}

pub struct Pools {
    pub control: BufferPool<ControlBuffer>,
    pub audio: BufferPool<AudioBuffer>
}

pub struct CoreInner {
    pub config: Config,
    pub pools: Pools,
    pub graph: Graph
}

impl CoreInner {
    pub fn new(config: Config) -> Arc<Self> {
        let control = BufferPool::new(config.buffer_size);
        let audio = BufferPool::new(config.buffer_size);
        let pools = Pools { control, audio };
        let graph = Graph::new();
        let this = Arc::new(Self { config, pools, graph });

        this.graph.initialize(&this);

        this
    }

    pub fn start(&self) {}
}

pub type Core = Arc<CoreInner>;

pub type WeakCore = Weak<CoreInner>;

pub struct Overlib(Arc<CoreInner>);

impl Deref for Overlib {
    type Target = Core;

    fn deref(&self) -> &Core { &self.0 }
}
