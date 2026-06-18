use crate::dspmodules::{self, gain, number, pass, sampledelay};
use crate::nrtmodules::nrtmodule::NRTModule;
use crate::dspmodules::dspmodule::DSPModule;

#[derive(Debug)]
pub struct Blank {
    
}
impl Blank {
    pub const fn new() -> Self {
        Self {  }
    }
}

impl NRTModule for Blank {
    fn build_dsp(&self) -> Box<dyn DSPModule> {

        number::Number::new_boxxed(0.0) 

    }

    fn automate(&self) {
        todo!()
    }
}