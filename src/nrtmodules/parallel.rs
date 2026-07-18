use std::sync::Arc;

use crate::dspmodules::multi_mixdown::MixdownMode;
use crate::dspmodules::{self, multi_mixdown, multi_parallel, multi_range, value,};
use crate::nrtmodules::nrtmodule::{NRTConnector, NRTConnectorKind, NRTModule};
use crate::dspmodules::dspmodule::DSPModule;
use iced::widget::{column, text};
use iced::Element;
use crate::{Message, PBModularParams};



pub struct Parallel {
    out_low: NRTConnector,
    out_high: NRTConnector,
    num_chains: usize,
}
impl Parallel {
    pub const fn new(out_low: NRTConnector, out_high: NRTConnector, num_chains: usize) -> Self {
        Self {out_low, out_high, num_chains}
    }
}

impl NRTModule for Parallel {
    fn build_dsp(&self) -> Box<dyn DSPModule> {
        //    TODO: the dsp layout should be as follows: 
        // mixdown <-- parallel <-- some dsp chain <-- buffer. 
        // essencially I need to connect dsp ABOVE the parallelized module that renders the signal as a buffer to refute parallelization higher in the graph.  
        // for now though, all this module does is create clones of everything above it. Which is fine. 
        // UPDATE: this is corrently SORTA happening with the multi_range module, which resets the dsp info to have one parallel chain. but doesn't
        // actually enforce that in any way. I need to write a lot more boilerplate before parallelization works fully. 
        // At this point a lot of stuff is kinda hardcoded and patched together. which is fine!   

        multi_mixdown::MultiMixdown::new_boxxed(
            multi_parallel::MultiParallel::new_boxxed(
                multi_range::MultiRange::new_boxxed(
                    self.out_low.connect_dsp(), 
                    self.out_high.connect_dsp()
                ),
                self.num_chains
            ), 
            self.num_chains,

            //TODO: have mixdown mide as an actual input; for now im just hardcoding add as the behaviour so that 
            // shit is easier to test. 
            MixdownMode::Add
        )
        

    }
    
    fn build_ui(&self, params: Arc<PBModularParams>) -> Element<'_, Message> {
        column![
            "parallelized module: ",
            self.out_high.connect_ui(params.clone()),
            self.out_low.connect_ui(params.clone()),
            text(format!("num chains: {}", self.num_chains))
        ]
        .into()
    }



}