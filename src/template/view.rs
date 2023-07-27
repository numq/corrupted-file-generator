use iced::Element;

use crate::elm::View;
use crate::template::entity::{TemplateMessage, TemplateState};

#[derive(Clone, Debug)]
pub struct TemplateView;

impl TemplateView {
    pub fn new() -> Box<dyn View<TemplateState, TemplateMessage>> { Box::from(TemplateView {}) }
}

impl View<TemplateState, TemplateMessage> for TemplateView {
    fn render(&self, state: &TemplateState) -> Element<TemplateMessage> {
        todo!("add template view");
    }
}