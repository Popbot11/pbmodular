use crate::dspmodules::dspmodule::{DSPModule, Signal};


/// doesn't implement anything yet, placeholder :)
pub struct Root {

}
impl Root {
    pub const fn new() -> Self {Root{}}
    // pub fn propigate_from() -> Box<dyn DSPModule> {}
}
impl DSPModule for Root {
    fn initalize(&mut self) {
        
    }
    fn reset(&mut self) {
        
    }
    fn process(&mut self, signal: Signal<f32>) -> Signal<f32> {
        signal
    }
}