use std::{rc::Rc, sync::Arc};

use crate::{PBModularParams, Sources, dspmodules::dspmodule::{DSPModule, Signal}};
pub struct Parallel {
    parallelized_module: Box<dyn DSPModule>,
    num_chains: usize,
}


impl Parallel {
    pub fn new(parallelized_module: Box<dyn DSPModule>, num_chains: usize) -> Self {
        Self{
            parallelized_module: parallelized_module,
            num_chains: num_chains
        }
    }

    pub fn new_boxxed(parallelized_module: Box<dyn DSPModule>, num_chains: usize) -> Box<Self> {
        Box::new(Parallel::new(parallelized_module, num_chains))
    }


}
impl DSPModule for Parallel {
    
    fn process(&mut self, sources: &Sources) -> Signal<f32> {
        Signal::Multi(
            (1..self.num_chains).map(|i| 
                self.parallelized_module.process(&sources.with_chains(self.num_chains, i)).unwrap()
            ).collect()
        )
    }



    fn dbg_log(&mut self) -> String {

        // "GAIN MODULE"
        format!(
            "PARALLEl [{} chains, {}]",
            self.num_chains,
            self.parallelized_module.dbg_log(),
        )
    }
}