pub trait DSPModule: Send + Sync {
   

    fn process(&mut self) -> Signal<f32>;
    fn process_signal(&mut self, signal: Signal<f32>) -> Signal<f32>;

    fn dbg_log(&mut self) -> String;
}



/// this is what contains all dsp signals accross the dsp graph. A signal can either be a singular value, or a vector of values. 
/// the exact usage of a signal depends entirely on the particular module. 
/// 
/// TODO: have `Single(&mut Buffer<T>)` and `Multi(Vec<$mut Buffer<T>>)` so that I can do more routing stuff.
#[derive(Debug, Clone)]
pub enum Signal<T> {
    None(()),
    Single(T),
    Multi(Vec<T>)
}
impl<T> Signal<T> {
    pub fn unwrap(self) -> T {
        match self {
            Signal::None(v) => panic!("called `Signal::unwrap()` on a None containing ()"), 
            Signal::Single(v) => v,
            Signal::Multi(v) => {
                if v.len() == 1 {
                    v.into_iter().next().unwrap()
                } else {
                    panic!("called `Signal::unwrap()` on a Multi containing {} elements", v.len())
                }
            }
        }
    }

    pub fn unwrap_multi(self) -> Vec<T> {
        match self {
            Signal::None(v) => panic!("called `Signal::unwrap_multi()` on a None containing ()"),
            Signal::Single(v) => vec![v],
            Signal::Multi(v) => v,
        }
    }

    pub fn as_string(self) -> String
    where
        T: std::fmt::Debug,
    {
        match self {
            Signal::None(_) => "()".to_string(),
            Signal::Single(v) => format!("Single<{:?}>", v),
            Signal::Multi(v) => format!("Multi<{:?}>", v),
        }
    }
}

impl<T> From<T> for Signal<T> {
    fn from(value: T) -> Self {
        Signal::Single(value)
    }
}

