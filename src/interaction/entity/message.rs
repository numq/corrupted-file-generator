use crate::interaction::entity::size_type::SizeType;

#[derive(Clone, Debug)]
pub enum InteractionMessage {
    Empty,
    UpdateName(String),
    UpdateExtension(String),
    UpdateSize(String),
    ChangeSizeType(SizeType),
    SaveFile { name: String, extension: String, size_in_bytes: u64 },
    OpenTemplate,
}