use std::{rc::Rc, sync::Arc};

use crate::{PBModularParams, Sources, dspmodules::dspmodule::{DSPModule, Signal}};

pub struct Input {

}

impl Input {
    pub fn new() -> Self {
        Self {
            
        }
    }
    pub fn new_boxxed() -> Box<Self>{
        Box::new(Input::new())
    }
}

impl DSPModule for Input {
    fn process(&mut self, sources: &Sources) -> Signal<f32> {
        Signal::Single(sources.input_sample)
    }
    // fn process_signal(&mut self, signal: Signal<f32>) -> Signal<f32> {
    //     signal
    // }

    fn dbg_log(&mut self) -> String {
        format!("INPUT")
    }
}