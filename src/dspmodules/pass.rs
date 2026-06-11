use crate::dspmodules::dspmodule::{DSPModule, Signal};

pub struct pass {

}
impl DSPModule for pass {
    fn initalize(&mut self) {
        
    }
    fn reset(&mut self) {
        
    }
    fn process(&mut self, signal: Signal<f32>) -> Signal<f32> {
        signal
    }
}