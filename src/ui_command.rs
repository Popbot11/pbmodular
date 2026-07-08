use egui::Ui;

pub enum UICommand {
    Label(String),
}
 
pub fn render_ui_command(ui: &mut Ui, commands: Vec<UICommand>) {
    for cmd in commands {
        match cmd {
            UICommand::Label(text) => {ui.label(text); }
        }
    }
}