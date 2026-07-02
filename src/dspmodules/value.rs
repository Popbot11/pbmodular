use crate::dspmodules::dspmodule::{DSPModule, Signal};
pub struct Value {
    value: Signal<f32>
}
impl Value {
    pub const fn new(value: Signal<f32>) -> Self {
        Self{
            value: value
        }
    }
    pub fn new_boxxed(value: Signal<f32>) -> Box<Self> {
        Box::new(Value::new(value))
    }
    // pub fn propigate_from() -> Box<dyn DSPModule> {}
}
impl DSPModule for Value {
    fn process(&mut self) -> Signal<f32> {
        self.value.clone()
    }
    fn process_signal(&mut self, signal: Signal<f32>) -> Signal<f32> {
        self.process()
    }

    fn dbg_log(&mut self) -> String {
       format!("NUMBER: {}", self.value.clone().unwrap())
    }
}