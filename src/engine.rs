use std::thread::{self, JoinHandle};
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};
use util::AtomicOption;
use types::OResult;
use context::Context;
use root::Root;
use playback::Playback;

pub struct Engine<'a> {
    core: &'a Overcore,
    playback: Mutex<Option<Box<Playback>>>,
    flag: AtomicBool,
    thread: AtomicOption<JoinHandle<OResult<()>>>
}

impl<'a> Engine<'a> {
    pub fn new(core: &'a Overcore) -> Self {
        let playback = Mutex::new(None);
        let flag = AtomicBool::new(false);
        let thread = AtomicOption::new(None);
        Engine { core, playback, flag, thread }
    }

    pub fn use_playback(playback: Box<Playback>) -> OResult<()> {
        x
    }

    pub fn start(&self) -> OResult<()> {
        debug_assert!(self.thread.take(Ordering::SeqCst).is_none());

        let handle = thread::Builder::new().name("engine".to_owned());
        let handle = handle.spawn(|| {
            let this = instance().engine;

            {
                let mut playback = this.playback.lock().unwrap();

                playback.open()?;

                playback.run(&this.flag)?;

                playback.close()?;
            }

            Ok(())
        }).unwrap();

        {
            let handle = Some(box handle);
            let old = self.thread.swap(handle, Ordering::SeqCst);
            debug_assert!(old.is_none());
        }

        Ok(())
    }

    pub fn stop(&self) -> OResult<()> {
        let handle = self.thread.take(Ordering::SeqCst).ok_or("Engine is not running")?;
        self.flag.store(true, Ordering::SeqCst);
        handle.join().unwrap();
        Ok(())
    }

    pub fn is_started(&self) -> bool {
        self.thread.is_some(Ordering::SeqCst)
    }

    pub fn toggle(&self) -> OResult<()> {
        if self.is_started() { self.stop() } else { self.start() }
    }
}
