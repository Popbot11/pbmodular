
use crate::dspmodules::{self, gain, value, pass, sampledelay};
use crate::nrtmodules::nrtmodule::NRTModule;
use crate::dspmodules::dspmodule::{DSPModule, Signal};

#[derive(Debug)]
pub struct NRTTest2 {
    
}
impl NRTTest2 {
    pub const fn new() -> Self {
        Self {  }
    }
}

impl NRTModule for NRTTest2 {
    fn build_dsp(self: Box<Self>) -> Box<dyn DSPModule> {

        const POSITIVE: bool = true;

        gain::Gain::new_boxxed(
            if POSITIVE {
                value::Value::new_boxxed(Signal::Single(1.0))           
            } else {
                value::Value::new_boxxed(Signal::Single(-1.0))            
            },

            value::Value::new_boxxed(Signal::Single(0.5))
        )

    }

}