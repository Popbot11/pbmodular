use crate::dspmodules::{self, gain, value, pass, sampledelay};
use crate::nrtmodules::nrtmodule::NRTModule;
use crate::dspmodules::dspmodule::{DSPModule, Signal};


use nice_plug::prelude::Editor;

#[derive(Debug)]
pub struct Blank {
    
}
impl Blank {
    pub const fn new() -> Self {
        Self {  }
    }
}

impl NRTModule for Blank {
    fn build_dsp(self: Box<Self>) -> Box<dyn DSPModule> {

        value::Value::new_boxxed(Signal::Single(0.0)) 

    }
    

}