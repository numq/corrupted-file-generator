use crate::elm::Reducer;
use crate::template::entity::{TemplateMessage, TemplateState};

#[derive(Clone, Debug)]
pub struct TemplateReducer;

impl TemplateReducer {
    pub fn new() -> Box<dyn Reducer<TemplateState, TemplateMessage>> { Box::from(TemplateReducer {}) }
}

impl Reducer<TemplateState, TemplateMessage> for TemplateReducer {
    fn reduce(&self, state: &TemplateState, message: &TemplateMessage) -> TemplateState {
        todo!("add template reducer");
    }
}