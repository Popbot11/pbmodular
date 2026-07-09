use std::{fmt::Debug, sync::{Arc, Mutex}};


use iced::{Element, Renderer, Theme, wgpu::Label, widget::{Column, button, column, text}};
use nice_plug::{editor::Editor, nice_dbg};
use nice_plug_iced::iced::{core::Widget, program::graphics::color};

use crate::{Message, dspmodules::{dspmodule::{DSPModule, Signal}, input, value}, nrtmodules::nrtmodule};

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
    fn build_ui(&self) -> Column<'_, Message, Theme, Renderer>;
    // fn replace_with(&self, repalcement: Box<dyn NRTModule>);
}


#[derive(Clone)]
pub struct NRTConnector {
    pub inner: Arc<Mutex<NRTConnectorKind>>,
}


pub enum NRTConnectorKind {
    /// corrosponds to a single static value that cannot be changed. 
    Value(Signal<f32>),

    /// corrosponds to a DSP module graph higher up in the chain, likely returned from an NRTModule's `build_dsp()` method  
    Module(Box<dyn NRTModule>),
    // Parameter(todo!()),
    // Buffer(todo!()),

    /// corrosponds to the acive audio input from the plugin host. 
    AudioInput(),
}

impl NRTConnector {
    pub fn value(signal: Signal<f32>) -> NRTConnector {
        NRTConnector {
            inner: Arc::new(Mutex::new(NRTConnectorKind::Value(signal))),
        }
    }

    pub fn module(module: Box<dyn NRTModule>) {
        todo!()
    }
    //etc

    pub fn replace_with_value(&self, signal: Signal<f32>) {
        let mut inner = self.inner.lock().unwrap();
        *inner = NRTConnectorKind::Value(signal);


    }

    pub fn connect(&self) -> Box<dyn DSPModule> {
        let inner = self.inner.lock().unwrap();
        match &*inner {
            NRTConnectorKind::Value(signal) => value::Value::new_boxxed(signal.clone()),
            NRTConnectorKind::AudioInput() => input::Input::new_boxxed(),
            NRTConnectorKind::Module(_module) => value::Value::new_boxxed(Signal::Single(0.0)),
        }
    }

    pub fn connect_ui(&self) -> Column<'_, Message, Theme, Renderer> {
        let connector = self.clone();
        let inner = self.inner.lock().unwrap();

        let body = match &*inner {
            NRTConnectorKind::Value(signal) => {
                column![text(format!("---- VALUE: {}", signal.clone().as_string()))]
            },

            NRTConnectorKind::AudioInput() => {
                column!["---- AUDIO INPUT"]
            },

            NRTConnectorKind::Module(_module) => {
                column!["---- MODULE: (placeholder)"]
            }
        };

        column![
            "-- CONNECTOR:",
            button("replace with value").on_press_with(move || Message::ReplaceWithValue(Arc::new(connector.clone()))),
            body.padding(10.0)
        ]
    }
}


impl NRTConnectorKind {
    pub fn connect(&self) -> Box<dyn DSPModule> {
        match self {
            NRTConnectorKind::Value(signal) => {
                value::Value::new_boxxed(signal.clone())
            },
            NRTConnectorKind::Module(_nrtmodule) => {
                value::Value::new_boxxed(Signal::Single(0.0))
            },
            NRTConnectorKind::AudioInput() => {
                input::Input::new_boxxed()
            },
        }
    }
}

// 

