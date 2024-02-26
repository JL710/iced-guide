use iced::widget;
use iced::widget::Component;
// ANCHOR: hyperlink_struct
pub struct Hyperlink {
    link: String,
}

impl Hyperlink {
    pub fn new(link: String) -> Self {
        Self { link }
    }
}
// ANCHOR_END: hyperlink_struct

// ANCHOR: hyperlink_event
#[derive(Debug, Copy, Clone)]
pub enum HyperlinkEvent {
    Clicked,
    MouseEnter,
    MouseExit,
}
// ANCHOR_END: hyperlink_event

// ANCHOR: hyperlink_state
#[derive(Default)]
pub struct HyperlinkState {
    hovered: bool,
}
// ANCHOR_END: hyperlink_state

// ANCHOR: implementing_component
impl<Message> Component<Message> for Hyperlink {
    // ANCHOR: component_types
    type State = HyperlinkState;
    type Event = HyperlinkEvent;
    // ANCHOR_END: component_types

    // ANCHOR: component_update
    fn update(&mut self, state: &mut Self::State, event: Self::Event) -> Option<Message> {
        match event {
            HyperlinkEvent::Clicked => println!("open link"),
            HyperlinkEvent::MouseEnter => state.hovered = true,
            HyperlinkEvent::MouseExit => state.hovered = false,
        }
        None
    }
    // ANCHOR_END: component_update

    // ANCHOR: component_view
    fn view(
        &self,
        state: &Self::State,
    ) -> iced::Element<'_, Self::Event, iced::Theme, iced::Renderer> {
        widget::container(
            widget::mouse_area(widget::text(&self.link).style(iced::theme::Text::Color(
                if state.hovered {
                    iced::Color::from_rgb(0.5, 0.5, 0.5)
                } else {
                    iced::Color::from_rgb(0.0, 0.0, 0.0)
                },
            )))
            .on_enter(HyperlinkEvent::MouseEnter)
            .on_exit(HyperlinkEvent::MouseExit)
            .on_press(HyperlinkEvent::Clicked),
        )
        .into()
    }
    // ANCHOR_END: component_view
}
// ANCHOR_END: implementing_component
