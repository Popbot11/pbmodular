use std::{fmt::Debug, sync::{Arc, Mutex}};


use iced::{Element, Renderer, Theme, wgpu::Label, widget::{Column, button, column, pick_list, row, text}};
use nice_plug::{editor::Editor, nice_dbg};
use nice_plug_iced::iced::{core::Widget, program::graphics::color};

use crate::{Message, dspmodules::{dspmodule::{DSPModule, Signal}, input, param, value}, nrtmodules::{NRTMODULE_TYPES, NRTModuleType::{self}, gain::Gain}};

/// a nrtmodule, or non-realtime module is the much bulkier sibling to the dspmodule. 
/// structs that implement nrtmodule are representations of everything that the user will interface with, 
/// and vitally contain instructions for how to construct networks of dspmodules. 
/// 
/// more specifically, nrtmodules contain information about user interface, plugin paremeter mapping, and signal processing. 
/// 
/// for example, a modal filter nrtmodule would contain instructions on how to build a modal filter out of dspmodules, 
/// details on how the particular modal filter UI is rendered, and information about controllable parameters. 
pub trait NRTModule: Send + Sync  {
    fn build_dsp(&self) -> Box<dyn DSPModule>;
    fn build_ui(&self) -> Element<'_, Message>;
}



#[derive(Clone)]
pub struct NRTConnector {
    pub inner: Arc<Mutex<NRTConnectorKind>>,
}

pub enum NRTConnectorKind {
    /// corrosponds to a single static value that cannot be changed. 
    Value(Signal<f32>),

    /// corrosponds to a NRT module graph higher up in the chain, likely returned from an NRTModule's `build_dsp()` method  
    Module(Box<dyn NRTModule>),
    
    /// corrosponds to a user editable parameter 
    Parameter(usize),

    /// corrosponds to the acive audio input from the plugin host. 
    AudioInput,
}

impl NRTConnector {


    /// Returns an NRTConnector of the Value type
    pub fn value(signal: Signal<f32>) -> NRTConnector {
        NRTConnector {
            inner: Arc::new(Mutex::new(NRTConnectorKind::Value(signal))),
        }
    }

    /// Returns an NRTConnector of the Module type
    pub fn module(module: Box<dyn NRTModule>) -> NRTConnector {
        NRTConnector {
            inner: Arc::new(Mutex::new(NRTConnectorKind::Module(module))),
        }
    }

    /// replaces this NRTConnector with one of the Value type
    pub fn replace_with_value(&self, signal: Signal<f32>) {
        let mut inner = self.inner.lock().unwrap();
        *inner = NRTConnectorKind::Value(signal);
    }

    /// Replaces this NRTConnector with one of the Module type
    pub fn replace_with_module(&self, connector: NRTConnectorKind) {
        let mut inner = self.inner.lock().unwrap();

        *inner = connector;
    }

    /// returns (but does not evaluate) the DSP chain of whatever is above this connetor 
    pub fn connect_dsp(&self) -> Box<dyn DSPModule> {
        let inner = self.inner.lock().unwrap();
        match &*inner {
            NRTConnectorKind::Value(signal) => value::Value::new_boxxed(signal.clone()),
            NRTConnectorKind::AudioInput => input::Input::new_boxxed(),
            NRTConnectorKind::Module(module) => module.build_dsp(),
            NRTConnectorKind::Parameter(slot) => param::Param::new_boxxed(*slot),
        }
    }

    /// returns GUI information about this particular connector. 
    pub fn connect_ui(&self) -> Column<'_, Message, Theme, Renderer> {
        let mut selected_module = NRTModuleType::Blank;

        let body =  {
            let inner = &*(self.inner.lock().unwrap());

            match inner {
                NRTConnectorKind::Value(signal) => {
                    column![text(format!("---- VALUE: {}", signal.clone().as_string()))]
                }
                NRTConnectorKind::AudioInput => {
                    column!["---- AUDIO INPUT"]
                }
                NRTConnectorKind::Module(module) => {
                    column!["---- MODULE"]
                }
                NRTConnectorKind::Parameter(slot) => {
                    column!["---- PUT A SLIDER HERE!!!!"]
                }
            }
        };

        column![
            "-- CONNECTOR:",
            // UI of this particular connector type
            body.padding(10.0), 
            
            // row of replacement options
            row![
                "replace with...",
                pick_list(
                    NRTMODULE_TYPES,
                    Some(selected_module),
                    move |module| Message::ReplaceConnector(
                        Arc::new(self.clone()),
                        Arc::new(module.as_connector()),
                    )
                ).placeholder("module"),

                button("value").on_press(Message::ReplaceConnector(
                    Arc::new(self.clone()), 
                    Arc::new(NRTConnectorKind::Value(Signal::Single(1.0)))
                )),

                button("input").on_press(Message::ReplaceConnector(
                    Arc::new(self.clone()), 
                    Arc::new(NRTConnectorKind::AudioInput)
                )),

                button("param").on_press(Message::ReplaceConnector(
                    Arc::new(self.clone()), 
                    Arc::new(NRTConnectorKind::Parameter(0))
                )),
            ],
            // dropdown of modules this connector can be replaced with
            
        ]
    }

    pub fn as_string(&self) -> String {
        match &*(self.inner.lock().unwrap()) {
            NRTConnectorKind::AudioInput => format!("audio input"),
            NRTConnectorKind::Value(value) => value.clone().as_string(),
            NRTConnectorKind::Module(module) => format!("module"),
            NRTConnectorKind::Parameter(slot) => format!("param: {}", slot)
        }
    }
}


impl std::fmt::Debug for NRTConnectorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Value(arg0) => f.debug_tuple("Value").field(arg0).finish(),
            Self::Module(arg0) => f.debug_tuple("Module").finish(),
            Self::AudioInput => f.debug_tuple("AudioInput").finish(),
            Self::Parameter(slot) => f.debug_tuple("Parameter").finish()
        }
    }
}

//

