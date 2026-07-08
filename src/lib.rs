use egui::{Margin, Pos2, Rect, Scene, Vec2};
use nice_plug::{prelude::*, wrapper::vst3::vst3::Steinberg::Vst::SampleRate};
use nice_plug_egui::{EguiState, create_egui_editor, resizable_window::ResizableWindow, widgets};
use std::sync::Arc;

use crate::{dspmodules::dspmodule::Signal, nrtmodules::{blank::Blank, gain::Gain, nrtmodule::{NRTConnector, NRTModule}, test::{self, NRTTest}, test2::NRTTest2, testinput::NRTTestInput}, ui_command::{UICommand, render_ui_command}};
use crate::dspmodules::dspmodule::DSPModule;

pub mod dspmodules;
pub mod nrtmodules;
pub mod ui_command;

const MIN_WINDOW_WIDTH: u32 = 300;
const MIN_WINDOW_HEIGHT: u32 = 240;

/// The time it takes for the peak meter to decay by 12 dB after switching to complete silence.
const PEAK_METER_DECAY_MS: f64 = 150.0;

/// If you are using message channels, then allocate enough capacity for the expected worse case
/// scenario for your plugin.
const GUI_TO_AUDIO_MSG_CHANNEL_CAPACITY: usize = 512;
const AUDIO_TO_GUI_MSG_CHANNEL_CAPACITY: usize = 128;

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

    /// A message channel to send events between the GUI and the audio thread.
    ///
    /// This is optional. If you don't need to pass events, you can omit this field.
    msg_channel: AudioMsgChannel,
    /// Used to demonstrate how to pass heap-allocated data from the GUI to the audio thread.
    heap_data_example: Vec<f32>,

    nrtgraph: Box<dyn NRTModule>,

    /// 
    dspgraph: Box<dyn DSPModule>,



    /// State that is synced between the GUI and the audio thread using a triple buffer.
    /// This can be used as an alternative to the message channel approach. Note, the roles of which
    /// thread has the input and which has the output can be reversed.
    ///
    /// The downside to this approach is that it takes 3x the memory.
    ///
    /// This is optional. If you don't need this, you can omit it.
    triple_buffer_state: triple_buffer::Output<TripleBufferState>,

    /// Temporarily hold on to the initial GUI state until the editor is first opened.
    initial_gui_state: Option<GuiState>,
    
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
    #[persist = "editor-state"]
    editor_state: Arc<EguiState>,

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
        
        
        let (to_audio_tx, from_gui_rx) = rtrb::RingBuffer::new(GUI_TO_AUDIO_MSG_CHANNEL_CAPACITY);
        let (to_gui_tx, from_audio_rx) = rtrb::RingBuffer::new(AUDIO_TO_GUI_MSG_CHANNEL_CAPACITY);

        let (triple_buffer_input, triple_buffer_output) =
            triple_buffer::triple_buffer(&TripleBufferState::default());
        
        Self {
            params: Arc::new(PBModularParams::default()),

            peak_meter_decay_weight: 1.0,
            peak_meter: Arc::new(AtomicF32::new(util::MINUS_INFINITY_DB)),

            msg_channel: AudioMsgChannel {
                to_gui_tx,
                from_gui_rx,
                msg_sent: false,
            },
            heap_data_example: Vec::new(),

            nrtgraph: Box::new(Blank::new()),

            dspgraph: Box::new(Blank::new()).build_dsp(),


            triple_buffer_state: triple_buffer_output,

            initial_gui_state: Some(GuiState {
                is_dragging_slider: false,
                msg_channel: GuiMsgChannel {
                    to_audio_tx,
                    from_audio_rx,
                },
                triple_buffer_state: triple_buffer_input,
                next_value: 0,
            })

        }
    }
}

impl Default for PBModularParams {
    fn default() -> Self {
        Self {
            editor_state: EguiState::from_size(MIN_WINDOW_WIDTH, MIN_WINDOW_HEIGHT),

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

/// Here you can store any state you need for your GUI.
///
/// This state persists across editor openings.
pub struct GuiState {
    is_dragging_slider: bool,

    /// A message channel to send events between the GUI and the audio thread.
    ///
    /// This is optional. If you don't need to pass events, you can omit this field.
    msg_channel: GuiMsgChannel,

    /// State that is synced between the GUI and the audio thread using a triple buffer.
    /// This can be used as an alternative the message channel approach. Note, the roles of which
    /// thread has the input and which has the output can be reversed.
    ///
    /// The downside to this approach is that it takes 3x the memory.
    ///
    /// This is optional. If you don't need this, you can omit it.
    triple_buffer_state: triple_buffer::Input<TripleBufferState>,
    next_value: u64,
}

/// A message channel to send events between the GUI and the audio thread.
///
/// This is optional. If you don't need to pass events, you can omit this.
pub struct GuiMsgChannel {
    /// A message channel to send events from the GUI to the audio thread.
    to_audio_tx: rtrb::Producer<GuiToAudioMsg>,
    /// A message channel to receive events from the audio thread.
    from_audio_rx: rtrb::Consumer<AudioToGuiMsg>,
}
/// A message channel to send events between the GUI and the audio thread.
///
/// This is optional. If you don't need to pass events, you can omit this.
pub struct AudioMsgChannel {
    /// A message channel to send events from the audio thread to the GUI thread.
    to_gui_tx: rtrb::Producer<AudioToGuiMsg>,
    /// A message channel to receive events from the GUI thread.
    from_gui_rx: rtrb::Consumer<GuiToAudioMsg>,
    msg_sent: bool,
}


pub enum GuiToAudioMsg {
    MessageA,
    MessageWithHeapData(Vec<f32>),
    RebuildDSP(Box<dyn NRTModule + Send>)
}
#[derive(Debug)]
pub enum AudioToGuiMsg {
    MessageA,
    DropOldHeapData(Vec<f32>),
}

/// State that is synced between the GUI and the audio thread using a triple buffer.
/// This can be used as an alternative the message channel approach. Note, the roles
/// of which thread has the input and which has the output can be reversed.
///
/// The downside to this approach is that it takes 3x the memory.
///
/// This is optional. If you don't need this, you can omit it.
#[derive(Debug, Default, Clone)]
pub struct TripleBufferState {
    value_a: bool,
    value_b: u64,
    some_data: Vec<u32>,

    click_button: bool,
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
        let params = self.params.clone();
        let peak_meter = self.peak_meter.clone();
        let egui_state = params.editor_state.clone();

        create_egui_editor(
            self.params.editor_state.clone(),
            self.initial_gui_state.take().unwrap(),
            Default::default(),
            |_egui_ctx, _queue, _gui_state| {},
            move |ui, setter, _queue, gui_state| {
                ResizableWindow::new("res-wind")
                    .min_size(Vec2::new(MIN_WINDOW_WIDTH as f32, MIN_WINDOW_HEIGHT as f32))
                    .show(ui, egui_state.as_ref(), |ui| {
                        egui::Frame::new()
                            .inner_margin(Margin::same(5))
                            .show(ui, |ui| {
                                
                                // This is a fancy widget that can get all the information it needs to properly
                                // display and modify the parameter from the parametr itself
                                // It's not yet fully implemented, as the text is missing.
                                

                                // ui.label("Gain");
                                // ui.add(widgets::ParamSlider::for_param(&params.gain, setter))

                                ui.add_space(10.0);

                                for i in 0..NUMPARAMSLOTS {
                                    egui::Grid::new(format!("param row {}", i + 1)).show(ui, |ui| {

                                        ui.label(format!("param {}", i + 1));
                                        ui.add(widgets::ParamSlider::for_param(&params.paramslots[i].paramslot , setter));
                                    });
                                }


                                // Demonstrate sending a message to the audio thread.
                                if ui.button("send message").clicked()
                                    && let Err(e) = gui_state
                                        .msg_channel
                                        .to_audio_tx
                                        .push(GuiToAudioMsg::MessageA)
                                {
                                    nice_error!("Failed to send message to audio thread: {}", e);
                                }
                                // Demonstrate receiving messages from the audio thread.
                                while let Ok(msg) = gui_state.msg_channel.from_audio_rx.pop() {
                                    nice_log!("Got message from audio thread: {:?}", &msg);
                                }


                                egui::Grid::new("button_row").show(ui, |ui| {
                                    
                                    
                                    if ui.button("gain at root").clicked() && let Err(e) = gui_state
                                        .msg_channel
                                        .to_audio_tx
                                        .push(GuiToAudioMsg::RebuildDSP(Box::new(
                                            Gain::new(
                                                NRTConnector::AudioInput(),
                                                NRTConnector::Value(Signal::Single(1.0))
                                            )
                                        ))) {
                                        nice_dbg!("replaced dsp graph with testinput module");
                                    }

                                    ui.label(" | ");

                                    if ui.button("awesome button").clicked()  && let Err(e) = gui_state
                                        .msg_channel
                                        .to_audio_tx
                                        .push(GuiToAudioMsg::RebuildDSP(Box::new(NRTTestInput::new()))) {
                                        nice_dbg!("awesome buttton has been clicked. i repeat awesome button has been clicked. ");
                                    }
                                });

                                if ui.button("render ui").clicked() {
                                    nice_dbg!("rendering UI"); 
                                    // render_ui_command(ui,  self.nrtgraph.build_ui());
                                    
                                }

                                // for cmd in self.nrtgraph.build_ui() {
                                //     match cmd {
                                //         UICommand::Label(text) => {ui.label(text); }
                                //     }
                                // }

                                

                                
                            });
                    });
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
        // Demonstrate receiving messages from the GUI thread.
        while let Ok(msg) = self.msg_channel.from_gui_rx.pop() {
            match msg {
                GuiToAudioMsg::MessageA => {
                    nice_dbg!("Got MessageA from GUI");
                    // panic!();

                    // TODO: add "stringifiation" for nrt modules so i can do debug logging for the 
                }

                

                GuiToAudioMsg::MessageWithHeapData(mut heap_data) => {
                    nice_dbg!("Got MessageWithHeapData from GUI");

                    // Replace the old heap data with the new data.
                    std::mem::swap(&mut self.heap_data_example, &mut heap_data);

                    // Note, you must be careful not to drop heap-allocated data on the audio
                    // thread. Send the old data back to the GUI thread to be deallocated there.
                    if let Err(e) = self
                        .msg_channel
                        .to_gui_tx
                        .push(AudioToGuiMsg::DropOldHeapData(heap_data))
                    {
                        nice_error!("Failed to send message to GUI thread: {}", e);
                    }
                }

                GuiToAudioMsg::RebuildDSP(module) => {
                    self.dspgraph = Box::new(module).build_dsp();

                    nice_dbg!(self.dspgraph.dbg_log());
                }
            }
        }

        // Demonstrate sending messages to the GUI thread.
        if self.params.editor_state.is_open() && !self.msg_channel.msg_sent {
            if let Err(e) = self.msg_channel.to_gui_tx.push(AudioToGuiMsg::MessageA) {
                nice_error!("Failed to send message to GUI thread: {}", e);
            }

            // Only send the example message once to avoid spamming the GUI.
            self.msg_channel.msg_sent = true;
        }

        // Demonstrate triple buffer usage.
        let state = self.triple_buffer_state.read();
        // Use the state somehow...
        let _ = &state.value_a;
        let _ = &state.value_b;
        let _ = &state.some_data;

        let click = match &state.click_button {true => 1.0, false => 0.0};

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
            if self.params.editor_state.is_open() {
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
