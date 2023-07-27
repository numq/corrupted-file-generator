use iced::{Alignment, Element, Length};
use iced::alignment::{Horizontal, Vertical};
use iced::widget::{button, Column, column, container, row, text, text_input};

use crate::elm::View;
use crate::interaction::entity::message::InteractionMessage;
use crate::interaction::entity::size_type::SizeType;
use crate::interaction::entity::state::InteractionState;

#[derive(Clone, Debug)]
pub struct InteractionView;

impl InteractionView {
    pub fn new() -> Box<dyn View<InteractionState, InteractionMessage>> { Box::from(InteractionView {}) }
}

impl View<InteractionState, InteractionMessage> for InteractionView {
    fn render(&self, state: &InteractionState) -> Element<InteractionMessage> {
        let input_name = text_input(
            "Name",
            &state.file_name,
        )
            .on_input(InteractionMessage::UpdateName)
            .padding(8)
            .size(32)
            .into();

        let input_extension = text_input(
            "Extension",
            &state.file_extension,
        )
            .on_input(InteractionMessage::UpdateExtension)
            .padding(8)
            .size(32)
            .into();

        let input_size_text = text_input(
            "Size",
            &Some(state.size_in_bytes)
                .map(|num| if num > 0 { num.to_string() } else { String::new() })
                .unwrap_or_default(),
        )
            .on_input(|value| {
                let input = if value.is_empty() { String::from("0") } else {
                    value.chars().filter(|c| {
                        if let Some(digit) = c.to_digit(10) {
                            return (0..9).contains(&digit);
                        }
                        false
                    }).collect()
                };
                InteractionMessage::UpdateSize(input)
            })
            .padding(8)
            .size(32)
            .into();

        let input_size_type = row(SizeType::values().iter().map(|size_type| {
            let label = text(size_type.name())
                .width(Length::Fill)
                .height(Length::Fill)
                .vertical_alignment(Vertical::Center)
                .horizontal_alignment(Horizontal::Center);
            let mut button = button(label)
                .padding(8)
                .width(Length::Fill);
            if state.selected_size_type.name() != size_type.name() {
                button = button.on_press(InteractionMessage::ChangeSizeType(size_type.clone()));
            }
            button.into()
        }).collect()).spacing(8).align_items(Alignment::Center).height(48).into();

        let button_save = {
            let mut button = button(
                text("Save file")
                    .horizontal_alignment(Horizontal::Center)
                    .vertical_alignment(Vertical::Center)
            ).width(96).height(48).padding(8);
            let name = state.file_name
                .trim()
                .chars()
                .filter(|c| c.is_ascii())
                .collect::<String>();
            let extension = state.file_extension
                .trim()
                .chars()
                .filter(|c| c.is_ascii())
                .collect::<String>();
            let size_in_bytes = state.selected_size_type.value(&state.size_in_bytes);
            if !name.is_empty() && !extension.is_empty() {
                let state = state.clone();
                button = button.on_press(InteractionMessage::SaveFile {
                    name: state.file_name,
                    extension: state.file_extension,
                    size_in_bytes,
                })
            }
            button.into()
        };

        let content: Column<InteractionMessage> = column(vec![
            input_name,
            input_extension,
            input_size_text,
            input_size_type,
            button_save,
        ])
            .align_items(Alignment::Center)
            .spacing(8)
            .padding(8)
            .into();

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}