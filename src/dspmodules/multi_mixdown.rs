use core::num;
use std::{rc::Rc, result, sync::Arc};

use iced::wgpu::naga::MathFunction::Mix;
use nice_plug::nice_dbg;

use crate::{PBModularParams, Sources, dspmodules::dspmodule::{DSPModule, Signal}};

// TODO: implement diferent mixdown modes that combine mc signal diferently. Currently i'm just going to do the sqrt mixdown 
pub enum MixdownMode {
    /// divide output by the square root of the number of chains
    Sqrt, 

    /// simply add each channel together with no other logic
    Add,
}

pub struct MultiMixdown {
    /// module that returns a signal of the multi variant 
    multi_signal: Box<dyn DSPModule>,
    
    /// the assumed number of channels. This module is able to work with dynamically sized mc signals, 
    /// but this initalized value is used by default to save resources
    num_chains: usize,

    /// precomputed sqrt of the number of chains
    sqrt_num_chains: f32,

    mixdown_mode: MixdownMode
}

impl MultiMixdown {
    pub fn new(multi_signal: Box<dyn DSPModule>, num_chains: usize, mixdown_mode: MixdownMode) -> Self {
        Self {
            
            multi_signal: multi_signal,
            num_chains: num_chains,
            
            sqrt_num_chains: match mixdown_mode {
                MixdownMode::Sqrt => {
                    (num_chains as f32).sqrt()
                }
                _ =>  0.0,
            },
            
            mixdown_mode: mixdown_mode,
        }
    }
    pub fn new_boxxed(multi_signal: Box<dyn DSPModule>, num_chains: usize, mixdown_mode: MixdownMode) -> Box<Self>{
        Box::new(MultiMixdown::new(multi_signal, num_chains, mixdown_mode))
    }
}

impl DSPModule for MultiMixdown {
    fn process(&mut self, sources: &Sources) -> Signal<f32> {
        let input = self.multi_signal.process(sources);

        // nice_dbg!(input.clone().as_string());
        
        Signal::Single( match input {
            Signal::Multi(signal) => {
                match self.mixdown_mode {
                    MixdownMode::Add => {
                        
                        signal.iter().sum::<f32>()
                    },
                    MixdownMode::Sqrt => {
                        // avoid computing square root unless the actual number of channels is not the precomputer amt. 
                        let actual_num_chains = signal.len();
                        if self.num_chains != actual_num_chains {
                            self.sqrt_num_chains = (actual_num_chains as f32).sqrt();
                        }
                        
                        signal.iter().sum::<f32>() / self.sqrt_num_chains

                    }
                }
            }
            Signal::Single(signal) => signal,
            Signal::None(_) => 0.0,
        })
    }
    // fn process_signal(&mut self, signal: Signal<f32>) -> Signal<f32> {
    //     signal
    // }

    fn dbg_log(&mut self) -> String {
        format!("INPUT")
    }
}