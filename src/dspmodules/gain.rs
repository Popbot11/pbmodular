use crate::dspmodules::dspmodule::{DSPModule, Signal};
pub struct Gain {
    input_a: Box<dyn DSPModule>,
    input_b: Box<dyn DSPModule>,
}
impl Gain {
    pub const fn new(input_a: Box<dyn DSPModule>, input_b: Box<dyn DSPModule>) -> Self {
        Self{
            input_a: input_a,
            input_b: input_b
        }
    }
    pub fn new_boxxed(input_a: Box<dyn DSPModule>, input_b: Box<dyn DSPModule>) -> Box<Self> {
        Box::new(Gain::new(input_a, input_b))
    }
}
impl DSPModule for Gain {
    fn process(&mut self) -> Signal<f32> {
        let input_a = self.input_a.process().unwrap();
        let input_b = self.input_b.process().unwrap();

        let result = Signal::Single(input_a * input_b);
        
        result
    }
}