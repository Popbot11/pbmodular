use crate::dspmodules::dspmodule::{DSPModule, Signal};
use std::mem;
/// a single sample delay. keeps track of a single value, which is the sample being delayed. 
/// 
/// accepts Single, outputs Single 
pub struct SampleDelay {
    input: Box<dyn DSPModule>,
    
    s1: Signal<f32>,
}
impl SampleDelay {
    pub fn new(input: Box<dyn DSPModule>) -> Self {
        Self{
            input,
            s1: Signal::Single(0.0)
        }
    }
    pub fn new_boxxed(input: Box<dyn DSPModule>) -> Box<Self> {
        Box::new(SampleDelay::new(input))
    }
}
impl DSPModule for SampleDelay{
    fn process(&mut self) -> Signal<f32> {
        let signal = self.input.process();
        // self.s1 = signal;
        let result = mem::replace(&mut self.s1, signal);
        result
    }


    fn dbg_log(&mut self) -> String {
        format!("SAMPLEDELAY [{}]", self.input.dbg_log())
    }
}
