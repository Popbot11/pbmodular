use std::sync::Arc;

use crate::dspmodules::{self, gain, value};
use crate::nrtmodules::nrtmodule::NRTModule;
use crate::dspmodules::dspmodule::{DSPModule, Signal};

use iced::Element;
use iced::widget::column;
use crate::{Message, PBModular, PBModularParams};

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
    fn build_ui(&self, params: Arc<PBModularParams>) -> Element<'_, Message> {
        column![
            "BLANK"
        ]
        .into()
    }





}