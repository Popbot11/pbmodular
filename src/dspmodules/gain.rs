use std::{rc::Rc, sync::Arc};

use crate::{PBModularParams, Sources, dspmodules::dspmodule::{DSPModule, Signal}};
pub struct Gain {
    input_a: Box<dyn DSPModule>,
    input_b: Box<dyn DSPModule>,
}

impl Gain {
    pub fn new(input_a: Box<dyn DSPModule>, input_b: Box<dyn DSPModule>) -> Self {
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
    
    fn process(&mut self, sources: &Sources) -> Signal<f32> {
        let input_a = self.input_a.process(sources).unwrap();
        let input_b = self.input_b.process(sources).unwrap();

        let result = Signal::Single(input_a * input_b);
        
        result
    }



    fn dbg_log(&mut self) -> String {

        // "GAIN MODULE"
        format!(
            "GAIN [{}, {}]",
            self.input_a.dbg_log(),
            self.input_b.dbg_log()
        )
    }
}