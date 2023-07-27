#[derive(Clone, Debug)]
pub struct TemplateState;

impl TemplateState {
    pub fn new() -> Self {
        TemplateState {}
    }
}

#[derive(Clone, Debug)]
pub enum TemplateMessage {
    Empty,
    Close,
}