/// a nrtmodule, or non-realtime module is the much bulkier sibling to the dspmodule. 
/// structs that implement nrtmodule are representations of everything that the user will interface with, 
/// and vitally contain instructions for how to construct networks of dspmodules. 
/// 
/// more specifically, nrtmodules contain information about user interface, plugin paremeter mapping, and signal processing. 
/// 
/// for example, a modal filter nrtmodule would contain instructions on how to build a modal filter out of dspmodules, 
/// details on how the particular modal filter UI is rendered, and information about controllable parameters. 
pub trait NRTModule {

}