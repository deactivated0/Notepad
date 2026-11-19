use iced::widget::{button, container, row, rule, space, text};
use iced::{Alignment, Element, Font, Length, Padding};

use crate::app::{ActiveMenu, Message, NotepadApp};
use crate::document::FormatAction;
use crate::icons;
use crate::theme::{self, FluentPalette};

/// Menu Bar (`File`, `Edit`, `View`) + Lightweight Formatting Toolbar + Settings Gear icon.
pub fn view<'a>(app: &'a NotepadApp, p: FluentPalette) -> Element<'a, Message> {
    let file_btn = menu_trigger("File", ActiveMenu::File, app.open_menu, p);
    let edit_btn = menu_trigger("Edit", ActiveMenu::Edit, app.open_menu, p);
    let view_btn = menu_trigger("View", ActiveMenu::View, app.open_menu, p);

    let left_menus = row![file_btn, edit_btn, view_btn]
        .spacing(4)
        .align_y(Alignment::Center);

    let mut bar = row![left_menus]
        .align_y(Alignment::Center)
        .padding(Padding::from([0, 8]))
        .height(Length::Fixed(36.0))
        .width(Length::Fill);

    if app.settings.show_formatting_toolbar {
        let h1_btn = fmt_btn(text("H1").size(11.5), FormatAction::Heading1, p);
        let h2_btn = fmt_btn(text("H2").size(11.5), FormatAction::Heading2, p);
        let h3_btn = fmt_btn(text("H3").size(11.5), FormatAction::Heading3, p);
        let h4_btn = fmt_btn(text("H4").size(11.5), FormatAction::Heading4, p);
        let h5_btn = fmt_btn(text("H5").size(11.5), FormatAction::Heading5, p);
        let h6_btn = fmt_btn(text("H6").size(11.5), FormatAction::Heading6, p);

        let bold_btn = fmt_btn(
            text("B").size(13.0).font(Font {
                weight: iced::font::Weight::Bold,
                ..Font::DEFAULT
            }),
            FormatAction::Bold,
            p,
        );
        let italic_btn = fmt_btn(
            text("I").size(13.0).font(Font {
                style: iced::font::Style::Italic,
                ..Font::DEFAULT
            }),
            FormatAction::Italic,
            p,
        );
        let strike_btn = fmt_btn(
            text("S̶").size(12.5).font(Font {
                weight: iced::font::Weight::Bold,
                ..Font::DEFAULT
            }),
            FormatAction::Strikethrough,
            p,
        );

        let list_btn = fmt_btn(
            icons::list(14.0, p.text_primary),
            FormatAction::BulletList,
            p,
        );
        let num_btn = fmt_btn(
            text("1.").size(12.0).font(Font {
                weight: iced::font::Weight::Bold,
                ..Font::DEFAULT
            }),
            FormatAction::NumberedList,
            p,
        );

        let code_btn = fmt_btn(
            text("</>").size(11.0).font(Font::MONOSPACE),
            FormatAction::Code,
            p,
        );
        let link_btn = fmt_btn(icons::link(14.0, p.text_primary), FormatAction::Link, p);
        let clear_btn = fmt_btn(text("Aₓ").size(12.0), FormatAction::ClearFormatting, p);

        let formatting_group = row![
            h1_btn,
            h2_btn,
            h3_btn,
            h4_btn,
            h5_btn,
            h6_btn,
            container(rule::vertical(1).style(theme::hairline_rule(p))).height(16),
            bold_btn,
            italic_btn,
            strike_btn,
            container(rule::vertical(1).style(theme::hairline_rule(p))).height(16),
            list_btn,
            num_btn,
            container(rule::vertical(1).style(theme::hairline_rule(p))).height(16),
            code_btn,
            link_btn,
            container(rule::vertical(1).style(theme::hairline_rule(p))).height(16),
            clear_btn,
        ]
        .spacing(2)
        .align_y(Alignment::Center);

        bar = bar
            .push(space::horizontal())
            .push(formatting_group)
            .push(space::horizontal());
    } else {
        bar = bar.push(space::horizontal());
    }

    let gear_btn = button(
        container(icons::settings_gear(16.0, p.text_primary))
            .center_x(30)
            .center_y(28),
    )
    .padding(0)
    .style(theme::toggle_toolbar_button(p, app.show_settings))
    .on_press(Message::ToggleSettings);

    bar = bar.push(gear_btn);

    container(bar)
        .width(Length::Fill)
        .height(Length::Fixed(36.0))
        .style(theme::surface_container(p))
        .into()
}

fn menu_trigger<'a>(
    label: &'static str,
    menu: ActiveMenu,
    open_menu: Option<ActiveMenu>,
    p: FluentPalette,
) -> Element<'a, Message> {
    button(text(label).size(13.0))
        .padding(Padding::from([5, 10]))
        .style(theme::menu_bar_button(p, open_menu == Some(menu)))
        .on_press(Message::ToggleMenu(menu))
        .into()
}

fn fmt_btn<'a>(
    content: impl Into<Element<'a, Message>>,
    action: FormatAction,
    p: FluentPalette,
) -> Element<'a, Message> {
    button(container(content).center_x(28).center_y(26))
        .padding(0)
        .style(theme::ghost_button(p))
        .on_press(Message::ApplyFormat(action))
        .into()
}
