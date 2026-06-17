use crate::dspmodules::dspmodule::{DSPModule, Signal};


/// doesn't implement anything yet, placeholder :)
pub struct Root {
    signal: Box<dyn DSPModule>
}
impl Root {
    pub const fn new(signal: Box<dyn DSPModule>) -> Self {Root{
        signal: signal
    }}
    // pub fn propigate_from() -> Box<dyn DSPModule> {}
}
impl DSPModule for Root {
    fn process(&mut self) -> Signal<f32> {
        let signal = self.signal.process();
        signal
    }
}