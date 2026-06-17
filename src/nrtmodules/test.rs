
use crate::dspmodules::{self, gain, number, root, sampledelay};
use crate::nrtmodules::nrtmodule::NRTModule;
use crate::dspmodules::dspmodule::DSPModule;

pub struct NRTTest {
    
}
impl NRTTest {
    pub const fn new() -> Self {
        Self {  }
    }
}

impl NRTModule for NRTTest {
    fn build_dsp(&self) -> Box<dyn DSPModule> {

        gain::Gain::new_boxxed(
            number::Number::new_boxxed(2.0), 
            number::Number::new_boxxed(0.5)
        )

    }

    fn automate(&self) {
        todo!()
    }
}