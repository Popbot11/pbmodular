
use std::sync::Arc;

use crate::dspmodules::{self, gain, value,};
use crate::nrtmodules::nrtmodule::{NRTConnector, NRTConnectorKind, NRTModule};
use crate::dspmodules::dspmodule::DSPModule;
use iced::widget::column;
use iced::Element;
use crate::{Message, PBModularParams};



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
            self.input_a.connect_dsp(),
            self.input_b.connect_dsp(),
        )

    }
    
    fn build_ui(&self, params: Arc<PBModularParams>) -> Element<'_, Message> {
        column![
            "input a:",
            self.input_a.connect_ui(params.clone()),
            "input b:",
            self.input_b.connect_ui(params.clone()),
        ]
        .into()
    }



}