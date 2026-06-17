use crate::dspmodules::sampledelay::SampleDelay;
use crate::nrtmodules::nrtmodule::NRTModule;
use crate::dspmodules::dspmodule::DSPModule;

pub struct Test {
    
}

impl NRTModule for Test {
    fn build_dsp(&self) -> Box<dyn DSPModule> {
        SampleDelay::new_boxxed()
    }

    fn automate(&self) {
        todo!()
    }
}