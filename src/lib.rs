
use nice_plug::{prelude::*, wrapper::vst3::vst3::Steinberg::Vst::SampleRate};
use nice_plug_iced::iced::{
    self, Center, PollSubNotifier, Theme,
    widget::{Column, ProgressBar, button, column, slider, text},
};
use nice_plug_iced::{EditorState, NiceGuiContext, WindowState, application, create_iced_editor};
use std::sync::{Arc, atomic::Ordering};

use crate::{dspmodules::dspmodule::Signal, nrtmodules::{blank::Blank, gain::Gain, nrtmodule::{NRTConnector, NRTModule}, test::{self, NRTTest}, test2::NRTTest2, testinput::NRTTestInput}};
use crate::dspmodules::dspmodule::DSPModule;

pub mod dspmodules;
pub mod nrtmodules;


const WINDOW_WIDTH: u32 = 300;
const WINDOW_HEIGHT: u32 = 240;

/// The time it takes for the peak meter to decay by 12 dB after switching to complete silence.
const PEAK_METER_DECAY_MS: f64 = 150.0;



const NUMPARAMSLOTS: usize = 4;

pub struct PBModular {
    params: Arc<PBModularParams>,

    /// Needed to normalize the peak meter's response based on the sample rate.
    peak_meter_decay_weight: f32,
    /// The current data for the peak meter. This is stored as an [`Arc`] so we can share it between
    /// the GUI and the audio processing parts. If you have more state to share, then it's a good
    /// idea to put all of that in a struct behind a single `Arc`.
    ///
    /// This is stored as voltage gain.
    peak_meter: Arc<AtomicF32>,

    /// An atomic flag used to notify the program when it should poll for new updates
    /// and redraw (i.e. as a result of the host updating parameters or the audio thread
    /// updating the state of meters). This flag is polled every frame right before
    /// drawing. If the flag is set then the [`poll_events`] subscription will be called, and
    /// the program will update and redraw.
    notifier: PollSubNotifier,

    nrtgraph: Box<dyn NRTModule>,

    /// 
    dspgraph: Box<dyn DSPModule>,


    
}

#[derive(Params)]
struct ParamSlot {
    #[id = "Parameter Slot"]
    pub paramslot: FloatParam,
}


#[derive(Params)]
pub struct PBModularParams {
    /// The editor state, saved together with the parameter state so the custom scaling can be
    /// restored.
    #[persist = "window-state"]
    window_state: Arc<WindowState>,

    #[nested(array, group = "paramslot")]
    pub paramslots: [ParamSlot; NUMPARAMSLOTS],

    #[id = "gain"]
    pub gain: FloatParam,

    // TODO: Remove this parameter when we're done implementing the widgets
    #[id = "foobar"]
    pub some_int: IntParam,
}

impl Default for PBModular {
    fn default() -> Self {
        

        
        Self {
            params: Arc::new(PBModularParams::default()),

            peak_meter_decay_weight: 1.0,
            peak_meter: Arc::new(AtomicF32::new(util::MINUS_INFINITY_DB)),

            notifier: PollSubNotifier::new(),

            nrtgraph: Box::new(Blank::new()),

            dspgraph: Box::new(Blank::new()).build_dsp(),


        }
    }
}

impl Default for PBModularParams {
    fn default() -> Self {
        Self {
            window_state: WindowState::from_logical_size(WINDOW_WIDTH, WINDOW_HEIGHT),

            // See the main gain example for more details
            gain: FloatParam::new(
                "Gain",
                util::db_to_gain(0.0),
                FloatRange::Skewed {
                    min: util::db_to_gain(-30.0),
                    max: util::db_to_gain(30.0),
                    factor: FloatRange::gain_skew_factor(-30.0, 30.0),
                },
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0))
            .with_unit(" dB")
            .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
            .with_string_to_value(formatters::s2v_f32_gain_to_db()),
            some_int: IntParam::new("Something", 3, IntRange::Linear { min: 0, max: 3 }),

            // create all parameter slots

            paramslots: std::array::from_fn(|i| ParamSlot {
                paramslot: FloatParam::new(
                    format!("param {}", i + 1),
                    0.5, 
                    FloatRange::Linear { min: 0.0, max: 1.0 }
                )
            })
        }
    }
}




impl Plugin for PBModular {
    const NAME: &'static str = "PBMODULAR (prototype)";
    const VENDOR: &'static str = "SUPERPLUGINS";
    const URL: &'static str = "https://popbot.work/";
    const EMAIL: &'static str = "info@example.com";

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(2),
            main_output_channels: NonZeroU32::new(2),
            ..AudioIOLayout::const_default()
        },
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(1),
            main_output_channels: NonZeroU32::new(1),
            ..AudioIOLayout::const_default()
        },
    ];

    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        create_iced_editor(
            self.params.window_state.clone(),
            MyEditorState {
                params: self.params.clone(),
                peak_meter: self.peak_meter.clone(),
            },

            self.notifier.clone(),
            Default::default(),
            |editor_state, nice_ctx| {
                application(
                    editor_state,
                    nice_ctx,
                    MyGui::new,
                    MyGui::update,
                    MyGui::view,
                )
                .theme(MyGui::theme)
                // Subscribe to the poller which detects when the application should poll
                // parameters/meters and redraw.
                .subscription(|_| iced::poll_events().map(|_| Message::Poll))
                .run()
            },
        )
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        // After `PEAK_METER_DECAY_MS` milliseconds of pure silence, the peak meter's value should
        // have dropped by 12 dB
        self.peak_meter_decay_weight = 0.25f64
            .powf((buffer_config.sample_rate as f64 * PEAK_METER_DECAY_MS / 1000.0).recip())
            as f32;

        true
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        
        // ideas for how to pull this bullshit off:
        //     - have a store of buffers initalized with the plugin state thats written to directly in the process loop, and require a reference to the buffer store be passed in as an argument 
        //     - have the `Signal` type be able to store a reference to an external buffer  
        //     - have a .into_dsp() method on the signal type that just returns a dspmodule object 
        //     - 

        for channel_samples in buffer.iter_samples() {
            let mut amplitude = 0.0;
            let num_samples = channel_samples.len();

            let gain = self.params.paramslots[0].paramslot.smoothed.next();


            for sample in channel_samples {

                *sample = self.dspgraph.process_signal(
                    Signal::Single(*sample)
                ).unwrap();

            }

            // To save resources, a plugin can (and probably should!) only perform expensive
            // calculations that are only displayed on the GUI while the GUI is open
            if self.params.window_state.is_open() {
                amplitude = (amplitude / num_samples as f32).abs();
                let current_peak_meter = self.peak_meter.load(std::sync::atomic::Ordering::Relaxed);
                let new_peak_meter = if amplitude > current_peak_meter {
                    amplitude
                } else {
                    current_peak_meter * self.peak_meter_decay_weight
                        + amplitude * (1.0 - self.peak_meter_decay_weight)
                };

                self.peak_meter
                    .store(new_peak_meter, std::sync::atomic::Ordering::Relaxed)
            }
        }

        ProcessStatus::Normal
    }
    
    const MIDI_INPUT: MidiConfig = MidiConfig::None;
    
    const MIDI_OUTPUT: MidiConfig = MidiConfig::None;
    
    const HARD_REALTIME_ONLY: bool = false;
    
    fn task_executor(&mut self) -> TaskExecutor<Self> {
        // In the default implementation we can simply ignore the value
        Box::new(|_| ())
    }
    
    fn filter_state(state: &mut PluginState) {}
    
    fn reset(&mut self) {}
    
    fn deactivate(&mut self) {}
}

#[derive(Debug, Clone, Copy)]
enum Message {
    /// Sent when the application should poll parameters/meters and redraw.
    Poll,
    Increment,
    Decrement,
    GainChanged(f32),
}


struct MyEditorState {
    params: Arc<PBModularParams>,
    peak_meter: Arc<AtomicF32>,
}


struct MyGui {
    /// The editor state is stored inside of a wrapper which allows the
    /// state to persist across editor opens.
    editor_state: EditorState<MyEditorState>,

    /// A handle that can be used to request operations from nice-plug, like
    /// resizing the window.
    #[allow(unused)]
    nice_ctx: NiceGuiContext,

    value: i64,
    peak_meter_db: f32,
}

impl MyGui {
    pub fn new(editor_state: EditorState<MyEditorState>, nice_ctx: NiceGuiContext) -> Self {
        Self {
            editor_state,
            nice_ctx,
            value: 0,
            peak_meter_db: nice_plug::util::gain_to_db(0.0),
        }
    }

    pub fn theme(&self) -> Option<Theme> {
        Some(Theme::Dark)
    }

    pub fn update(&mut self, message: Message) {
        let setter = self.nice_ctx.param_setter();
        let params = &self.editor_state.params;

        match message {
            Message::Poll => {
                self.peak_meter_db = nice_plug::util::gain_to_db(
                    self.editor_state.peak_meter.load(Ordering::Relaxed),
                );
            }
            Message::Increment => {
                self.value += 1;
            }
            Message::Decrement => {
                self.value -= 1;
            }
            Message::GainChanged(value) => {
                // TODO: Add generic slider widget
                setter.begin_set_parameter(&params.gain);
                setter.set_parameter_normalized(&params.gain, value);
                setter.end_set_parameter(&params.gain);
            }
        }
    }

    pub fn view(&self) -> Column<'_, Message> {
        let params = &self.editor_state.params;

        column![
            button("Increment").on_press(Message::Increment),
            text(self.value).size(30),
            button("Decrement").on_press(Message::Decrement),
            // TODO: Add generic slider widget
            slider(
                0.0..=1.0,
                params.gain.modulated_normalized_value(),
                Message::GainChanged
            )
            .step(0.001f32),
            text(
                params
                    .gain
                    .normalized_value_to_string(params.gain.modulated_normalized_value(), true)
            ),
            ProgressBar::new(-80.0..=0.0, self.peak_meter_db),
        ]
        .padding(20)
        .spacing(12.0)
        .align_x(Center)
    }
}

impl ClapPlugin for PBModular {
    const CLAP_ID: &'static str = "com.moist-plugins-gmbh-egui.nice-plug-gain-egui";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("a prototype for my modular plugin");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[
        ClapFeature::AudioEffect,
        ClapFeature::Stereo,
        ClapFeature::Mono,
        ClapFeature::Utility,
    ];
}

impl Vst3Plugin for PBModular {
    const VST3_CLASS_ID: [u8; 16] = *b"GainGuiYeahBoyyy";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Fx, Vst3SubCategory::Tools];
}

nice_export_clap!(PBModular);
nice_export_vst3!(PBModular);
