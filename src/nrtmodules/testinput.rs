
use crate::dspmodules::{self, gain, input, value, pass, sampledelay};
use crate::nrtmodules::nrtmodule::NRTModule;
use crate::dspmodules::dspmodule::DSPModule;
use egui::Ui;
use crate::ui_command::UICommand;

use nice_plug::prelude::Editor;

#[derive(Debug)]
pub struct NRTTestInput {
    
}
impl NRTTestInput {
    pub const fn new() -> Self {
        Self {  } 
    }
}

impl NRTModule for NRTTestInput {
    fn build_dsp(self: Box<Self>) -> Box<dyn DSPModule> {

        input::Input::new_boxxed()

    }
    
    fn build_ui(&self) -> Vec<UICommand>{
        todo!();
    }
}