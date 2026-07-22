use std::{rc::Rc, sync::Arc};

use crate::{PBModularParams, Sources, dspmodules::dspmodule::{DSPModule, Signal}};

/// outputs a diferent value depending on the current_chain index of this module instance. 
/// intended to be used in parallel
pub struct MultiRange {
    out_low: Box<dyn DSPModule>,
    out_high: Box<dyn DSPModule>,
}


impl MultiRange {
    pub fn new(
        out_low: Box<dyn DSPModule>,
        out_high: Box<dyn DSPModule>,
    ) -> Self { 
        Self{out_low, out_high} 
    }

    pub fn new_boxxed(
        out_low: Box<dyn DSPModule>,
        out_high: Box<dyn DSPModule>,
    ) -> Box<Self> {
        Box::new(MultiRange::new(out_low, out_high))
    }


}
impl DSPModule for MultiRange {
    
    // TODO: WRITE PROCESS METHOD
    fn process(&mut self, sources: &Sources) -> Signal<f32> {
        // multi_range "scopes out" of the multichannel enclosure, so we need to make sure that higher modules
        // understand that there aren't other chains. 
        let out_low = self.out_low.process(&sources.with_chains(1,1)).unwrap();
        let out_high = self.out_high.process(&sources.with_chains(1,1)).unwrap();
        Signal::Single(

            (
                ((sources.current_chain - 1) as f32) * (out_high - out_low) 
                / ((sources.current_num_chains - 1) as f32)
            ) + out_low
        )
    }



    fn dbg_log(&mut self) -> String {

        // "GAIN MODULE"
        format!(
            "MULTI_RANGE"
        )
    }
}