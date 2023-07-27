use crate::elm::Reducer;
use crate::generation::generator::FileGenerator;
use crate::interaction::entity::message::InteractionMessage;
use crate::interaction::entity::state::InteractionState;

#[derive(Clone, Debug)]
pub struct InteractionReducer;

impl InteractionReducer {
    pub fn new() -> Box<dyn Reducer<InteractionState, InteractionMessage>> { Box::from(InteractionReducer {}) }
}

impl Reducer<InteractionState, InteractionMessage> for InteractionReducer {
    fn reduce(&self, state: &InteractionState, message: &InteractionMessage) -> InteractionState {
        let state = &mut state.clone();
        match message {
            InteractionMessage::Empty => (),
            InteractionMessage::UpdateName(name) => {
                state.file_name = name.trim().to_string();
            }
            InteractionMessage::UpdateExtension(extension) => {
                state.file_extension = extension.trim().to_string();
            }
            InteractionMessage::UpdateSize(size) => {
                if let Ok(num) = size.parse() {
                    state.size_in_bytes = num
                }
            }
            InteractionMessage::ChangeSizeType(size_type) => {
                state.selected_size_type = size_type.clone();
            }

            InteractionMessage::SaveFile { name, extension, size_in_bytes } => {
                let file_dialog = rfd::FileDialog::new()
                    .set_title("Save file")
                    .add_filter("Custom extension", &[&extension])
                    .set_file_name(&format!("{}.{}", name, extension));
                if let Some(buf) = file_dialog.save_file() {
                    if let Some(path) = buf.to_str() {
                        let _ = FileGenerator::create(&path, *size_in_bytes);
                    }
                }
            }
            InteractionMessage::OpenTemplate => ()
        }
        state.clone()
    }
}