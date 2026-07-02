use std::fmt::Debug;

use nice_plug::editor::Editor;

use crate::dspmodules::{dspmodule::{DSPModule, Signal}, value};

/// a nrtmodule, or non-realtime module is the much bulkier sibling to the dspmodule. 
/// structs that implement nrtmodule are representations of everything that the user will interface with, 
/// and vitally contain instructions for how to construct networks of dspmodules. 
/// 
/// more specifically, nrtmodules contain information about user interface, plugin paremeter mapping, and signal processing. 
/// 
/// for example, a modal filter nrtmodule would contain instructions on how to build a modal filter out of dspmodules, 
/// details on how the particular modal filter UI is rendered, and information about controllable parameters. 
pub trait NRTModule {
    fn build_dsp(self: Box<Self>) -> Box<dyn DSPModule>;
    // fn build_editor(&self) -> Option<Box<dyn Editor>>;

}

pub enum NRTConnector {
    /// corrosponds to a single static value that cannot be changed. 
    Value(Signal<f32>),

    /// corrosponds to a DSP module graph higher up in the chain, likely returned from an NRTModule's `build_dsp()` method  
    Module(Box<dyn DSPModule>),
    // Parameter(todo!()),
    // Buffer(todo!()),

    /// corrosponds to an acive audio input from the plugin host. 
    AudioInput(f32),
}

impl NRTConnector {
    pub fn connect(self) -> Box<dyn DSPModule>{
        match self {
            NRTConnector::Value(signal) => {
                value::Value::new_boxxed(signal)
            },

            NRTConnector::Module(dspmodule) => {
                dspmodule
            },
            NRTConnector::AudioInput(_) => todo!(),
        }
    }
}

// 

