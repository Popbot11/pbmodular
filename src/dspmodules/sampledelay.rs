use crate::dspmodules::dspmodule::{DSPModule, Signal};
use std::mem;
/// a single sample delay. keeps track of a single value, which is the sample being delayed. 
/// 
/// accepts Single, outputs Single 
pub struct SampleDelay {
    s1: Signal<f32>
}
impl DSPModule for SampleDelay{
    fn process(&mut self, signal: Signal<f32>) -> Signal<f32> {
        // let result = &self.s1;
        // self.s1 = signal;
        let result = mem::replace(&mut self.s1, signal);
        result
    }
    fn initalize(&mut self) {

    }
    fn reset(&mut self) {

    }
}
