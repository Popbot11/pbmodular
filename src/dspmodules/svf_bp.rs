use std::f32::consts::PI;

use crate::dspmodules::dspmodule::{DSPModule, Signal};



/// doesn't implement anything yet, placeholder :)
pub struct SVF_bp {
    samplerate: f32,

    signal: Box<dyn DSPModule>,
    f: Box<dyn DSPModule>,
    q: Box<dyn DSPModule>,

    z_1: f32,
    z_2: f32,
}
impl SVF_bp {
    pub fn new(
        signal: Box<dyn DSPModule>,
        f: Box<dyn DSPModule>,
        q: Box<dyn DSPModule>,

        samplerate: f32
    ) -> Self {SVF_bp{
        samplerate: samplerate,

        signal: signal,
        f: f,
        q: q,

        z_1: 0.0,
        z_2: 0.0,
    }}

    pub fn new_boxxed(        
        signal: Box<dyn DSPModule>,
        f: Box<dyn DSPModule>,
        q: Box<dyn DSPModule>,

        samplerate: f32
    ) -> Box<Self>{
        Box::new(SVF_bp::new(signal, f,q, samplerate))
    }
    
}
impl DSPModule for SVF_bp {
    fn process(&mut self) -> Signal<f32> {
        
        let signal = self.signal.process().unwrap();
        let f = self.f.process().unwrap();
        let q = self.q.process().unwrap();
        
        // coeffs:
        let g = f32::tan(PI * f / self.samplerate);
        let k = 1.0 / q;

        let a1 = 1.0 / (1.0 + (g * (g + k)));
        let a2 = g * a1;
        let a3 = g * a2;

        // tick:

        let v3 = signal - self.z_2;
        let v1 = (a1 * self.z_1) + (a2 * v3);
        let v2 = self.z_2 + (a2 * self.z_1) + (a3 * v3);
        
        self.z_1 = (2.0 * v1) - self.z_1;
        self.z_2 = (2.0 * v2) - self.z_2;

        Signal::Single(v1)

    }
    fn process_signal(&mut self, signal: Signal<f32>) -> Signal<f32> {
        todo!();
    }

    fn dbg_log(&mut self) -> String {
        format!("SVF_bp [{}]", self.signal.dbg_log())
    }
}