use std::sync::Arc;

use crate::dspmodules::{self, multi_mixdown, parallel, value,};
use crate::nrtmodules::nrtmodule::{NRTConnector, NRTConnectorKind, NRTModule};
use crate::dspmodules::dspmodule::DSPModule;
use iced::widget::{column, text};
use iced::Element;
use crate::{Message, PBModularParams};



pub struct Parallel {
    parallelized_module: NRTConnector,
    num_chains: usize,
}
impl Parallel {
    pub const fn new(parallelized_module: NRTConnector, num_chains: usize) -> Self {
        Self {parallelized_module, num_chains}
    }
}

impl NRTModule for Parallel {
    fn build_dsp(&self) -> Box<dyn DSPModule> {
        //    TODO: the dsp layout should be as follows: 
        // mixdown <-- parallel <-- some dsp chain <-- buffer. 
        // essencially I need to connect dsp ABOVE the parallelized module that renders the signal as a buffer to refute parallelization higher in the graph.  
        // for now though, all this module does is create clones of everything above it. Which is fine. 
        multi_mixdown::MultiMixdown::new_boxxed(
            parallel::Parallel::new_boxxed(
                self.parallelized_module.connect_dsp(),
                self.num_chains
            ), 
            self.num_chains
        )
        

    }
    
    fn build_ui(&self, params: Arc<PBModularParams>) -> Element<'_, Message> {
        column![
            "parallelized module: ",
            self.parallelized_module.connect_ui(params.clone()),
            text(format!("num chains: {}", self.num_chains))
        ]
        .into()
    }



}