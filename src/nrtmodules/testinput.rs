
use crate::dspmodules::{self, gain, input, number, pass, sampledelay};
use crate::nrtmodules::nrtmodule::NRTModule;
use crate::dspmodules::dspmodule::DSPModule;

#[derive(Debug)]
pub struct NRTTestInput {
    
}
impl NRTTestInput {
    pub const fn new() -> Self {
        Self {  }
    }
}

impl NRTModule for NRTTestInput {
    fn build_dsp(&self) -> Box<dyn DSPModule> {

        input::Input::new_boxxed()

    }

    fn automate(&self) {
        todo!()
    }
}