use crate::dspmodules::dspmodule::{DSPModule, Signal};
pub struct Number {
    value: f32
}
impl Number {
    pub const fn new(value: f32) -> Self {
        Self{
            value: value
        }
    }
    pub fn new_boxxed(value: f32) -> Box<Self> {
        Box::new(Number::new(value))
    }
    // pub fn propigate_from() -> Box<dyn DSPModule> {}
}
impl DSPModule for Number {
    fn process(&mut self) -> Signal<f32> {
        Signal::Single(self.value)
    }
}