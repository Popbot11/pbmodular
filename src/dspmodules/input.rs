use crate::dspmodules::dspmodule::{DSPModule, Signal};

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
    fn process(&mut self) -> Signal<f32> {
        Signal::Single(0.0)
    }
    fn process_signal(&mut self, signal: Signal<f32>) -> Signal<f32> {
        signal
    }

    fn dbg_log(&mut self) -> String {
        format!("INPUT")
    }
}