use iced::highlighter;
use iced::widget::{Column, container, rich_text, row, scrollable, span, text, text_editor};
use iced::{Alignment, Element, Font, Length, Padding};

use crate::app::{Message, NotepadApp};
use crate::theme::{self, FluentPalette};

/// Main editor surface: supports live `text_editor` with automatic syntax highlighting
/// for Code, Config, Data, Markdown, and Script/SVG files, plus Windows 11's "Formatted" view.
pub fn view<'a>(app: &'a NotepadApp, p: FluentPalette) -> Element<'a, Message> {
    let tab = app.active_tab_ref();
    let scaled_size = app.effective_font_size();
    let font = app.settings.font_family.to_iced_font();

    if tab.formatted_view {
        return formatted_preview(&tab.content, scaled_size, font, p);
    }

    let wrapping = if app.settings.word_wrap {
        text::Wrapping::Word
    } else {
        text::Wrapping::None
    };

    let base_editor = text_editor(&tab.content)
        .placeholder("")
        .on_action(Message::EditorAction)
        .font(font)
        .size(scaled_size)
        .line_height(1.45)
        .padding(Padding::from([16, 22]))
        .wrapping(wrapping)
        .height(Length::Fill)
        .style(theme::notepad_editor_style(p));

    let editor_element: Element<'a, Message> = if app.settings.syntax_highlighting {
        if let Some(syntax) = tab.file_type.syntax_token {
            let hl_theme = if p.is_dark {
                highlighter::Theme::Base16Ocean
            } else {
                highlighter::Theme::InspiredGitHub
            };
            base_editor.highlight(syntax, hl_theme).into()
        } else {
            base_editor.into()
        }
    } else {
        base_editor.into()
    };

    container(editor_element)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(theme::editor_container(p))
        .into()
}

/// Renders lightweight Markdown formatting matching Windows 11 Notepad's "Formatted" mode with zero document clones.
fn formatted_preview<'a>(
    content: &'a text_editor::Content,
    base_size: f32,
    base_font: Font,
    p: FluentPalette,
) -> Element<'a, Message> {
    let mut col = Column::new().spacing(8).width(Length::Fill);

    for line in content.lines() {
        let trimmed = line.text.trim_end();
        if trimmed.is_empty() {
            col = col.push(container(text("")).height(Length::Fixed(base_size * 0.6)));
            continue;
        }

        if let Some(h1) = trimmed.strip_prefix("# ") {
            col = col.push(
                text(h1.to_string())
                    .size(base_size * 2.05)
                    .font(Font {
                        weight: iced::font::Weight::Bold,
                        ..base_font
                    })
                    .color(p.text_primary),
            );
        } else if let Some(h2) = trimmed.strip_prefix("## ") {
            col = col.push(
                text(h2.to_string())
                    .size(base_size * 1.65)
                    .font(Font {
                        weight: iced::font::Weight::Bold,
                        ..base_font
                    })
                    .color(p.text_primary),
            );
        } else if let Some(h3) = trimmed.strip_prefix("### ") {
            col = col.push(
                text(h3.to_string())
                    .size(base_size * 1.35)
                    .font(Font {
                        weight: iced::font::Weight::Semibold,
                        ..base_font
                    })
                    .color(p.text_primary),
            );
        } else if let Some(h4) = trimmed.strip_prefix("#### ") {
            col = col.push(
                text(h4.to_string())
                    .size(base_size * 1.20)
                    .font(Font {
                        weight: iced::font::Weight::Semibold,
                        ..base_font
                    })
                    .color(p.text_primary),
            );
        } else if let Some(h5) = trimmed.strip_prefix("##### ") {
            col = col.push(
                text(h5.to_string())
                    .size(base_size * 1.10)
                    .font(Font {
                        weight: iced::font::Weight::Medium,
                        ..base_font
                    })
                    .color(p.text_primary),
            );
        } else if let Some(h6) = trimmed.strip_prefix("###### ") {
            col = col.push(
                text(h6.to_string())
                    .size(base_size * 1.00)
                    .font(Font {
                        weight: iced::font::Weight::Medium,
                        ..base_font
                    })
                    .color(p.text_secondary),
            );
        } else if let Some(item) = trimmed
            .strip_prefix("- ")
            .or_else(|| trimmed.strip_prefix("* "))
        {
            let bullet = text("  •  ").size(base_size).color(p.text_secondary);
            let item_view = render_inline_spans(item, base_size, base_font, p);
            col = col.push(
                row![bullet, item_view]
                    .spacing(0)
                    .align_y(Alignment::Center),
            );
        } else if let Some(pos) = trimmed.find(". ") {
            if trimmed[..pos].bytes().all(|b| b.is_ascii_digit()) {
                let num_str = &trimmed[..pos + 2];
                let rest_str = &trimmed[pos + 2..];
                let num_view = text(format!("  {num_str}"))
                    .size(base_size)
                    .color(p.text_secondary);
                let rest_view = render_inline_spans(rest_str, base_size, base_font, p);
                col = col.push(
                    row![num_view, rest_view]
                        .spacing(0)
                        .align_y(Alignment::Center),
                );
            } else {
                col = col.push(render_inline_spans(trimmed, base_size, base_font, p));
            }
        } else {
            col = col.push(render_inline_spans(trimmed, base_size, base_font, p));
        }
    }

    let content = container(col)
        .padding(Padding::from([18, 26]))
        .width(Length::Fill);

    container(scrollable(content).width(Length::Fill).height(Length::Fill))
        .width(Length::Fill)
        .height(Length::Fill)
        .style(theme::editor_container(p))
        .into()
}

/// Zero-copy scanner for `**bold**`, `*italic*`, and `[label](url)` spans.
fn render_inline_spans<'a>(
    line: &str,
    base_size: f32,
    base_font: Font,
    p: FluentPalette,
) -> Element<'a, Message> {
    // Fast path: if the line contains no markdown control characters, emit a single text widget with zero span allocations
    if !line.contains('*') && !line.contains('~') && !line.contains('`') && !line.contains('[') {
        return text(line.to_string())
            .size(base_size)
            .font(base_font)
            .color(p.text_primary)
            .into();
    }

    let mut spans: Vec<iced::widget::text::Span<'a, (), Font>> = Vec::new();
    let mut rest = line;

    while !rest.is_empty() {
        // Bold: **text**
        if let Some(after_open) = rest.strip_prefix("**") {
            if let Some(close_idx) = after_open.find("**") {
                let bold_text = &after_open[..close_idx];
                spans.push(
                    span(bold_text.to_string())
                        .size(base_size)
                        .font(Font {
                            weight: iced::font::Weight::Bold,
                            ..base_font
                        })
                        .color(p.text_primary),
                );
                rest = &after_open[close_idx + 2..];
                continue;
            }
        }

        // Strikethrough: ~~text~~
        if let Some(after_open) = rest.strip_prefix("~~") {
            if let Some(close_idx) = after_open.find("~~") {
                let strike_text = &after_open[..close_idx];
                spans.push(
                    span(strike_text.to_string())
                        .size(base_size)
                        .font(base_font)
                        .color(p.text_secondary)
                        .strikethrough(true),
                );
                rest = &after_open[close_idx + 2..];
                continue;
            }
        }

        // Italic: *text*
        if let Some(after_open) = rest.strip_prefix('*') {
            if let Some(close_idx) = after_open.find('*') {
                let italic_text = &after_open[..close_idx];
                spans.push(
                    span(italic_text.to_string())
                        .size(base_size)
                        .font(Font {
                            style: iced::font::Style::Italic,
                            ..base_font
                        })
                        .color(p.text_primary),
                );
                rest = &after_open[close_idx + 1..];
                continue;
            }
        }

        // Inline Code: `code`
        if let Some(after_open) = rest.strip_prefix('`') {
            if let Some(close_idx) = after_open.find('`') {
                let code_text = &after_open[..close_idx];
                spans.push(
                    span(code_text.to_string())
                        .size(base_size)
                        .font(Font::MONOSPACE)
                        .color(p.text_secondary),
                );
                rest = &after_open[close_idx + 1..];
                continue;
            }
        }

        // Link: [label](url)
        if let Some(after_open) = rest.strip_prefix('[') {
            if let Some(close_bracket) = after_open.find(']') {
                let after_bracket = &after_open[close_bracket + 1..];
                if after_bracket.starts_with('(') {
                    if let Some(close_paren) = after_bracket.find(')') {
                        let label = &after_open[..close_bracket];
                        spans.push(
                            span(label.to_string())
                                .size(base_size)
                                .font(base_font)
                                .color(p.link)
                                .underline(true),
                        );
                        rest = &after_bracket[close_paren + 1..];
                        continue;
                    }
                }
            }
        }

        // Consume plain text until next potential markdown character
        let next_marker = rest[1..]
            .find(['*', '~', '`', '['])
            .map(|idx| idx + 1)
            .unwrap_or(rest.len());

        let plain_segment = &rest[..next_marker];
        spans.push(
            span(plain_segment.to_string())
                .size(base_size)
                .font(base_font)
                .color(p.text_primary),
        );
        rest = &rest[next_marker..];
    }

    rich_text(spans).size(base_size).into()
}
