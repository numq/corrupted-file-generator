use iced::Element;

pub trait Reducer<State, Message> {
    fn reduce(&self, state: &State, message: &Message) -> State;
}

pub trait View<State, Message> {
    fn render(&self, state: &State) -> Element<Message>;
}