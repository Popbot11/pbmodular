use crate::dspmodules::{self, gain, value, pass, sampledelay};
use crate::nrtmodules::nrtmodule::NRTModule;
use crate::dspmodules::dspmodule::{DSPModule, Signal};

use iced::{Element, Renderer, Theme};
use iced::widget::{Column, button, column, row, text};
use crate::Message;
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
    fn build_dsp(&self) -> Box<dyn DSPModule> {

        value::Value::new_boxxed(Signal::Single(0.0)) 

    }
    fn build_ui(&self) -> Column<'_, Message, Theme, Renderer>{
       
            column![
                // button("build dsp").on_press(Message::BuildDSP(todo!())),
                

                "AAAAAAAAAAAAAAAAAAAAAAAA"
            ]
        
    }


}