use std::f64::consts::PI;
use ::buffer::prelude::*;
use ::plugins::prelude::*;
use ::utils::note::ActiveNotes;
use ::hardconf::RATE;
use ::Core;
use super::NativePlugin;

const TWO_PI: f64 = 2. * PI;
const COEFF: f64 = TWO_PI / RATE as f64;

#[inline]
fn calculate(position: usize, phase: &mut f64, note: &Note) -> Frame {
    let frame: Frame = phase.sin().into();

    let next = *phase + COEFF * position as f64 * note.freq();

    *phase = if next >= TWO_PI { next - TWO_PI } else { next };

    frame * note.params.velocities().into()
}

#[derive(Default)]
pub struct FunctionGenerator(ActiveNotes<f64>);

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
                    self.0.apply_moment(moment, &0.);
                    *frame = self.0.iter_mut()
                        .map(|(note, phase)| calculate(i, phase, note))
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
