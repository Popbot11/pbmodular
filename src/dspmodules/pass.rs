use crate::dspmodules::dspmodule::{DSPModule, Signal};


/// doesn't implement anything yet, placeholder :)
pub struct Pass {
    signal: Box<dyn DSPModule>
}
impl Pass {
    pub fn new(signal: Box<dyn DSPModule>) -> Self {Pass{
        signal: signal
    }}
    // pub fn propigate_from() -> Box<dyn DSPModule> {}
}
impl DSPModule for Pass {
    fn process(&mut self) -> Signal<f32> {
        let signal = self.signal.process();
        signal
    }
    fn process_signal(&mut self, signal: Signal<f32>) -> Signal<f32> {
        todo!()
    }
}