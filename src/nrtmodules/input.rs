
use crate::dspmodules::pass::Pass;
use crate::dspmodules::{self, gain, number, pass, sampledelay};
use crate::nrtmodules::nrtmodule::NRTModule;
use crate::dspmodules::dspmodule::DSPModule;


// TODO: make this able to take in an actual sample value and pass it to the chain of dspmodules within. 
// generally connect up the nrtmodules, or have a wway for nrtmodules to see connections and then bind dspmodules based on those connections.  
#[derive(Debug)]
pub struct Input {
    input_dsp: Box<dyn DSPModule>
}
impl Input {
    pub fn new(input: Box<dyn DSPModule>) -> Self {
        Self {input_dsp: input }
    }
}

impl NRTModule for Input {
    fn build_dsp(&self) -> Box<dyn DSPModule> {

        todo!()

    }

    fn automate(&self) {
        todo!()
    }
}