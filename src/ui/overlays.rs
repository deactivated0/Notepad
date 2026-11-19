use iced::widget::{
    button, column, container, mouse_area, pick_list, row, rule, scrollable, space, stack, text,
    text_input, toggler, Column,
};
use iced::{Alignment, Element, Length, Padding};

use crate::app::{ActiveMenu, Message, NotepadApp};
use crate::icons;
use crate::settings::EditorFontFamily;
use crate::theme::{self, AppTheme, FluentPalette};

/// Floating dropdown menus for File, Edit, and View (Windows 11 Fluent style).
pub fn menu_dropdown<'a>(
    app: &'a NotepadApp,
    menu: ActiveMenu,
    p: FluentPalette,
) -> Element<'a, Message> {
    let left_offset = match menu {
        ActiveMenu::File => 8.0,
        ActiveMenu::Edit => 54.0,
        ActiveMenu::View => 100.0,
    };

    let items: Column<'a, Message> = match menu {
        ActiveMenu::File => column![
            menu_item("New tab", "Ctrl+T", Message::NewTab, p),
            menu_item("Open", "Ctrl+O", Message::OpenFile, p),
            menu_item("Save", "Ctrl+S", Message::SaveFile, p),
            menu_item("Save as", "Ctrl+Shift+S", Message::SaveFileAs, p),
            menu_sep(p),
            menu_item("Close tab", "Ctrl+W", Message::CloseActiveTab, p),
            menu_item("Exit", "Alt+F4", Message::CloseWindow, p),
        ]
        .spacing(2),
        ActiveMenu::Edit => column![
            menu_item("Undo", "Ctrl+Z", Message::Undo, p),
            menu_item("Redo", "Ctrl+Y", Message::Redo, p),
            menu_sep(p),
            menu_item("Find", "Ctrl+F", Message::OpenFind, p),
            menu_item("Replace", "Ctrl+H", Message::OpenReplace, p),
            menu_sep(p),
            menu_item("Time/Date", "F5", Message::InsertTimeDate, p),
        ]
        .spacing(2),
        ActiveMenu::View => {
            let tab = app.active_tab_ref();
            column![
                menu_item("Zoom in", "Ctrl+Plus", Message::ZoomIn, p),
                menu_item("Zoom out", "Ctrl+Minus", Message::ZoomOut, p),
                menu_item("Restore default zoom", "Ctrl+0", Message::ZoomReset, p),
                menu_sep(p),
                menu_check_item(
                    "Status bar",
                    app.settings.show_status_bar,
                    Message::ToggleStatusBar,
                    p,
                ),
                menu_check_item(
                    "Word wrap",
                    app.settings.word_wrap,
                    Message::ToggleWordWrap,
                    p,
                ),
                menu_check_item(
                    "Syntax highlighting",
                    app.settings.syntax_highlighting,
                    Message::ToggleSyntaxHighlighting,
                    p,
                ),
                menu_check_item(
                    "Formatting toolbar",
                    app.settings.show_formatting_toolbar,
                    Message::ToggleFormattingToolbar,
                    p,
                ),
                menu_check_item(
                    "Formatted preview",
                    tab.formatted_view,
                    Message::ToggleFormattedView,
                    p,
                ),
            ]
            .spacing(2)
        }
    };

    let card = container(items)
        .width(Length::Fixed(236.0))
        .padding(Padding::from([5, 5]))
        .style(theme::overlay_card(p));

    // Backdrop click-outside dismiss + positioned menu card.
    let backdrop = mouse_area(
        container(space::horizontal())
            .width(Length::Fill)
            .height(Length::Fill),
    )
    .on_press(Message::DismissOverlays);

    let positioned = container(card).padding(Padding {
        top: 76.0,
        right: 0.0,
        bottom: 0.0,
        left: left_offset,
    });

    let layers = vec![backdrop.into(), positioned.into()];

    stack(layers)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

fn menu_item<'a>(
    label: &'static str,
    shortcut: &'static str,
    msg: Message,
    p: FluentPalette,
) -> Element<'a, Message> {
    button(
        row![
            text(label).size(13.0).color(p.text_primary),
            space::horizontal(),
            text(shortcut).size(11.5).color(p.text_secondary),
        ]
        .align_y(Alignment::Center)
        .width(Length::Fill),
    )
    .width(Length::Fill)
    .padding(Padding::from([6, 10]))
    .style(theme::ghost_button(p))
    .on_press(msg)
    .into()
}

fn menu_check_item<'a>(
    label: &'static str,
    checked: bool,
    msg: Message,
    p: FluentPalette,
) -> Element<'a, Message> {
    button(
        row![
            container(
                text(if checked { "✓" } else { "" })
                    .size(12.5)
                    .color(p.accent)
            )
            .width(Length::Fixed(18.0)),
            text(label).size(13.0).color(p.text_primary),
        ]
        .align_y(Alignment::Center)
        .width(Length::Fill),
    )
    .width(Length::Fill)
    .padding(Padding::from([6, 10]))
    .style(theme::ghost_button(p))
    .on_press(msg)
    .into()
}

fn menu_sep<'a>(p: FluentPalette) -> Element<'a, Message> {
    container(rule::horizontal(1).style(theme::hairline_rule(p)))
        .padding(Padding::from([4, 6]))
        .into()
}

/// Floating Windows 11 Fluent Find & Replace bar anchored top-right inside editor.
pub fn find_replace_bar<'a>(app: &'a NotepadApp, p: FluentPalette) -> Element<'a, Message> {
    let tab = app.active_tab_ref();
    let total_matches = tab.count_matches(&app.find_query, app.find_match_case);
    let match_status = if app.find_query.is_empty() {
        String::new()
    } else if total_matches == 0 {
        "No results".to_string()
    } else {
        format!(
            "{} of {}",
            (app.find_match_index % total_matches) + 1,
            total_matches
        )
    };

    let find_row = row![
        text_input("Find", &app.find_query)
            .on_input(Message::FindQueryChanged)
            .on_submit(Message::FindNext)
            .size(13)
            .padding(Padding::from([6, 10]))
            .width(Length::Fixed(210.0))
            .style(theme::fluent_input_style(p)),
        text(match_status).size(11.5).color(p.text_secondary),
        button(text("Aa").size(12.0))
            .padding(Padding::from([5, 7]))
            .style(theme::toggle_toolbar_button(p, app.find_match_case))
            .on_press(Message::ToggleFindMatchCase),
        button(text("↑").size(13.0))
            .padding(Padding::from([4, 8]))
            .style(theme::ghost_button(p))
            .on_press(Message::FindPrevious),
        button(text("↓").size(13.0))
            .padding(Padding::from([4, 8]))
            .style(theme::ghost_button(p))
            .on_press(Message::FindNext),
        button(
            container(icons::close(12.0, p.text_primary))
                .center_x(24)
                .center_y(24),
        )
        .padding(0)
        .style(theme::ghost_button(p))
        .on_press(Message::CloseFindReplace),
    ]
    .spacing(6)
    .align_y(Alignment::Center);

    let mut content = column![find_row].spacing(8);

    if app.show_replace {
        let replace_row = row![
            text_input("Replace with", &app.replace_query)
                .on_input(Message::ReplaceQueryChanged)
                .on_submit(Message::ReplaceOne)
                .size(13)
                .padding(Padding::from([6, 10]))
                .width(Length::Fixed(210.0))
                .style(theme::fluent_input_style(p)),
            button(text("Replace").size(12.0))
                .padding(Padding::from([6, 10]))
                .style(theme::accent_button(p))
                .on_press(Message::ReplaceOne),
            button(text("Replace all").size(12.0))
                .padding(Padding::from([6, 10]))
                .style(theme::ghost_button(p))
                .on_press(Message::ReplaceAll),
        ]
        .spacing(6)
        .align_y(Alignment::Center);

        content = content.push(replace_row);
    }

    let card = container(content)
        .padding(Padding::from([10, 12]))
        .style(theme::overlay_card(p));

    container(row![space::horizontal(), card])
        .width(Length::Fill)
        .padding(Padding {
            top: 82.0,
            right: 20.0,
            bottom: 0.0,
            left: 20.0,
        })
        .into()
}

/// Floating Windows 11 Fluent Settings Flyout (anchored to top-right gear icon).
pub fn settings_flyout<'a>(app: &'a NotepadApp, p: FluentPalette) -> Element<'a, Message> {
    let header = row![
        text("Settings").size(15).font(iced::Font {
            weight: iced::font::Weight::Semibold,
            ..iced::Font::DEFAULT
        }).color(p.text_primary),
        space::horizontal(),
        button(
            container(icons::close(12.0, p.text_secondary))
                .center_x(24)
                .center_y(24),
        )
        .padding(0)
        .style(theme::ghost_button(p))
        .on_press(Message::ToggleSettings),
    ]
    .align_y(Alignment::Center)
    .padding(Padding::from([8, 14]))
    .width(Length::Fill);

    let theme_row = flyout_setting_row(
        "App theme",
        pick_list(AppTheme::ALL, Some(app.settings.theme), Message::SetTheme)
            .text_size(12.5)
            .padding(Padding::from([4, 10])),
        p,
    );

    let font_row = flyout_setting_row(
        "Font family",
        pick_list(
            EditorFontFamily::ALL,
            Some(app.settings.font_family),
            Message::SetFontFamily,
        )
        .text_size(12.5)
        .padding(Padding::from([4, 10])),
        p,
    );

    let size_controls = row![
        button(text("−").size(13))
            .padding(Padding::from([3, 8]))
            .style(theme::ghost_button(p))
            .on_press(Message::DecreaseBaseFontSize),
        text(format!("{} pt", app.settings.base_font_size))
            .size(12.5)
            .color(p.text_primary),
        button(text("+").size(13))
            .padding(Padding::from([3, 8]))
            .style(theme::ghost_button(p))
            .on_press(Message::IncreaseBaseFontSize),
    ]
    .spacing(8)
    .align_y(Alignment::Center);

    let font_size_row = flyout_setting_row("Font size", size_controls, p);

    let wrap_row = flyout_setting_row(
        "Word wrap",
        toggler(app.settings.word_wrap).on_toggle(|_| Message::ToggleWordWrap),
        p,
    );

    let syntax_row = flyout_setting_row(
        "Syntax highlighting",
        toggler(app.settings.syntax_highlighting).on_toggle(|_| Message::ToggleSyntaxHighlighting),
        p,
    );

    let toolbar_row = flyout_setting_row(
        "Formatting toolbar",
        toggler(app.settings.show_formatting_toolbar)
            .on_toggle(|_| Message::ToggleFormattingToolbar),
        p,
    );

    let status_row = flyout_setting_row(
        "Status bar",
        toggler(app.settings.show_status_bar).on_toggle(|_| Message::ToggleStatusBar),
        p,
    );

    let about_sec = column![
        text("Notepad v1.0.0").size(12).font(iced::Font {
            weight: iced::font::Weight::Medium,
            ..iced::Font::DEFAULT
        }).color(p.text_secondary),
        text("Built with Rust & Iced 0.14").size(11).color(p.text_muted),
    ]
    .spacing(2)
    .padding(Padding::from([8, 14]));

    let section_header = |title: &'static str| {
        container(
            text(title)
                .size(11.5)
                .font(iced::Font {
                    weight: iced::font::Weight::Semibold,
                    ..iced::Font::DEFAULT
                })
                .color(p.text_secondary),
        )
        .padding(Padding {
            top: 8.0,
            right: 14.0,
            bottom: 2.0,
            left: 14.0,
        })
    };

    let content_list = column![
        section_header("Appearance"),
        theme_row,
        rule::horizontal(1).style(theme::hairline_rule(p)),
        section_header("Editor"),
        font_row,
        font_size_row,
        wrap_row,
        syntax_row,
        toolbar_row,
        status_row,
        rule::horizontal(1).style(theme::hairline_rule(p)),
        about_sec,
    ]
    .spacing(2)
    .width(Length::Fill);

    let scrollable_content = scrollable(content_list)
        .width(Length::Fill)
        .height(Length::Shrink);

    let card = container(column![
        header,
        rule::horizontal(1).style(theme::hairline_rule(p)),
        scrollable_content,
    ])
    .width(Length::Fixed(350.0))
    .style(theme::overlay_card(p));

    let backdrop = mouse_area(
        container(space::horizontal())
            .width(Length::Fill)
            .height(Length::Fill),
    )
    .on_press(Message::DismissOverlays);

    let positioned = container(row![space::horizontal(), card]).padding(Padding {
        top: 76.0,
        right: 12.0,
        bottom: 0.0,
        left: 0.0,
    });

    let layers = vec![backdrop.into(), positioned.into()];

    stack(layers)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

fn flyout_setting_row<'a>(
    title: &'static str,
    control: impl Into<Element<'a, Message>>,
    p: FluentPalette,
) -> Element<'a, Message> {
    container(
        row![
            text(title).size(13).color(p.text_primary),
            space::horizontal(),
            control.into(),
        ]
        .align_y(Alignment::Center)
        .width(Length::Fill),
    )
    .padding(Padding::from([6, 14]))
    .width(Length::Fill)
    .into()
}
