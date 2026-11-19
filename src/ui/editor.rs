use iced::highlighter;
use iced::widget::{container, rich_text, scrollable, span, text, text_editor, Column};
use iced::{Element, Font, Length, Padding};

use crate::app::{Message, NotepadApp};
use crate::theme::{self, FluentPalette};

/// Main editor surface: supports live `text_editor` with automatic syntax highlighting
/// for Code, Config, Data, Markdown, and Script/SVG files, plus Windows 11's "Formatted" view.
pub fn view<'a>(app: &'a NotepadApp, p: FluentPalette) -> Element<'a, Message> {
    let tab = app.active_tab_ref();
    let scaled_size = app.effective_font_size();
    let font = app.settings.font_family.to_iced_font();

    if tab.formatted_view {
        return formatted_preview(tab.content.text(), scaled_size, font, p);
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

/// Renders lightweight Markdown formatting matching Windows 11 Notepad's "Formatted" mode.
fn formatted_preview<'a>(
    raw: String,
    base_size: f32,
    base_font: Font,
    p: FluentPalette,
) -> Element<'a, Message> {
    let mut col = Column::new().spacing(8).width(Length::Fill);

    for line in raw.lines() {
        let trimmed = line.trim_end();
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
        } else if let Some(item) = trimmed.strip_prefix("- ").or_else(|| trimmed.strip_prefix("* "))
        {
            col = col.push(render_inline_spans(
                &format!("  •  {item}"),
                base_size,
                base_font,
                p,
            ));
        } else if let Some(pos) = trimmed.find(". ") {
            if trimmed[..pos].chars().all(|c| c.is_ascii_digit()) {
                col = col.push(render_inline_spans(
                    &format!("  {trimmed}"),
                    base_size,
                    base_font,
                    p,
                ));
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

/// Lightweight inline parser for `**bold**`, `*italic*`, and `[label](url)` spans.
fn render_inline_spans<'a>(
    line: &str,
    base_size: f32,
    base_font: Font,
    p: FluentPalette,
) -> Element<'a, Message> {
    let mut spans = Vec::new();
    let chars: Vec<char> = line.chars().collect();
    let len = chars.len();
    let mut i = 0;
    let mut buf = String::new();

    let flush_plain = |buf: &mut String, spans: &mut Vec<iced::widget::text::Span<'a, (), Font>>| {
        if !buf.is_empty() {
            spans.push(
                span(std::mem::take(buf))
                    .size(base_size)
                    .font(base_font)
                    .color(p.text_primary),
            );
        }
    };

    while i < len {
        // Bold: **text**
        if i + 1 < len && chars[i] == '*' && chars[i + 1] == '*' {
            if let Some(end) = find_double_star(&chars, i + 2) {
                flush_plain(&mut buf, &mut spans);
                let bold_text: String = chars[i + 2..end].iter().collect();
                spans.push(
                    span(bold_text)
                        .size(base_size)
                        .font(Font {
                            weight: iced::font::Weight::Bold,
                            ..base_font
                        })
                        .color(p.text_primary),
                );
                i = end + 2;
                continue;
            }
        }

        // Italic: *text*
        if chars[i] == '*' {
            if let Some(end) = find_single_char(&chars, i + 1, '*') {
                flush_plain(&mut buf, &mut spans);
                let italic_text: String = chars[i + 1..end].iter().collect();
                spans.push(
                    span(italic_text)
                        .size(base_size)
                        .font(Font {
                            style: iced::font::Style::Italic,
                            ..base_font
                        })
                        .color(p.text_primary),
                );
                i = end + 1;
                continue;
            }
        }

        // Strikethrough: ~~text~~
        if i + 1 < len && chars[i] == '~' && chars[i + 1] == '~' {
            if let Some(end) = find_double_tilde(&chars, i + 2) {
                flush_plain(&mut buf, &mut spans);
                let strike_text: String = chars[i + 2..end].iter().collect();
                spans.push(
                    span(strike_text)
                        .size(base_size)
                        .font(base_font)
                        .color(p.text_secondary)
                        .strikethrough(true),
                );
                i = end + 2;
                continue;
            }
        }

        // Inline Code: `code`
        if chars[i] == '`' {
            if let Some(end) = find_single_char(&chars, i + 1, '`') {
                flush_plain(&mut buf, &mut spans);
                let code_text: String = chars[i + 1..end].iter().collect();
                spans.push(
                    span(code_text)
                        .size(base_size)
                        .font(Font::MONOSPACE)
                        .color(p.text_secondary),
                );
                i = end + 1;
                continue;
            }
        }

        // Link: [label](url)
        if chars[i] == '[' {
            if let Some((label, after_idx)) = parse_link(&chars, i) {
                flush_plain(&mut buf, &mut spans);
                spans.push(
                    span(label)
                        .size(base_size)
                        .font(base_font)
                        .color(p.link)
                        .underline(true),
                );
                i = after_idx;
                continue;
            }
        }

        buf.push(chars[i]);
        i += 1;
    }

    flush_plain(&mut buf, &mut spans);

    rich_text(spans).size(base_size).into()
}

fn find_double_star(chars: &[char], start: usize) -> Option<usize> {
    let mut j = start;
    while j + 1 < chars.len() {
        if chars[j] == '*' && chars[j + 1] == '*' {
            return Some(j);
        }
        j += 1;
    }
    None
}

fn find_double_tilde(chars: &[char], start: usize) -> Option<usize> {
    let mut j = start;
    while j + 1 < chars.len() {
        if chars[j] == '~' && chars[j + 1] == '~' {
            return Some(j);
        }
        j += 1;
    }
    None
}

fn find_single_char(chars: &[char], start: usize, target: char) -> Option<usize> {
    (start..chars.len()).find(|&j| chars[j] == target)
}

fn parse_link(chars: &[char], start: usize) -> Option<(String, usize)> {
    let close_bracket = find_single_char(chars, start + 1, ']')?;
    if close_bracket + 1 >= chars.len() || chars[close_bracket + 1] != '(' {
        return None;
    }
    let close_paren = find_single_char(chars, close_bracket + 2, ')')?;
    let label: String = chars[start + 1..close_bracket].iter().collect();
    Some((label, close_paren + 1))
}
