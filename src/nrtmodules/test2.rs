
use crate::dspmodules::{self, gain, number, pass, sampledelay};
use crate::nrtmodules::nrtmodule::NRTModule;
use crate::dspmodules::dspmodule::DSPModule;

#[derive(Debug)]
pub struct NRTTest2 {
    
}
impl NRTTest2 {
    pub const fn new() -> Self {
        Self {  }
    }
}

impl NRTModule for NRTTest2 {
    fn build_dsp(&self) -> Box<dyn DSPModule> {

        gain::Gain::new_boxxed(
            number::Number::new_boxxed(-2.0), 
            number::Number::new_boxxed(0.5)
        )

    }

    fn automate(&self) {
        todo!()
    }
}