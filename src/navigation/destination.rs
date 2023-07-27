use crate::interaction::entity::message::InteractionMessage;
use crate::interaction::entity::state::InteractionState;
use crate::template::entity::{TemplateMessage, TemplateState};

#[derive(Clone, Debug)]
pub enum Destination {
    Interaction(InteractionState, InteractionMessage),
    Template(TemplateState, TemplateMessage),
}