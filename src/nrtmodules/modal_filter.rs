use std::sync::Arc;

use crate::dspmodules::multi_mixdown::MixdownMode;
use crate::dspmodules::{self, multi_mixdown, multi_parallel, multi_range, svf_bp, value,};
use crate::nrtmodules::modal_filter;
use crate::nrtmodules::nrtmodule::{NRTConnector, NRTConnectorKind, NRTModule};
use crate::dspmodules::dspmodule::DSPModule;
use iced::widget::{column, text};
use iced::Element;
use crate::{Message, PBModularParams};



pub struct ModalFilter {
    input: NRTConnector,

    f_low: NRTConnector,
    f_high: NRTConnector,

    q_low: NRTConnector,
    q_high: NRTConnector,

    num_chains: usize,
}
impl ModalFilter {
    pub const fn new(
        input: NRTConnector,
        
        f_low: NRTConnector,
        f_high: NRTConnector,

        q_low: NRTConnector,
        q_high: NRTConnector,

        num_chains: usize,
    ) -> Self {
        Self {input, f_low, f_high, q_low, q_high, num_chains}
    }
}

impl NRTModule for ModalFilter {
    fn build_dsp(&self) -> Box<dyn DSPModule> {
        
        multi_mixdown::MultiMixdown::new_boxxed(
            multi_parallel::MultiParallel::new_boxxed(
                
                svf_bp::SVF_bp::new_boxxed(

                    self.input.connect_dsp(),

                    multi_range::MultiRange::new_boxxed(
                        self.f_low.connect_dsp(), 
                        self.f_high.connect_dsp(),
                    ),
                    multi_range::MultiRange::new_boxxed(
                        self.q_low.connect_dsp(), 
                        self.q_high.connect_dsp(),
                    ),
                ),
                self.num_chains
            ), 
            self.num_chains,

            MixdownMode::Sqrt
        )
        

    }
    
    fn build_ui(&self, params: Arc<PBModularParams>) -> Element<'_, Message> {
        column![
            "MODAL FILTER",
            text(format!("num chains: {}", self.num_chains)),
            self.input.connect_ui(params.clone()),
            self.f_low.connect_ui(params.clone()),
            self.f_high.connect_ui(params.clone()),
            self.q_low.connect_ui(params.clone()),
            self.q_high.connect_ui(params.clone()),
        ]
        .into()
    }



}