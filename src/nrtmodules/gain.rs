
use std::sync::Arc;

use crate::dspmodules::{self, gain, value, pass, sampledelay};
use crate::nrtmodules::nrtmodule::{NRTConnector, NRTConnectorKind, NRTModule};
use crate::dspmodules::dspmodule::DSPModule;
use iced::widget::{Column, column, container};
use iced::{Element, Renderer, Theme};
use crate::Message;
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
    fn build_dsp(&self) -> Box<dyn DSPModule> {

        gain::Gain::new_boxxed(
            self.input_a.connect(),
            self.input_b.connect(),
        )

    }
    
    fn build_ui(&self) -> Column<'_, Message, Theme, Renderer>{
        column![
            "input a:",
            self.input_a.connect_ui(),
            "input b:",
            self.input_b.connect_ui()
        ]
    }


}