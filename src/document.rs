use std::collections::VecDeque;
use std::path::PathBuf;

use iced::advanced::text::editor::Position;
use iced::widget::text_editor;

const MAX_UNDO_HISTORY: usize = 64;

/// Detected file format & syntax highlighting metadata for all supported extensions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FileType {
    pub label: &'static str,
    /// Syntax extension token passed to `iced::highlighter` (`None` = plain text).
    pub syntax_token: Option<&'static str>,
}

impl FileType {
    pub const PLAIN_TEXT: Self = Self {
        label: "Plain Text",
        syntax_token: None,
    };

    /// Detects file type from a filename or path across all 28 supported extensions.
    pub fn from_filename(name: &str) -> Self {
        let ext = name
            .rsplit('.')
            .next()
            .filter(|_| name.contains('.'))
            .unwrap_or("")
            .to_ascii_lowercase();

        match ext.as_str() {
            // Text
            "txt" | "text" => Self::PLAIN_TEXT,
            "log" => Self {
                label: "Log",
                syntax_token: None,
            },
            // Markdown
            "md" | "markdown" => Self {
                label: "Markdown",
                syntax_token: Some("md"),
            },
            // Configuration
            "json" => Self {
                label: "JSON",
                syntax_token: Some("json"),
            },
            "xml" => Self {
                label: "XML",
                syntax_token: Some("xml"),
            },
            "yaml" | "yml" => Self {
                label: "YAML",
                syntax_token: Some("yaml"),
            },
            "toml" => Self {
                label: "TOML",
                syntax_token: Some("toml"),
            },
            "ini" => Self {
                label: "INI",
                syntax_token: Some("ini"),
            },
            "conf" => Self {
                label: "Config",
                syntax_token: Some("ini"),
            },
            // Code
            "rs" => Self {
                label: "Rust",
                syntax_token: Some("rs"),
            },
            "py" => Self {
                label: "Python",
                syntax_token: Some("py"),
            },
            "js" => Self {
                label: "JavaScript",
                syntax_token: Some("js"),
            },
            "ts" => Self {
                label: "TypeScript",
                syntax_token: Some("ts"),
            },
            "html" => Self {
                label: "HTML",
                syntax_token: Some("html"),
            },
            "css" => Self {
                label: "CSS",
                syntax_token: Some("css"),
            },
            "c" => Self {
                label: "C",
                syntax_token: Some("c"),
            },
            "cpp" => Self {
                label: "C++",
                syntax_token: Some("cpp"),
            },
            "java" => Self {
                label: "Java",
                syntax_token: Some("java"),
            },
            "sh" => Self {
                label: "Shell",
                syntax_token: Some("sh"),
            },
            // Data
            "csv" => Self {
                label: "CSV",
                syntax_token: Some("csv"),
            },
            "tsv" => Self {
                label: "TSV",
                syntax_token: Some("tsv"),
            },
            "sql" => Self {
                label: "SQL",
                syntax_token: Some("sql"),
            },
            // Other
            "bat" => Self {
                label: "Batch",
                syntax_token: Some("bat"),
            },
            "ps1" => Self {
                label: "PowerShell",
                syntax_token: Some("ps1"),
            },
            "svg" => Self {
                label: "SVG",
                syntax_token: Some("xml"),
            },
            _ => Self::PLAIN_TEXT,
        }
    }
}

/// Line ending style shown in the Windows 11 Notepad status bar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LineEndingStyle {
    #[default]
    WindowsCrlf,
    UnixLf,
}

impl LineEndingStyle {
    pub fn label(self) -> &'static str {
        match self {
            LineEndingStyle::WindowsCrlf => "Windows (CRLF)",
            LineEndingStyle::UnixLf => "Unix (LF)",
        }
    }

    pub fn toggle(self) -> Self {
        match self {
            LineEndingStyle::WindowsCrlf => LineEndingStyle::UnixLf,
            LineEndingStyle::UnixLf => LineEndingStyle::WindowsCrlf,
        }
    }

    pub fn detect(text: &str) -> Self {
        if text.contains("\r\n") {
            LineEndingStyle::WindowsCrlf
        } else if text.contains('\n') {
            LineEndingStyle::UnixLf
        } else {
            LineEndingStyle::WindowsCrlf
        }
    }

    pub fn normalize(self, text: &str) -> String {
        let lf = text.replace("\r\n", "\n");
        match self {
            LineEndingStyle::WindowsCrlf => lf.replace('\n', "\r\n"),
            LineEndingStyle::UnixLf => lf,
        }
    }
}

/// Lightweight formatting actions matching the Windows 11 Notepad formatting toolbar.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FormatAction {
    Heading1,
    Heading2,
    Heading3,
    Heading4,
    Heading5,
    Heading6,
    BulletList,
    NumberedList,
    Bold,
    Italic,
    Strikethrough,
    Code,
    Link,
    ClearFormatting,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LastEditOp {
    None,
    InsertChar(char),
    Backspace,
    Delete,
    Bulk,
}

/// A single open document tab with O(1) cached status metrics.
pub struct Tab {
    pub id: usize,
    pub title: String,
    pub path: Option<PathBuf>,
    pub file_type: FileType,
    pub content: text_editor::Content,
    pub is_dirty: bool,
    pub line_ending: LineEndingStyle,
    pub formatted_view: bool,

    // Cached metrics so `view()` performs zero full-document scans.
    pub char_count: usize,
    pub char_count_formatted: String,
    pub cursor_line: usize,
    pub cursor_col: usize,

    // Bounded undo/redo snapshot stacks.
    undo_stack: VecDeque<String>,
    redo_stack: VecDeque<String>,
    last_edit: LastEditOp,
    consecutive_typing_count: u16,
}

impl Tab {
    pub fn new(id: usize, title: impl Into<String>, initial_text: &str) -> Self {
        let title = title.into();
        let file_type = FileType::from_filename(&title);
        let line_ending = LineEndingStyle::detect(initial_text);
        let content = text_editor::Content::with_text(initial_text);
        let mut tab = Self {
            id,
            title,
            path: None,
            file_type,
            content,
            is_dirty: false,
            line_ending,
            formatted_view: false,
            char_count: 0,
            char_count_formatted: String::new(),
            cursor_line: 1,
            cursor_col: 1,
            undo_stack: VecDeque::with_capacity(MAX_UNDO_HISTORY),
            redo_stack: VecDeque::with_capacity(MAX_UNDO_HISTORY),
            last_edit: LastEditOp::None,
            consecutive_typing_count: 0,
        };
        tab.recompute_metrics();
        tab
    }

    pub fn from_file(id: usize, name: String, path: Option<PathBuf>, text: &str) -> Self {
        let mut tab = Self::new(id, name, text);
        tab.path = path;
        tab.is_dirty = false;
        tab
    }

    pub fn update_title_and_type(&mut self, new_title: String) {
        self.file_type = FileType::from_filename(&new_title);
        self.title = new_title;
    }

    /// Performs a `text_editor::Action` with O(1) metrics updates and intelligent undo chunking.
    pub fn perform(&mut self, action: text_editor::Action) {
        let is_edit = action.is_edit();
        if is_edit {
            let cursor = self.content.cursor();
            let has_selection = cursor.selection.is_some();

            // Intelligent undo grouping (coalesce consecutive typing into logical words)
            let should_snapshot = match &action {
                text_editor::Action::Edit(text_editor::Edit::Insert(c)) => {
                    if has_selection || self.undo_stack.is_empty() {
                        true
                    } else if c.is_whitespace() {
                        true
                    } else if !matches!(self.last_edit, LastEditOp::InsertChar(_)) {
                        true
                    } else {
                        self.consecutive_typing_count >= 24
                    }
                }
                text_editor::Action::Edit(text_editor::Edit::Backspace) => {
                    has_selection || !matches!(self.last_edit, LastEditOp::Backspace) || self.undo_stack.is_empty()
                }
                text_editor::Action::Edit(text_editor::Edit::Delete) => {
                    has_selection || !matches!(self.last_edit, LastEditOp::Delete) || self.undo_stack.is_empty()
                }
                _ => true,
            };

            if should_snapshot {
                self.push_undo_snapshot();
                self.consecutive_typing_count = 0;
            }

            match &action {
                text_editor::Action::Edit(text_editor::Edit::Insert(c)) => {
                    self.last_edit = LastEditOp::InsertChar(*c);
                    self.consecutive_typing_count = self.consecutive_typing_count.saturating_add(1);
                }
                text_editor::Action::Edit(text_editor::Edit::Backspace) => {
                    self.last_edit = LastEditOp::Backspace;
                }
                text_editor::Action::Edit(text_editor::Edit::Delete) => {
                    self.last_edit = LastEditOp::Delete;
                }
                _ => {
                    self.last_edit = LastEditOp::Bulk;
                    self.consecutive_typing_count = 0;
                }
            }

            // O(1) fast-path metric updates for single-character edits without selection
            let can_fast_path = !has_selection;
            match &action {
                text_editor::Action::Edit(text_editor::Edit::Insert(c))
                    if can_fast_path && *c != '\n' =>
                {
                    self.content.perform(action);
                    self.is_dirty = true;
                    self.char_count += 1;
                    self.char_count_formatted = format_with_commas(self.char_count);
                    self.update_cursor_only();
                    return;
                }
                text_editor::Action::Edit(text_editor::Edit::Backspace)
                    if can_fast_path && cursor.position.column > 0 =>
                {
                    self.content.perform(action);
                    self.is_dirty = true;
                    self.char_count = self.char_count.saturating_sub(1);
                    self.char_count_formatted = format_with_commas(self.char_count);
                    self.update_cursor_only();
                    return;
                }
                _ => {}
            }

            self.content.perform(action);
            self.is_dirty = true;
            self.recompute_metrics();
        } else {
            self.content.perform(action);
            self.update_cursor_only();
        }
    }

    #[inline]
    pub fn update_cursor_only(&mut self) {
        let cursor = self.content.cursor();
        self.cursor_line = cursor.position.line + 1;
        self.cursor_col = cursor.position.column + 1;
    }

    pub fn recompute_metrics(&mut self) {
        let mut count = 0usize;
        for line in self.content.lines() {
            count += if line.text.is_ascii() {
                line.text.len()
            } else {
                line.text.chars().count()
            };
        }
        if count != self.char_count || self.char_count_formatted.is_empty() {
            self.char_count = count;
            self.char_count_formatted = format_with_commas(count);
        }
        self.update_cursor_only();
    }

    fn push_undo_snapshot(&mut self) {
        let snapshot = self.content.text();
        if self.undo_stack.back().is_some_and(|last| last == &snapshot) {
            return;
        }
        if self.undo_stack.len() >= MAX_UNDO_HISTORY {
            self.undo_stack.pop_front();
        }
        self.undo_stack.push_back(snapshot);
        self.redo_stack.clear();
    }

    pub fn undo(&mut self) -> bool {
        if let Some(prev) = self.undo_stack.pop_back() {
            let current = self.content.text();
            if self.redo_stack.len() >= MAX_UNDO_HISTORY {
                self.redo_stack.pop_front();
            }
            self.redo_stack.push_back(current);
            self.content = text_editor::Content::with_text(&prev);
            self.is_dirty = true;
            self.recompute_metrics();
            true
        } else {
            false
        }
    }

    pub fn redo(&mut self) -> bool {
        if let Some(next) = self.redo_stack.pop_back() {
            let current = self.content.text();
            if self.undo_stack.len() >= MAX_UNDO_HISTORY {
                self.undo_stack.pop_front();
            }
            self.undo_stack.push_back(current);
            self.content = text_editor::Content::with_text(&next);
            self.is_dirty = true;
            self.recompute_metrics();
            true
        } else {
            false
        }
    }

    pub fn insert_text(&mut self, text: &str) {
        self.push_undo_snapshot();
        self.content
            .perform(text_editor::Action::Edit(text_editor::Edit::Paste(
                std::sync::Arc::new(text.to_string()),
            )));
        self.is_dirty = true;
        self.recompute_metrics();
    }

    /// Applies lightweight Markdown formatting matching Windows 11 Notepad's toolbar.
    pub fn apply_format(&mut self, fmt: FormatAction) {
        if fmt == FormatAction::ClearFormatting {
            self.push_undo_snapshot();
            let selected = self.content.selection().unwrap_or_default();
            if !selected.is_empty() {
                let cleaned = strip_markdown_formatting(&selected);
                self.content
                    .perform(text_editor::Action::Edit(text_editor::Edit::Paste(
                        std::sync::Arc::new(cleaned),
                    )));
            } else {
                let raw = self.content.text();
                let cleaned = strip_markdown_formatting(&raw);
                self.content = text_editor::Content::with_text(&cleaned);
            }
            self.is_dirty = true;
            self.recompute_metrics();
            return;
        }

        match fmt {
            FormatAction::Heading1 => self.apply_heading(1),
            FormatAction::Heading2 => self.apply_heading(2),
            FormatAction::Heading3 => self.apply_heading(3),
            FormatAction::Heading4 => self.apply_heading(4),
            FormatAction::Heading5 => self.apply_heading(5),
            FormatAction::Heading6 => self.apply_heading(6),
            FormatAction::BulletList => self.apply_bullet_list(),
            FormatAction::NumberedList => self.apply_numbered_list(),
            FormatAction::Bold => self.apply_inline_wrapper("**", "**", "bold"),
            FormatAction::Italic => self.apply_inline_wrapper("*", "*", "italic"),
            FormatAction::Strikethrough => self.apply_inline_wrapper("~~", "~~", "strikethrough"),
            FormatAction::Code => self.apply_code(),
            FormatAction::Link => self.apply_link(),
            FormatAction::ClearFormatting => unreachable!(),
        }
    }

    fn apply_heading(&mut self, level: usize) {
        self.push_undo_snapshot();
        let prefix = format!("{} ", "#".repeat(level.clamp(1, 6)));
        let cursor = self.content.cursor();

        if let Some(selected) = self.content.selection() {
            if !selected.is_empty() {
                let formatted = selected
                    .lines()
                    .map(|l| {
                        let trimmed = l.trim_start();
                        let clean = strip_heading_prefix(trimmed);
                        format!("{prefix}{clean}")
                    })
                    .collect::<Vec<_>>()
                    .join("\n");
                self.content
                    .perform(text_editor::Action::Edit(text_editor::Edit::Paste(
                        std::sync::Arc::new(formatted),
                    )));
                self.is_dirty = true;
                self.recompute_metrics();
                return;
            }
        }

        // Apply to current line
        let line_idx = cursor.position.line;
        let line_text = self
            .content
            .line(line_idx)
            .map(|l| l.text.to_string())
            .unwrap_or_default();

        let trimmed = line_text.trim_start();
        let new_line = if trimmed.starts_with(&prefix) {
            strip_heading_prefix(trimmed).to_string()
        } else if trimmed.starts_with('#') {
            let clean = strip_heading_prefix(trimmed);
            format!("{prefix}{clean}")
        } else if trimmed.is_empty() {
            prefix
        } else {
            format!("{prefix}{trimmed}")
        };

        let line_char_count = line_text.chars().count();
        let mut target_cursor = cursor;
        target_cursor.position = Position {
            line: line_idx,
            column: line_char_count,
        };
        target_cursor.selection = Some(Position {
            line: line_idx,
            column: 0,
        });
        self.content.move_to(target_cursor);
        self.content
            .perform(text_editor::Action::Edit(text_editor::Edit::Paste(
                std::sync::Arc::new(new_line),
            )));
        self.is_dirty = true;
        self.recompute_metrics();
    }

    fn apply_bullet_list(&mut self) {
        self.push_undo_snapshot();
        let cursor = self.content.cursor();

        if let Some(selected) = self.content.selection() {
            if !selected.is_empty() {
                let formatted = selected
                    .lines()
                    .map(|l| {
                        let trimmed = l.trim_start();
                        if let Some(rest) = trimmed.strip_prefix("- ").or_else(|| trimmed.strip_prefix("* ")) {
                            rest.to_string()
                        } else {
                            format!("- {trimmed}")
                        }
                    })
                    .collect::<Vec<_>>()
                    .join("\n");
                self.content
                    .perform(text_editor::Action::Edit(text_editor::Edit::Paste(
                        std::sync::Arc::new(formatted),
                    )));
                self.is_dirty = true;
                self.recompute_metrics();
                return;
            }
        }

        let line_idx = cursor.position.line;
        let line_text = self
            .content
            .line(line_idx)
            .map(|l| l.text.to_string())
            .unwrap_or_default();

        let trimmed = line_text.trim_start();
        let new_line = if let Some(rest) = trimmed.strip_prefix("- ").or_else(|| trimmed.strip_prefix("* ")) {
            rest.to_string()
        } else if trimmed.is_empty() {
            "- ".to_string()
        } else {
            format!("- {trimmed}")
        };

        let line_char_count = line_text.chars().count();
        let mut target_cursor = cursor;
        target_cursor.position = Position {
            line: line_idx,
            column: line_char_count,
        };
        target_cursor.selection = Some(Position {
            line: line_idx,
            column: 0,
        });
        self.content.move_to(target_cursor);
        self.content
            .perform(text_editor::Action::Edit(text_editor::Edit::Paste(
                std::sync::Arc::new(new_line),
            )));
        self.is_dirty = true;
        self.recompute_metrics();
    }

    fn apply_numbered_list(&mut self) {
        self.push_undo_snapshot();
        let cursor = self.content.cursor();

        if let Some(selected) = self.content.selection() {
            if !selected.is_empty() {
                let formatted = selected
                    .lines()
                    .enumerate()
                    .map(|(i, l)| {
                        let trimmed = l.trim_start();
                        if let Some(pos) = trimmed.find(". ") {
                            if trimmed[..pos].chars().all(|c| c.is_ascii_digit()) {
                                return trimmed[pos + 2..].to_string();
                            }
                        }
                        format!("{}. {trimmed}", i + 1)
                    })
                    .collect::<Vec<_>>()
                    .join("\n");
                self.content
                    .perform(text_editor::Action::Edit(text_editor::Edit::Paste(
                        std::sync::Arc::new(formatted),
                    )));
                self.is_dirty = true;
                self.recompute_metrics();
                return;
            }
        }

        let line_idx = cursor.position.line;
        let line_text = self
            .content
            .line(line_idx)
            .map(|l| l.text.to_string())
            .unwrap_or_default();

        let trimmed = line_text.trim_start();
        let new_line = if let Some(pos) = trimmed.find(". ") {
            if trimmed[..pos].chars().all(|c| c.is_ascii_digit()) {
                trimmed[pos + 2..].to_string()
            } else {
                format!("1. {trimmed}")
            }
        } else if trimmed.is_empty() {
            "1. ".to_string()
        } else {
            format!("1. {trimmed}")
        };

        let line_char_count = line_text.chars().count();
        let mut target_cursor = cursor;
        target_cursor.position = Position {
            line: line_idx,
            column: line_char_count,
        };
        target_cursor.selection = Some(Position {
            line: line_idx,
            column: 0,
        });
        self.content.move_to(target_cursor);
        self.content
            .perform(text_editor::Action::Edit(text_editor::Edit::Paste(
                std::sync::Arc::new(new_line),
            )));
        self.is_dirty = true;
        self.recompute_metrics();
    }

    fn apply_inline_wrapper(&mut self, open: &str, close: &str, default_placeholder: &str) {
        self.push_undo_snapshot();
        let selected = self.content.selection().unwrap_or_default();

        if !selected.is_empty() {
            let replacement = if selected.starts_with(open)
                && selected.ends_with(close)
                && selected.len() >= open.len() + close.len()
            {
                selected[open.len()..selected.len() - close.len()].to_string()
            } else {
                format!("{open}{selected}{close}")
            };
            self.content
                .perform(text_editor::Action::Edit(text_editor::Edit::Paste(
                    std::sync::Arc::new(replacement),
                )));
        } else {
            let cursor = self.content.cursor();
            let line_idx = cursor.position.line;
            let col = cursor.position.column;
            let line_text = self
                .content
                .line(line_idx)
                .map(|l| l.text.to_string())
                .unwrap_or_default();

            let chars: Vec<char> = line_text.chars().collect();
            if col <= chars.len() && !chars.is_empty() {
                let mut start = col;
                while start > 0 && !chars[start - 1].is_whitespace() {
                    start -= 1;
                }
                let mut end = col;
                while end < chars.len() && !chars[end].is_whitespace() {
                    end += 1;
                }

                if start < end {
                    let word: String = chars[start..end].iter().collect();
                    let replacement = if word.starts_with(open)
                        && word.ends_with(close)
                        && word.len() >= open.len() + close.len()
                    {
                        word[open.len()..word.len() - close.len()].to_string()
                    } else {
                        format!("{open}{word}{close}")
                    };

                    let mut target_cursor = cursor;
                    target_cursor.position = Position {
                        line: line_idx,
                        column: end,
                    };
                    target_cursor.selection = Some(Position {
                        line: line_idx,
                        column: start,
                    });
                    self.content.move_to(target_cursor);
                    self.content
                        .perform(text_editor::Action::Edit(text_editor::Edit::Paste(
                            std::sync::Arc::new(replacement),
                        )));
                    self.is_dirty = true;
                    self.recompute_metrics();
                    return;
                }
            }

            let wrapper = format!("{open}{default_placeholder}{close}");
            self.content
                .perform(text_editor::Action::Edit(text_editor::Edit::Paste(
                    std::sync::Arc::new(wrapper),
                )));
        }
        self.is_dirty = true;
        self.recompute_metrics();
    }

    fn apply_code(&mut self) {
        self.push_undo_snapshot();
        let selected = self.content.selection().unwrap_or_default();
        if selected.contains('\n') {
            let replacement = format!("```\n{selected}\n```");
            self.content
                .perform(text_editor::Action::Edit(text_editor::Edit::Paste(
                    std::sync::Arc::new(replacement),
                )));
        } else if !selected.is_empty() {
            let replacement = if selected.starts_with('`') && selected.ends_with('`') && selected.len() >= 2 {
                selected[1..selected.len() - 1].to_string()
            } else {
                format!("`{selected}`")
            };
            self.content
                .perform(text_editor::Action::Edit(text_editor::Edit::Paste(
                    std::sync::Arc::new(replacement),
                )));
        } else {
            self.apply_inline_wrapper("`", "`", "code");
        }
        self.is_dirty = true;
        self.recompute_metrics();
    }

    fn apply_link(&mut self) {
        self.push_undo_snapshot();
        let selected = self.content.selection().unwrap_or_default();
        let replacement = if selected.starts_with("http://") || selected.starts_with("https://") {
            format!("[link]({selected})")
        } else if !selected.is_empty() {
            format!("[{selected}](https://example.com)")
        } else {
            "[link text](https://example.com)".to_string()
        };
        self.content
            .perform(text_editor::Action::Edit(text_editor::Edit::Paste(
                std::sync::Arc::new(replacement),
            )));
        self.is_dirty = true;
        self.recompute_metrics();
    }

    pub fn text_for_save(&self) -> String {
        self.line_ending.normalize(&self.content.text())
    }

    /// Counts occurrences of `query` in the document with zero full-document clones.
    pub fn count_matches(&self, query: &str, match_case: bool) -> usize {
        if query.is_empty() {
            return 0;
        }
        let mut total = 0;
        if match_case {
            for line in self.content.lines() {
                total += line.text.matches(query).count();
            }
        } else if query.is_ascii() {
            let query_lower = query.to_ascii_lowercase();
            let needle = query_lower.as_bytes();
            for line in self.content.lines() {
                let bytes = line.text.as_bytes();
                if bytes.len() >= needle.len() {
                    let mut i = 0;
                    while i + needle.len() <= bytes.len() {
                        if bytes[i..i + needle.len()].eq_ignore_ascii_case(needle) {
                            total += 1;
                            i += needle.len();
                        } else {
                            i += 1;
                        }
                    }
                }
            }
        } else {
            let query_lower = query.to_lowercase();
            for line in self.content.lines() {
                let line_lower = line.text.to_lowercase();
                total += line_lower.matches(&query_lower).count();
            }
        }
        total
    }

    /// Replaces the first or all occurrences of `query` with `replacement`.
    pub fn replace_occurrences(
        &mut self,
        query: &str,
        replacement: &str,
        match_case: bool,
        all: bool,
    ) -> usize {
        if query.is_empty() {
            return 0;
        }
        let source = self.content.text();
        let (updated, count) = if match_case {
            if all {
                let c = source.matches(query).count();
                if c == 0 {
                    return 0;
                }
                (source.replace(query, replacement), c)
            } else if source.contains(query) {
                (source.replacen(query, replacement, 1), 1)
            } else {
                return 0;
            }
        } else {
            replace_case_insensitive(&source, query, replacement, all)
        };

        if count > 0 {
            self.push_undo_snapshot();
            self.content = text_editor::Content::with_text(&updated);
            self.is_dirty = true;
            self.recompute_metrics();
        }
        count
    }

    /// Moves the cursor to the N-th (0-based) match of `query` without full document string copies.
    pub fn jump_to_match(&mut self, query: &str, match_case: bool, index: usize) {
        if query.is_empty() {
            return;
        }
        let total = self.count_matches(query, match_case);
        if total == 0 {
            return;
        }
        let target_match = index % total;
        let mut current_match = 0;
        let mut target_pos: Option<(usize, usize)> = None;

        let query_lower = if match_case {
            String::new()
        } else {
            query.to_lowercase()
        };
        let is_ascii_search = !match_case && query.is_ascii();

        'outer: for (line_idx, line) in self.content.lines().enumerate() {
            let line_text = line.text;
            if match_case {
                for (byte_offset, _) in line_text.match_indices(query) {
                    if current_match == target_match {
                        let col = line_text[..byte_offset].chars().count();
                        target_pos = Some((line_idx, col));
                        break 'outer;
                    }
                    current_match += 1;
                }
            } else if is_ascii_search {
                let bytes = line_text.as_bytes();
                let needle = query_lower.as_bytes();
                let mut offset = 0;
                while offset + needle.len() <= bytes.len() {
                    if bytes[offset..offset + needle.len()].eq_ignore_ascii_case(needle) {
                        if current_match == target_match {
                            let col = line_text[..offset].chars().count();
                            target_pos = Some((line_idx, col));
                            break 'outer;
                        }
                        current_match += 1;
                        offset += needle.len();
                    } else {
                        offset += 1;
                    }
                }
            } else {
                let line_lower = line_text.to_lowercase();
                for (byte_offset, _) in line_lower.match_indices(&query_lower) {
                    if current_match == target_match {
                        let col = line_text
                            .char_indices()
                            .take_while(|&(b_idx, _)| b_idx < byte_offset)
                            .count();
                        target_pos = Some((line_idx, col));
                        break 'outer;
                    }
                    current_match += 1;
                }
            }
        }

        if let Some((line, column)) = target_pos {
            self.content.move_to(text_editor::Cursor {
                position: text_editor::Position { line, column },
                selection: None,
            });
            self.update_cursor_only();
        }
    }
}

pub fn format_with_commas(mut n: usize) -> String {
    if n == 0 {
        return "0".to_string();
    }
    let mut buf = [0u8; 32];
    let mut pos = 32;
    let mut count = 0;

    while n > 0 {
        if count > 0 && count % 3 == 0 {
            pos -= 1;
            buf[pos] = b',';
        }
        pos -= 1;
        buf[pos] = b'0' + (n % 10) as u8;
        n /= 10;
        count += 1;
    }

    // Safety: buf[pos..32] contains only ASCII '0'..'9' and ','
    let s = unsafe { std::str::from_utf8_unchecked(&buf[pos..32]) };
    s.to_string()
}

fn strip_heading_prefix(s: &str) -> &str {
    let bytes = s.as_bytes();
    let mut count = 0;
    while count < bytes.len() && bytes[count] == b'#' {
        count += 1;
    }
    if count > 0 && count <= 6 {
        s[count..].trim_start()
    } else {
        s
    }
}

fn strip_markdown_formatting(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut first = true;

    for line in input.lines() {
        if !first {
            out.push('\n');
        }
        first = false;

        let trimmed = line.trim_start();
        let stripped = strip_heading_prefix(trimmed);
        let stripped = if let Some(rest) = stripped.strip_prefix("- ").or_else(|| stripped.strip_prefix("* ")) {
            rest
        } else if let Some(pos) = stripped.find(". ") {
            if stripped[..pos].bytes().all(|b| b.is_ascii_digit()) {
                &stripped[pos + 2..]
            } else {
                stripped
            }
        } else {
            stripped
        };

        let mut chars = stripped.chars().peekable();
        while let Some(ch) = chars.next() {
            if ch == '*' && chars.peek() == Some(&'*') {
                chars.next();
                continue;
            }
            if ch == '~' && chars.peek() == Some(&'~') {
                chars.next();
                continue;
            }
            if ch == '`' {
                continue;
            }
            out.push(ch);
        }
    }
    out
}

fn replace_case_insensitive(
    haystack: &str,
    needle: &str,
    replacement: &str,
    all: bool,
) -> (String, usize) {
    if needle.is_empty() {
        return (haystack.to_string(), 0);
    }

    let lower_haystack = haystack.to_lowercase();
    let lower_needle = needle.to_lowercase();

    if !lower_haystack.contains(&lower_needle) {
        return (haystack.to_string(), 0);
    }

    let mut result = String::with_capacity(haystack.len());
    let mut last_end = 0;
    let mut count = 0;

    for (start, matched) in lower_haystack.match_indices(&lower_needle) {
        if start < last_end || start > haystack.len() {
            continue;
        }
        let end = (start + matched.len()).min(haystack.len());
        if !haystack.is_char_boundary(start) || !haystack.is_char_boundary(end) {
            continue;
        }
        result.push_str(&haystack[last_end..start]);
        result.push_str(replacement);
        last_end = end;
        count += 1;
        if !all {
            break;
        }
    }

    if count == 0 {
        return (haystack.to_string(), 0);
    }

    result.push_str(&haystack[last_end..]);
    (result, count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_heading_prefix() {
        assert_eq!(strip_heading_prefix("# Title"), "Title");
        assert_eq!(strip_heading_prefix("## Subtitle"), "Subtitle");
        assert_eq!(strip_heading_prefix("###### Small Heading"), "Small Heading");
        assert_eq!(strip_heading_prefix("####### Too many hashes"), "####### Too many hashes");
        assert_eq!(strip_heading_prefix("No hash"), "No hash");
    }

    #[test]
    fn test_strip_markdown_formatting() {
        let md = "# Heading 1\n- Item 1\n1. Numbered\n**Bold** and ~~Strike~~ and `code`";
        let cleaned = strip_markdown_formatting(md);
        assert_eq!(cleaned, "Heading 1\nItem 1\nNumbered\nBold and Strike and code");
    }

    #[test]
    fn test_headings_apply_and_toggle() {
        let mut tab = Tab::new(1, "test.md", "My Heading");
        tab.apply_format(FormatAction::Heading1);
        assert_eq!(tab.content.text(), "# My Heading");

        // Changing to H2
        tab.apply_format(FormatAction::Heading2);
        assert_eq!(tab.content.text(), "## My Heading");

        // Changing to H6
        tab.apply_format(FormatAction::Heading6);
        assert_eq!(tab.content.text(), "###### My Heading");

        // Clicking H6 again toggles it off
        tab.apply_format(FormatAction::Heading6);
        assert_eq!(tab.content.text(), "My Heading");
    }

    #[test]
    fn test_inline_formatting() {
        let mut tab = Tab::new(1, "test.txt", "hello");
        tab.apply_format(FormatAction::Bold);
        assert_eq!(tab.content.text(), "**hello**");

        // Applying bold again unwraps it
        tab.apply_format(FormatAction::Bold);
        assert_eq!(tab.content.text(), "hello");
    }

    #[test]
    fn test_benchmark_baseline() {
        use std::time::Instant;

        // 1. Create a 5,000-line document
        let sample_line = "The quick brown fox jumps over the lazy dog. 1234567890\n";
        let doc_text = sample_line.repeat(5000);
        let mut tab = Tab::new(1, "benchmark.txt", &doc_text);

        // Benchmark typing 500 characters
        let start_type = Instant::now();
        for _ in 0..500 {
            tab.perform(text_editor::Action::Edit(text_editor::Edit::Insert('a')));
        }
        let typing_duration = start_type.elapsed();
        println!("BASELINE: Typing 500 chars into 5000 lines: {:?}", typing_duration);

        // Benchmark count matches (case-insensitive)
        let start_find = Instant::now();
        let matches = tab.count_matches("brown", false);
        let find_duration = start_find.elapsed();
        assert_eq!(matches, 5000);
        println!("BASELINE: Case-insensitive find in 5000 lines: {:?}", find_duration);

        // Benchmark format_with_commas
        let start_commas = Instant::now();
        for i in 1..=10_000 {
            let _ = format_with_commas(i * 123);
        }
        let commas_duration = start_commas.elapsed();
        println!("BASELINE: format_with_commas 10,000 iterations: {:?}", commas_duration);
    }
}
