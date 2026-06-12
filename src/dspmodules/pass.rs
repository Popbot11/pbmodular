use crate::dspmodules::dspmodule::{DSPModule, Signal};


/// simple example of how dsp modules that don't need to keep track of a state will normally have zero fields.  
pub struct Pass {}
impl DSPModule for Pass {
    fn initalize(&mut self) {
        
    }
    fn reset(&mut self) {
        
    }
    fn process(&mut self, signal: Signal<f32>) -> Signal<f32> {
        signal
    }
}