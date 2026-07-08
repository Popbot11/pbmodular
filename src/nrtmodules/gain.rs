
use std::sync::Arc;

use crate::dspmodules::{self, gain, value, pass, sampledelay};
use crate::nrtmodules::nrtmodule::{NRTConnector, NRTModule};
use crate::dspmodules::dspmodule::DSPModule;

use nice_plug::prelude::Editor;



pub struct Gain {
    input_a: NRTConnector,
    input_b: NRTConnector,
}
impl Gain {
    pub const fn new(input_a: NRTConnector, input_b: NRTConnector) -> Self {
        Self {input_a, input_b}
    }
}

impl NRTModule for Gain {
    fn build_dsp(self: Box<Self>) -> Box<dyn DSPModule> {

        gain::Gain::new_boxxed(
            self.input_a.connect(),
            self.input_b.connect(),
        )

    }
    


}