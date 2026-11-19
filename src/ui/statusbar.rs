use iced::widget::{button, container, row, rule, space, text};
use iced::{Alignment, Element, Length, Padding};

use crate::app::{Message, NotepadApp};
use crate::theme::{self, FluentPalette};

/// Windows 11 Notepad bottom status bar:
/// `Ln 1, Col 1 | 1,440 characters | Formatted | Rust | 100% | Windows (CRLF) | UTF-8`
pub fn view<'a>(app: &'a NotepadApp, p: FluentPalette) -> Element<'a, Message> {
    let tab = app.active_tab_ref();

    let cursor_lbl = text(format!("Ln {}, Col {}", tab.cursor_line, tab.cursor_col))
        .size(11.5)
        .color(p.text_secondary);

    let chars_lbl = text(format!(
        "{} {}",
        tab.char_count_formatted,
        if tab.char_count == 1 {
            "character"
        } else {
            "characters"
        }
    ))
    .size(11.5)
    .color(p.text_secondary);

    let mode_btn = button(
        text(if tab.formatted_view {
            "A  Formatted"
        } else {
            "A  Plain text"
        })
        .size(11.5)
        .color(p.text_secondary),
    )
    .padding(Padding::from([2, 8]))
    .style(theme::statusbar_button(p))
    .on_press(Message::ToggleFormattedView);

    let file_type_lbl = text(tab.file_type.label).size(11.5).color(p.text_secondary);

    let zoom_btn = button(
        text(format!("{}%", app.zoom_percent))
            .size(11.5)
            .color(p.text_secondary),
    )
    .padding(Padding::from([2, 8]))
    .style(theme::statusbar_button(p))
    .on_press(Message::ZoomReset);

    let eol_btn = button(
        text(tab.line_ending.label())
            .size(11.5)
            .color(p.text_secondary),
    )
    .padding(Padding::from([2, 8]))
    .style(theme::statusbar_button(p))
    .on_press(Message::ToggleLineEnding);

    let encoding_lbl = text("UTF-8").size(11.5).color(p.text_secondary);

    let divider = || container(rule::vertical(1).style(theme::hairline_rule(p))).height(14);

    let status_row = row![
        container(cursor_lbl).padding(Padding::from([0, 8])),
        divider(),
        container(chars_lbl).padding(Padding::from([0, 8])),
        divider(),
        mode_btn,
        space::horizontal(),
        divider(),
        container(file_type_lbl).padding(Padding::from([0, 8])),
        divider(),
        zoom_btn,
        divider(),
        eol_btn,
        divider(),
        container(encoding_lbl).padding(Padding::from([0, 12])),
    ]
    .align_y(Alignment::Center)
    .spacing(2)
    .padding(Padding::from([0, 6]))
    .height(Length::Fixed(28.0))
    .width(Length::Fill);

    container(status_row)
        .width(Length::Fill)
        .height(Length::Fixed(28.0))
        .style(theme::statusbar_container(p))
        .into()
}
