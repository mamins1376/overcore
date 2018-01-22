use std::f64::consts::PI;
use std::collections::HashMap;
use ::buffer::prelude::*;
use ::plugins::prelude::*;
use ::hardconf::RATE;
use ::Core;
use super::NativePlugin;

const TWO_PI: f64 = 2. * PI;
const COEFF: f64 = TWO_PI / RATE as f64;

#[inline]
fn calculate(position: usize, handle: &mut NoteHandle) -> Frame {
    let frame: Frame = handle.phase.sin().into();

    let phase = handle.phase + COEFF * position as f64 * handle.freq;

    handle.phase = if phase >= TWO_PI { phase - TWO_PI } else { phase };

    frame * handle.note.params.velocities().into()
}

struct NoteHandle {
    note: Note,
    phase: f64,
    // cache to note frequency
    freq: f64
}

#[derive(Default)]
pub struct FunctionGenerator(HashMap<NoteName, NoteHandle>);

impl FunctionGenerator {
    #[inline]
    fn apply_moment(&mut self, moment: &Moment) {
        if let &Some(ref events) = moment {
            for event in events.iter() {
                match event {
                    &Event::NoteOn(ref note, _) => {
                        let handle = NoteHandle {
                            note: note.clone(),
                            phase: 0.,
                            freq: note.freq()
                        };
                        self.0.insert(note.name.clone(), handle);
                    },
                    &Event::NoteSet(ref name, ref param) => {
                        if let Some(handle) = self.0.get_mut(name) {
                            handle.note.params.apply(param);
                            if let &NoteParam::Cents(cents) = param {
                                handle.freq = name.detune(cents);
                            }
                        }
                    },
                    &Event::NoteOff(ref name) => {
                        self.0.remove(name);
                    },
                    &Event::Panic => {
                        self.0.clear()
                    },
                    _ => {}
                }
            }
        }
    }
}

impl Plugin for FunctionGenerator {
    fn get_io_descriptor(&self) -> PluginIoDesc {
        use std::convert::TryFrom;
        TryFrom::try_from("Control:C|Wave:A").unwrap()
    }

    fn process(&mut self, inputs: &PluginIo, outputs: &mut PluginIo)
        -> PluginResult<()> {
        if inputs.len() != 1 || outputs.len() != 1 {
            return Err(PluginError::InvalidArgument);
        }

        if let PluginIoBuffer::Control(ref control) = inputs[0] {
            if let PluginIoBuffer::Audio(ref mut audio) = outputs[1] {
                let items = control.iter().zip(audio.iter_mut());

                for (i, (moment, frame)) in items.enumerate() {
                    self.apply_moment(moment);

                    *frame = self.0.values_mut()
                        .map(|handle| calculate(i, handle))
                        .sum();
                }
            }
        }

        Ok(())
    }
}

impl NativePlugin for FunctionGenerator {
    fn new(_: &Core) -> Self { Default::default() }

    fn get_uuid() -> &'static str { "ea8467d9-6b7d-41fa-b9f6-a33e28db701f" }

    fn get_desc(id: usize) -> PluginDesc {
        PluginDesc::default().with_id(id).with_uuid(Self::get_uuid())
            .with_name("Function Generator")
            .with_description("generates multiple waves.")
    }
}
