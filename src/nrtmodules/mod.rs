use std::sync::{Arc, Mutex};

use crate::{dspmodules::{dspmodule::Signal, input}, nrtmodules::{blank::Blank, gain::Gain, nrtmodule::{NRTConnector, NRTConnectorKind, NRTModule}}};

pub mod nrtmodule;

// pub mod test;
// pub mod test2;
// pub mod testinput;


pub mod blank;
pub mod gain;
pub mod parallel;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NRTModuleType {
    Blank,
    Gain,
    // Parallel

}

const NRTMODULE_TYPES: [NRTModuleType; 2] = [
    NRTModuleType::Blank,
    NRTModuleType::Gain,
    // NRTModuleType::Parallel

];

impl std::fmt::Display for NRTModuleType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Blank => "Blank",
            Self::Gain => "Gain",
            // Self::Parallel => "Parallel"

        })
    }
}


impl NRTModuleType {
    fn as_connector(&self) -> NRTConnectorKind{
        match self {

            NRTModuleType::Blank => {
                NRTConnectorKind::Module(Box::new(Blank::new()))
            },

            NRTModuleType::Gain => {
                NRTConnectorKind::Module(Box::new(Gain::new(
                    NRTConnector::value(Signal::Single(0.0)),
                    NRTConnector::value(Signal::Single(0.0))
                )))
            },

            // NRTModuleType::Parallel => {}

        }
    }
}


