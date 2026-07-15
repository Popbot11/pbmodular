use std::{rc::Rc, sync::Arc};

use crate::{PBModularParams, Sources, dspmodules::dspmodule::{DSPModule, Signal}};

pub struct Param {
    slot: usize,
}

impl Param {
    pub fn new(slot: usize) -> Self {
        Self {
            slot: slot
        }
    }
    pub fn new_boxxed(slot: usize) -> Box<Self>{
        Box::new(Param::new(slot))
    }
}

impl DSPModule for Param {
    fn process(&mut self, sources: &Sources) -> Signal<f32> {
        Signal::Single(
            sources.params.paramslots[self.slot].paramslot.smoothed.next()
        )
    }


    fn dbg_log(&mut self) -> String {
        format!("PARAM: {}", self.slot)
    }
}