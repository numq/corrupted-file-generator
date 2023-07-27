use iced::{Application, Command, Element, executor, Theme};

use crate::elm::{Reducer, View};
use crate::interaction::entity::message::InteractionMessage;
use crate::interaction::entity::state::InteractionState;
use crate::interaction::reducer::InteractionReducer;
use crate::interaction::view::InteractionView;
use crate::navigation::destination::Destination;
use crate::template::entity::{TemplateMessage, TemplateState};
use crate::template::reducer::TemplateReducer;
use crate::template::view::TemplateView;

pub struct Root {
    interaction_reducer: Box<dyn Reducer<InteractionState, InteractionMessage>>,
    interaction_view: Box<dyn View<InteractionState, InteractionMessage>>,

    template_reducer: Box<dyn Reducer<TemplateState, TemplateMessage>>,
    template_view: Box<dyn View<TemplateState, TemplateMessage>>,

    current_destination: Destination,
}

impl Root {
    pub fn new() -> Self {
        Root {
            interaction_reducer: InteractionReducer::new(),
            interaction_view: InteractionView::new(),

            template_reducer: TemplateReducer::new(),
            template_view: TemplateView::new(),

            current_destination: Destination::Interaction(InteractionState::new(), InteractionMessage::Empty),
        }
    }

    pub fn navigate(&mut self, destination: Destination) -> Command<Destination> {
        self.current_destination = destination.clone();
        Command::none()
    }
}

impl Application for Root {
    type Executor = executor::Default;
    type Message = Destination;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Destination>) {
        (Root::new(), Command::none())
    }

    fn title(&self) -> String {
        match &self.current_destination {
            Destination::Interaction(..) => String::from("File generator"),
            Destination::Template(..) => String::from("Templates"),
        }
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Destination::Interaction(state, message) => {
                match message {
                    InteractionMessage::OpenTemplate => self.navigate(Destination::Template(TemplateState::new(), TemplateMessage::Empty)),
                    _ => {
                        self.current_destination = Destination::Interaction(self.interaction_reducer.reduce(&state, &message), InteractionMessage::Empty);
                        Command::none()
                    }
                }
            }
            Destination::Template(state, message) => {
                match message {
                    TemplateMessage::Close => self.navigate(Destination::Interaction(InteractionState::new(), InteractionMessage::Empty)),
                    _ => {
                        self.current_destination = Destination::Template(self.template_reducer.reduce(&state, &message), TemplateMessage::Empty);
                        Command::none()
                    }
                }
            }
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        match &self.current_destination {
            Destination::Interaction(state, ..) => {
                self.interaction_view.render(&state).map(|message| Destination::Interaction(state.clone(), message))
            }
            Destination::Template(state, ..) => {
                self.template_view.render(&state).map(|message| Destination::Template(state.clone(), message))
            }
        }
    }
}