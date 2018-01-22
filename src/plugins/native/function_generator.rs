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
fn calculate(position: usize, state: &mut NoteState) -> Frame {
    let frame: Frame = state.phase.sin().into();

    let phase = state.phase + COEFF * position as f64 * state.freq;

    state.phase = if phase >= TWO_PI { phase - TWO_PI } else { phase };

    frame * state.velocities.clone()
}

struct NoteState {
    params: NoteParams,
    phase: f64,
    // cache to note frequency
    freq: f64,
    // cache to velocities
    velocities: Frame
}

#[derive(Default)]
pub struct FunctionGenerator(HashMap<NoteName, NoteState>);

impl FunctionGenerator {
    #[inline]
    fn apply_moment(&mut self, moment: &Moment) {
        if let &Some(ref events) = moment {
            for event in events.iter() {
                match event {
                    &Event::NoteOn(ref note, _) => {
                        let state = NoteState {
                            params: note.params.clone(),
                            phase: 0.,
                            freq: note.freq(),
                            velocities: note.params.velocities().into()
                        };
                        self.0.insert(note.name.clone(), state);
                    },
                    &Event::NoteSet(ref name, ref param) => {
                        if let Some(state) = self.0.get_mut(name) {
                            state.params.apply(param);
                            if let &NoteParam::Cents(c) = param {
                                state.freq = name.detune(c)
                            } else {
                                state.velocities = state.params.velocities().into();
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
                        .map(|state| calculate(i, state))
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
