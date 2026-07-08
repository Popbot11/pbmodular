
use crate::dspmodules::{self, gain, value, pass, sampledelay};
use crate::nrtmodules::nrtmodule::NRTModule;
use crate::dspmodules::dspmodule::{DSPModule, Signal};
use egui::Ui;
use nice_plug::prelude::Editor;
use crate::ui_command::UICommand;


#[derive(Debug)]
pub struct NRTTest {
    
}
impl NRTTest {
    pub const fn new() -> Self {
        Self {  }
    }
}

impl NRTModule for NRTTest {
    fn build_dsp(self: Box<Self>) -> Box<dyn DSPModule> {

        

        gain::Gain::new_boxxed(            
            value::Value::new_boxxed(Signal::Single(2.0)) ,
            
            value::Value::new_boxxed(Signal::Single(0.5))
        )

    }
    
    fn build_ui(&self) -> Vec<UICommand>{
        todo!();
    }

}