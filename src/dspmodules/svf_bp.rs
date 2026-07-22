use std::f32::consts::PI;

use crate::{Sources, dspmodules::dspmodule::{DSPModule, Signal}};



/// doesn't implement anything yet, placeholder :)
pub struct SVF_bp {

    input: Box<dyn DSPModule>,
    f: Box<dyn DSPModule>,
    q: Box<dyn DSPModule>,

    z_1: f32,
    z_2: f32,
}
impl SVF_bp {
    pub fn new(
        input: Box<dyn DSPModule>,
        f: Box<dyn DSPModule>,
        q: Box<dyn DSPModule>,

    ) -> Self {SVF_bp{

        input: input,
        f: f,
        q: q,

        z_1: 0.0,
        z_2: 0.0,
    }}

    pub fn new_boxxed(        
        input: Box<dyn DSPModule>,
        f: Box<dyn DSPModule>,
        q: Box<dyn DSPModule>,

    ) -> Box<Self>{
        Box::new(SVF_bp::new(input, f,q))
    }
    
}
impl DSPModule for SVF_bp {
    fn process(&mut self,sources: &Sources) -> Signal<f32> {
        
        let input = self.input.process(sources).unwrap();

        // TODO: multiplying the f and q inputs because currently parameters are forcibly scaled between 0 and 1. 
        // I need to generally be able to annotate parameter ranges in the sources object so that I don'e need to this. 
        let f = self.f.process(sources).unwrap() * 2000.0;
        let q = self.q.process(sources).unwrap()* 200.0;
        
        // coeffs:
        let g = f32::tan(PI * f / sources.sample_rate);
        let k = 1.0 / q;

        let a1 = 1.0 / (1.0 + (g * (g + k)));
        let a2 = g * a1;
        let a3 = g * a2;

        // tick:

        let v3 = input - self.z_2;
        let v1 = (a1 * self.z_1) + (a2 * v3);
        let v2 = self.z_2 + (a2 * self.z_1) + (a3 * v3);
        let i = sources.current_chain.clone();


        self.z_1 = (2.0 * v1) - self.z_1;
        self.z_2 = (2.0 * v2) - self.z_2;

        Signal::Single(v1)

    }


    fn dbg_log(&mut self) -> String {
        format!("SVF_bp [{}]", self.input.dbg_log())
    }
}