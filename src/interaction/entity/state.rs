use crate::interaction::entity::size_type::SizeType;

#[derive(Clone, Debug)]
pub struct InteractionState {
    pub file_name: String,
    pub file_extension: String,
    pub size_in_bytes: u64,
    pub selected_size_type: SizeType,
    pub notification: Option<String>,
}

impl InteractionState {
    pub fn new() -> Self {
        InteractionState {
            file_name: String::new(),
            file_extension: String::new(),
            size_in_bytes: 0,
            selected_size_type: SizeType::KB,
            notification: None,
        }
    }
}