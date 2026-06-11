use crate::dspmodules::dspmodule::DSPModule;

pub struct Gain {
    gain: f32,
}

// impl DSPModule for Gain {
//     fn initalize(&mut self) {
//         self.gain = 0.0;
//     }

//     fn process(&mut self, sigs: Vec<f32>) -> f32 {
//         sigs[0] * self.gain
//     }

//     fn reset(&mut self) {
//         self.gain = 0.0;
//     }
// }