use iced::widget::{button, container, mouse_area, row, space, text, Row};
use iced::{Alignment, Element, Length, Padding};

use crate::app::{Message, NotepadApp};
use crate::icons;
use crate::theme::{self, FluentPalette};

/// Windows 11 Fluent Tabbed Title Bar (Mica surface + Logo + Tabs + '+' + Window Controls).
pub fn view<'a>(app: &'a NotepadApp, p: FluentPalette) -> Element<'a, Message> {
    let logo = container(icons::notepad_logo(20.0))
        .padding(Padding {
            top: 0.0,
            right: 8.0,
            bottom: 0.0,
            left: 12.0,
        })
        .center_y(Length::Fill);

    let mut tab_strip = Row::new()
        .spacing(2)
        .align_y(Alignment::End)
        .height(Length::Fill);

    for (idx, tab) in app.tabs.iter().enumerate() {
        let is_active = idx == app.active_tab;
        let display_title = if tab.title.chars().count() > 26 {
            let truncated: String = tab.title.chars().take(25).collect();
            format!("{truncated}…")
        } else {
            tab.title.clone()
        };

        let title_text = text(display_title).size(12.5).color(if is_active {
            p.text_primary
        } else {
            p.text_secondary
        });

        let dirty_or_close: Element<'a, Message> = if tab.is_dirty {
            button(
                container(text("•").size(16).color(p.text_secondary))
                    .center_x(20)
                    .center_y(20),
            )
            .padding(0)
            .style(theme::ghost_button(p))
            .on_press(Message::CloseTab(idx))
            .into()
        } else {
            button(
                container(icons::close(11.0, p.text_secondary))
                    .center_x(20)
                    .center_y(20),
            )
            .padding(0)
            .style(theme::ghost_button(p))
            .on_press(Message::CloseTab(idx))
            .into()
        };

        let tab_inner = row![
            button(
                container(title_text)
                    .width(Length::Fill)
                    .clip(true)
                    .center_y(Length::Fill)
            )
            .padding(Padding::from([0, 4]))
            .width(Length::Fill)
            .height(Length::Fill)
            .style(theme::ghost_button(p))
            .on_press(Message::SelectTab(idx)),
            dirty_or_close,
        ]
        .align_y(Alignment::Center)
        .spacing(4)
        .padding(Padding {
            top: 0.0,
            right: 6.0,
            bottom: 0.0,
            left: 10.0,
        });

        let tab_widget: Element<'a, Message> = if is_active {
            container(tab_inner)
                .width(Length::Fixed(215.0))
                .height(Length::Fixed(32.0))
                .style(theme::active_tab_container(p))
                .into()
        } else {
            container(tab_inner)
                .width(Length::Fixed(195.0))
                .height(Length::Fixed(30.0))
                .into()
        };

        tab_strip = tab_strip.push(tab_widget);
    }

    let new_tab_btn = container(
        button(
            container(icons::plus(13.0, p.text_primary))
                .center_x(28)
                .center_y(26),
        )
        .padding(0)
        .style(theme::ghost_button(p))
        .on_press(Message::NewTab),
    )
    .padding(Padding {
        top: 0.0,
        right: 4.0,
        bottom: 3.0,
        left: 4.0,
    })
    .align_y(Alignment::End)
    .height(Length::Fill);

    // Draggable empty space between tabs and window controls (works on Linux Wayland/X11, Windows, macOS).
    let drag_region = mouse_area(
        container(space::horizontal())
            .width(Length::Fill)
            .height(Length::Fill),
    )
    .on_press(Message::DragWindow)
    .on_double_click(Message::MaximizeWindow);

    let mut bar = row![logo, tab_strip, new_tab_btn, drag_region]
        .height(Length::Fixed(38.0))
        .width(Length::Fill)
        .align_y(Alignment::End);

    // Custom caption controls on native desktop builds.
    #[cfg(not(target_arch = "wasm32"))]
    {
        let min_btn = button(
            container(icons::minimize(11.0, p.text_primary))
                .center_x(46)
                .center_y(38),
        )
        .padding(0)
        .style(theme::window_control_button(p, false))
        .on_press(Message::MinimizeWindow);

        let max_btn = button(
            container(icons::maximize(11.0, p.text_primary))
                .center_x(46)
                .center_y(38),
        )
        .padding(0)
        .style(theme::window_control_button(p, false))
        .on_press(Message::MaximizeWindow);

        let close_btn = button(
            container(icons::close(11.0, p.text_primary))
                .center_x(46)
                .center_y(38),
        )
        .padding(0)
        .style(theme::window_control_button(p, true))
        .on_press(Message::CloseWindow);

        bar = bar.push(row![min_btn, max_btn, close_btn].height(Length::Fill));
    }

    container(bar)
        .width(Length::Fill)
        .height(Length::Fixed(38.0))
        .style(theme::titlebar_container(p))
        .into()
}
