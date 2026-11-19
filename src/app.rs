use iced::widget::{column, rule, stack, text_editor};
use iced::{Element, Event, Length, Subscription, Task, Theme, event, keyboard, system, window};

use crate::document::{FormatAction, Tab};
use crate::io::{self, IoError, LoadedFile, SavedFile};
use crate::settings::{EditorFontFamily, UserSettings};
use crate::theme::{self, AppTheme, FluentPalette};
use crate::ui;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActiveMenu {
    File,
    Edit,
    View,
}

#[derive(Debug, Clone)]
pub enum Message {
    // Editor & Tabs
    EditorAction(text_editor::Action),
    NewTab,
    SelectTab(usize),
    CloseTab(usize),
    CloseActiveTab,

    // File I/O (Async Native & WASM)
    OpenFile,
    FileOpened(Result<LoadedFile, IoError>),
    SaveFile,
    SaveFileAs,
    FileSaved(usize, Result<SavedFile, IoError>),

    // Edit & Formatting
    Undo,
    Redo,
    InsertTimeDate,
    ApplyFormat(FormatAction),
    ToggleFormattedView,
    ToggleLineEnding,

    // Find & Replace
    OpenFind,
    OpenReplace,
    CloseFindReplace,
    FindQueryChanged(String),
    ReplaceQueryChanged(String),
    ToggleFindMatchCase,
    FindNext,
    FindPrevious,
    ReplaceOne,
    ReplaceAll,

    // View & Zoom
    ZoomIn,
    ZoomOut,
    ZoomReset,
    ToggleWordWrap,
    ToggleStatusBar,
    ToggleFormattingToolbar,
    ToggleSyntaxHighlighting,

    // Menus & Settings
    ToggleMenu(ActiveMenu),
    DismissOverlays,
    ToggleSettings,
    SetTheme(AppTheme),
    SetFontFamily(EditorFontFamily),
    IncreaseBaseFontSize,
    DecreaseBaseFontSize,

    // Window Controls (Native Desktop)
    DragWindow,
    MinimizeWindow,
    MaximizeWindow,
    CloseWindow,

    // Global Events
    RuntimeEvent(Event),
    SystemThemeChanged(iced::theme::Mode),
}

pub struct NotepadApp {
    pub tabs: Vec<Tab>,
    pub active_tab: usize,
    next_tab_id: usize,

    pub settings: UserSettings,
    pub system_dark: bool,
    pub zoom_percent: u16,

    pub open_menu: Option<ActiveMenu>,
    pub show_settings: bool,

    // Find & Replace state
    pub show_find: bool,
    pub show_replace: bool,
    pub find_query: String,
    pub replace_query: String,
    pub find_match_case: bool,
    pub find_match_index: usize,
    pub find_total_matches: std::cell::Cell<Option<usize>>,
}

impl NotepadApp {
    pub fn boot() -> (Self, Task<Message>) {
        let settings = UserSettings::load();
        let initial_tab = Tab::new(1, "Untitled", "");

        let mut app = Self {
            tabs: vec![initial_tab],
            active_tab: 0,
            next_tab_id: 2,
            settings,
            system_dark: detect_initial_dark_mode(),
            zoom_percent: 100,
            open_menu: None,
            show_settings: false,
            show_find: false,
            show_replace: false,
            find_query: String::new(),
            replace_query: String::new(),
            find_match_case: false,
            find_match_index: 0,
            find_total_matches: std::cell::Cell::new(None),
        };

        let sys_theme_task = system::theme().map(Message::SystemThemeChanged);

        // Check CLI argument on native desktop (e.g. `notepad notes.txt`).
        #[cfg(not(target_arch = "wasm32"))]
        if let Some(arg) = std::env::args().nth(1) {
            let path = std::path::PathBuf::from(&arg);
            if path.is_file() {
                return (
                    app,
                    Task::batch([
                        sys_theme_task,
                        Task::perform(io::read_path(path), Message::FileOpened),
                    ]),
                );
            } else if !arg.starts_with('-') {
                let name = path
                    .file_name()
                    .map(|s| s.to_string_lossy().into_owned())
                    .unwrap_or_else(|| arg.clone());
                let mut new_tab = Tab::new(1, name, "");
                new_tab.path = Some(path);
                app.tabs = vec![new_tab];
                return (app, sys_theme_task);
            }
        }

        (app, sys_theme_task)
    }

    pub fn title(&self) -> String {
        let tab = self.active_tab_ref();
        let prefix = if tab.is_dirty { "*" } else { "" };
        format!("{prefix}{} - Notepad", tab.title)
    }

    pub fn palette(&self) -> FluentPalette {
        self.settings.theme.palette(self.system_dark)
    }

    pub fn theme(&self) -> Theme {
        self.palette().iced_theme()
    }

    #[inline]
    pub fn active_tab_ref(&self) -> &Tab {
        &self.tabs[self.active_tab]
    }

    #[inline]
    pub fn active_tab_mut(&mut self) -> &mut Tab {
        &mut self.tabs[self.active_tab]
    }

    #[inline]
    pub fn effective_font_size(&self) -> f32 {
        (f32::from(self.settings.base_font_size) * f32::from(self.zoom_percent) / 100.0)
            .clamp(8.0, 96.0)
    }

    #[inline]
    pub fn find_matches_count(&self) -> usize {
        if let Some(count) = self.find_total_matches.get() {
            count
        } else {
            let count = self
                .active_tab_ref()
                .count_matches(&self.find_query, self.find_match_case);
            self.find_total_matches.set(Some(count));
            count
        }
    }

    #[inline]
    pub fn invalidate_find_matches(&self) {
        self.find_total_matches.set(None);
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::EditorAction(action) => {
                self.open_menu = None;
                if action.is_edit() {
                    self.invalidate_find_matches();
                }
                self.active_tab_mut().perform(action);
                Task::none()
            }

            Message::NewTab => {
                self.open_menu = None;
                self.show_settings = false;
                self.invalidate_find_matches();
                let id = self.next_tab_id;
                self.next_tab_id += 1;
                self.tabs.push(Tab::new(id, "Untitled", ""));
                self.active_tab = self.tabs.len() - 1;
                Task::none()
            }

            Message::SelectTab(idx) => {
                self.open_menu = None;
                self.show_settings = false;
                if idx < self.tabs.len() && idx != self.active_tab {
                    self.active_tab = idx;
                    self.invalidate_find_matches();
                }
                Task::none()
            }

            Message::CloseTab(idx) => {
                self.open_menu = None;
                self.invalidate_find_matches();
                if self.tabs.len() <= 1 {
                    let id = self.next_tab_id;
                    self.next_tab_id += 1;
                    self.tabs[0] = Tab::new(id, "Untitled", "");
                    self.active_tab = 0;
                } else if idx < self.tabs.len() {
                    self.tabs.remove(idx);
                    if idx < self.active_tab {
                        self.active_tab -= 1;
                    } else if self.active_tab >= self.tabs.len() {
                        self.active_tab = self.tabs.len() - 1;
                    }
                }
                Task::none()
            }

            Message::CloseActiveTab => {
                let idx = self.active_tab;
                self.update(Message::CloseTab(idx))
            }

            Message::OpenFile => {
                self.open_menu = None;
                Task::perform(io::open_file_dialog(), Message::FileOpened)
            }

            Message::FileOpened(Ok(file)) => {
                self.show_settings = false;
                let current = self.active_tab_ref();
                let reuse_current =
                    !current.is_dirty && current.path.is_none() && current.content.is_empty();

                if reuse_current {
                    let id = self.tabs[self.active_tab].id;
                    self.tabs[self.active_tab] =
                        Tab::from_file(id, file.name, file.path, &file.contents);
                } else {
                    let id = self.next_tab_id;
                    self.next_tab_id += 1;
                    self.tabs
                        .push(Tab::from_file(id, file.name, file.path, &file.contents));
                    self.active_tab = self.tabs.len() - 1;
                }
                Task::none()
            }

            Message::FileOpened(Err(_)) => Task::none(),

            Message::SaveFile => {
                self.open_menu = None;
                let tab = self.active_tab_ref();
                let tab_id = tab.id;
                let path = tab.path.clone();
                let suggested = ensure_extension(&tab.title);
                let text = tab.text_for_save();
                Task::perform(io::save_file(path, suggested, text), move |res| {
                    Message::FileSaved(tab_id, res)
                })
            }

            Message::SaveFileAs => {
                self.open_menu = None;
                let tab = self.active_tab_ref();
                let tab_id = tab.id;
                let suggested = ensure_extension(&tab.title);
                let text = tab.text_for_save();
                Task::perform(io::save_file(None, suggested, text), move |res| {
                    Message::FileSaved(tab_id, res)
                })
            }

            Message::FileSaved(tab_id, Ok(saved)) => {
                if let Some(tab) = self.tabs.iter_mut().find(|t| t.id == tab_id) {
                    tab.path = saved.path;
                    tab.update_title_and_type(saved.name);
                    tab.is_dirty = false;
                }
                Task::none()
            }

            Message::FileSaved(_, Err(_)) => Task::none(),

            Message::Undo => {
                self.open_menu = None;
                self.invalidate_find_matches();
                self.active_tab_mut().undo();
                Task::none()
            }

            Message::Redo => {
                self.open_menu = None;
                self.invalidate_find_matches();
                self.active_tab_mut().redo();
                Task::none()
            }

            Message::InsertTimeDate => {
                self.open_menu = None;
                self.invalidate_find_matches();
                let stamp = chrono::Local::now().format("%H:%M %Y-%m-%d").to_string();
                self.active_tab_mut().insert_text(&stamp);
                Task::none()
            }

            Message::ApplyFormat(fmt) => {
                self.open_menu = None;
                self.invalidate_find_matches();
                self.active_tab_mut().apply_format(fmt);
                Task::none()
            }

            Message::ToggleFormattedView => {
                self.open_menu = None;
                let tab = self.active_tab_mut();
                tab.formatted_view = !tab.formatted_view;
                Task::none()
            }

            Message::ToggleLineEnding => {
                let tab = self.active_tab_mut();
                tab.line_ending = tab.line_ending.toggle();
                tab.is_dirty = true;
                Task::none()
            }

            Message::OpenFind => {
                self.open_menu = None;
                self.show_settings = false;
                self.show_find = true;
                self.show_replace = false;
                self.invalidate_find_matches();
                Task::none()
            }

            Message::OpenReplace => {
                self.open_menu = None;
                self.show_settings = false;
                self.show_find = true;
                self.show_replace = true;
                self.invalidate_find_matches();
                Task::none()
            }

            Message::CloseFindReplace => {
                self.show_find = false;
                self.show_replace = false;
                Task::none()
            }

            Message::FindQueryChanged(q) => {
                self.find_query = q;
                self.find_match_index = 0;
                self.invalidate_find_matches();
                Task::none()
            }

            Message::ReplaceQueryChanged(q) => {
                self.replace_query = q;
                Task::none()
            }

            Message::ToggleFindMatchCase => {
                self.find_match_case = !self.find_match_case;
                self.find_match_index = 0;
                self.invalidate_find_matches();
                Task::none()
            }

            Message::FindNext => {
                let total = self.find_matches_count();
                if total > 0 {
                    let idx = self.find_match_index % total;
                    let tab = &mut self.tabs[self.active_tab];
                    tab.jump_to_match(&self.find_query, self.find_match_case, idx);
                    self.find_match_index = (idx + 1) % total;
                }
                Task::none()
            }

            Message::FindPrevious => {
                let total = self.find_matches_count();
                if total > 0 {
                    let idx = if self.find_match_index == 0 {
                        total - 1
                    } else {
                        (self.find_match_index - 1) % total
                    };
                    let tab = &mut self.tabs[self.active_tab];
                    tab.jump_to_match(&self.find_query, self.find_match_case, idx);
                    self.find_match_index = idx;
                }
                Task::none()
            }

            Message::ReplaceOne => {
                let query = self.find_query.clone();
                let repl = self.replace_query.clone();
                let match_case = self.find_match_case;
                self.active_tab_mut()
                    .replace_occurrences(&query, &repl, match_case, false);
                self.invalidate_find_matches();
                Task::none()
            }

            Message::ReplaceAll => {
                let query = self.find_query.clone();
                let repl = self.replace_query.clone();
                let match_case = self.find_match_case;
                self.active_tab_mut()
                    .replace_occurrences(&query, &repl, match_case, true);
                self.invalidate_find_matches();
                Task::none()
            }

            Message::ZoomIn => {
                self.open_menu = None;
                self.zoom_percent = (self.zoom_percent + 10).min(500);
                Task::none()
            }

            Message::ZoomOut => {
                self.open_menu = None;
                self.zoom_percent = self.zoom_percent.saturating_sub(10).max(10);
                Task::none()
            }

            Message::ZoomReset => {
                self.open_menu = None;
                self.zoom_percent = 100;
                Task::none()
            }

            Message::ToggleWordWrap => {
                self.open_menu = None;
                self.settings.word_wrap = !self.settings.word_wrap;
                self.settings.save();
                Task::none()
            }

            Message::ToggleStatusBar => {
                self.open_menu = None;
                self.settings.show_status_bar = !self.settings.show_status_bar;
                self.settings.save();
                Task::none()
            }

            Message::ToggleFormattingToolbar => {
                self.open_menu = None;
                self.settings.show_formatting_toolbar = !self.settings.show_formatting_toolbar;
                self.settings.save();
                Task::none()
            }

            Message::ToggleSyntaxHighlighting => {
                self.open_menu = None;
                self.settings.syntax_highlighting = !self.settings.syntax_highlighting;
                self.settings.save();
                Task::none()
            }

            Message::ToggleMenu(menu) => {
                self.open_menu = if self.open_menu == Some(menu) {
                    None
                } else {
                    Some(menu)
                };
                Task::none()
            }

            Message::DismissOverlays => {
                self.open_menu = None;
                self.show_settings = false;
                Task::none()
            }

            Message::ToggleSettings => {
                self.open_menu = None;
                self.show_settings = !self.show_settings;
                Task::none()
            }

            Message::SetTheme(t) => {
                self.settings.theme = t;
                self.settings.save();
                if t == AppTheme::System {
                    self.system_dark = detect_initial_dark_mode();
                }
                Task::none()
            }

            Message::SystemThemeChanged(mode) => {
                #[cfg(target_os = "linux")]
                {
                    let is_dark = detect_desktop_dark_mode();
                    self.system_dark = match mode {
                        iced::theme::Mode::Dark => true,
                        iced::theme::Mode::Light => is_dark,
                        iced::theme::Mode::None => is_dark,
                    };
                }
                #[cfg(not(target_os = "linux"))]
                {
                    match mode {
                        iced::theme::Mode::Dark => self.system_dark = true,
                        iced::theme::Mode::Light => self.system_dark = false,
                        iced::theme::Mode::None => {
                            self.system_dark = detect_initial_dark_mode();
                        }
                    }
                }
                Task::none()
            }

            Message::SetFontFamily(f) => {
                self.settings.font_family = f;
                self.settings.save();
                Task::none()
            }

            Message::IncreaseBaseFontSize => {
                self.settings.base_font_size = (self.settings.base_font_size + 1).min(48);
                self.settings.save();
                Task::none()
            }

            Message::DecreaseBaseFontSize => {
                self.settings.base_font_size =
                    self.settings.base_font_size.saturating_sub(1).max(8);
                self.settings.save();
                Task::none()
            }

            Message::DragWindow => {
                window::oldest().then(|id| id.map(window::drag).unwrap_or_else(Task::none))
            }

            Message::MinimizeWindow => window::oldest().then(|id| {
                id.map(|id| window::minimize(id, true))
                    .unwrap_or_else(Task::none)
            }),

            Message::MaximizeWindow => window::oldest()
                .then(|id| id.map(window::toggle_maximize).unwrap_or_else(Task::none)),

            Message::CloseWindow => {
                window::oldest().then(|id| id.map(window::close).unwrap_or_else(Task::none))
            }

            Message::RuntimeEvent(evt) => self.handle_runtime_event(evt),
        }
    }

    fn handle_runtime_event(&mut self, evt: Event) -> Task<Message> {
        match evt {
            Event::Keyboard(keyboard::Event::KeyPressed { key, modifiers, .. }) => {
                if modifiers.command() {
                    if let keyboard::Key::Character(ref ch) = key {
                        if modifiers.alt() {
                            match ch.as_str() {
                                "1" => {
                                    return self
                                        .update(Message::ApplyFormat(FormatAction::Heading1));
                                }
                                "2" => {
                                    return self
                                        .update(Message::ApplyFormat(FormatAction::Heading2));
                                }
                                "3" => {
                                    return self
                                        .update(Message::ApplyFormat(FormatAction::Heading3));
                                }
                                "4" => {
                                    return self
                                        .update(Message::ApplyFormat(FormatAction::Heading4));
                                }
                                "5" => {
                                    return self
                                        .update(Message::ApplyFormat(FormatAction::Heading5));
                                }
                                "6" => {
                                    return self
                                        .update(Message::ApplyFormat(FormatAction::Heading6));
                                }
                                _ => {}
                            }
                        }
                        match ch.as_str() {
                            "n" | "N" | "t" | "T" => return self.update(Message::NewTab),
                            "o" | "O" => return self.update(Message::OpenFile),
                            "s" | "S" => {
                                return if modifiers.shift() {
                                    self.update(Message::SaveFileAs)
                                } else {
                                    self.update(Message::SaveFile)
                                };
                            }
                            "w" | "W" => return self.update(Message::CloseActiveTab),
                            "f" | "F" => return self.update(Message::OpenFind),
                            "h" | "H" => return self.update(Message::OpenReplace),
                            "z" | "Z" => return self.update(Message::Undo),
                            "y" | "Y" => return self.update(Message::Redo),
                            "b" | "B" => {
                                return self.update(Message::ApplyFormat(FormatAction::Bold));
                            }
                            "i" | "I" => {
                                return self.update(Message::ApplyFormat(FormatAction::Italic));
                            }
                            "k" | "K" => {
                                return self.update(Message::ApplyFormat(FormatAction::Link));
                            }
                            "=" | "+" => return self.update(Message::ZoomIn),
                            "-" => return self.update(Message::ZoomOut),
                            "0" => return self.update(Message::ZoomReset),
                            "," => return self.update(Message::ToggleSettings),
                            _ => {}
                        }
                    }
                } else {
                    match key {
                        keyboard::Key::Named(keyboard::key::Named::Escape) => {
                            self.open_menu = None;
                            self.show_find = false;
                            self.show_replace = false;
                            if self.show_settings {
                                self.show_settings = false;
                            }
                        }
                        keyboard::Key::Named(keyboard::key::Named::F5) => {
                            return self.update(Message::InsertTimeDate);
                        }
                        _ => {}
                    }
                }
                Task::none()
            }

            #[cfg(not(target_arch = "wasm32"))]
            Event::Window(window::Event::FileDropped(path)) => {
                Task::perform(io::read_path(path), Message::FileOpened)
            }

            _ => Task::none(),
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::batch([
            event::listen().map(Message::RuntimeEvent),
            system::theme_changes().map(Message::SystemThemeChanged),
        ])
    }

    pub fn view(&self) -> Element<'_, Message> {
        let p = self.palette();

        let titlebar = ui::titlebar::view(self, p);
        let menubar = ui::menubar::view(self, p);

        let workspace: Element<'_, Message> = ui::editor::view(self, p);

        let mut base = column![
            titlebar,
            menubar,
            rule::horizontal(1).style(theme::hairline_rule(p)),
            workspace,
        ]
        .width(Length::Fill)
        .height(Length::Fill);

        if self.settings.show_status_bar {
            base = base
                .push(rule::horizontal(1).style(theme::hairline_rule(p)))
                .push(ui::statusbar::view(self, p));
        }

        let mut layers = vec![base.into()];

        if self.show_find {
            layers.push(ui::overlays::find_replace_bar(self, p));
        }

        if let Some(menu) = self.open_menu {
            layers.push(ui::overlays::menu_dropdown(self, menu, p));
        }

        if self.show_settings {
            layers.push(ui::overlays::settings_flyout(self, p));
        }

        stack(layers)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

fn ensure_extension(title: &str) -> String {
    if title.contains('.') {
        title.to_string()
    } else {
        format!("{title}.txt")
    }
}

fn detect_initial_dark_mode() -> bool {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(mql)) = window.match_media("(prefers-color-scheme: dark)") {
                return mql.matches();
            }
        }
        false
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        detect_desktop_dark_mode()
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn detect_desktop_dark_mode() -> bool {
    // 1. Environment variable check (instant)
    if let Ok(gtk_theme) = std::env::var("GTK_THEME") {
        let lower = gtk_theme.to_lowercase();
        if lower.contains("dark") {
            return true;
        }
    }

    // 2. GTK config files (~/.config/gtk-4.0/settings.ini, ~/.config/gtk-3.0/settings.ini) (instant file read, ~0.005ms)
    if let Some(config_dir) = dirs::config_dir() {
        for gtk_ver in &["gtk-4.0", "gtk-3.0"] {
            let path = config_dir.join(gtk_ver).join("settings.ini");
            if let Ok(content) = std::fs::read_to_string(path) {
                for line in content.lines() {
                    let trimmed = line.trim();
                    if trimmed.starts_with("gtk-application-prefer-dark-theme") {
                        if trimmed.ends_with("true") || trimmed.ends_with("1") {
                            return true;
                        }
                    } else if trimmed.starts_with("gtk-theme-name") {
                        let lower = trimmed.to_lowercase();
                        if lower.contains("dark") || lower.contains("-dark") {
                            return true;
                        }
                    }
                }
            }
        }

        // 3. KDE globals (~/.config/kdeglobals)
        let kde_globals = config_dir.join("kdeglobals");
        if let Ok(content) = std::fs::read_to_string(kde_globals) {
            let lower = content.to_lowercase();
            if lower.contains("breezedark") || lower.contains("color-scheme=dark") {
                return true;
            }
        }
    }

    // 4. Portal / GSettings via gsettings (sub-process fallback)
    if let Ok(output) = std::process::Command::new("gsettings")
        .args(["get", "org.gnome.desktop.interface", "color-scheme"])
        .output()
    {
        let stdout = String::from_utf8_lossy(&output.stdout);
        if stdout.contains("prefer-dark") {
            return true;
        }
        if stdout.contains("prefer-light") {
            return false;
        }
    }

    // 5. Freedesktop Portal settings via gdbus (sub-process fallback)
    if let Ok(output) = std::process::Command::new("gdbus")
        .args([
            "call",
            "--session",
            "--dest",
            "org.freedesktop.portal.Desktop",
            "--object-path",
            "/org/freedesktop/portal/desktop",
            "--method",
            "org.freedesktop.portal.Settings.Read",
            "org.freedesktop.appearance",
            "color-scheme",
        ])
        .output()
    {
        let stdout = String::from_utf8_lossy(&output.stdout);
        if stdout.contains("uint32 1") || stdout.contains("<1>") {
            return true;
        } else if stdout.contains("uint32 2") || stdout.contains("<2>") {
            return false;
        }
    }

    // 6. Windows registry
    #[cfg(target_os = "windows")]
    {
        if let Ok(output) = std::process::Command::new("reg")
            .args([
                "query",
                r"HKCU\Software\Microsoft\Windows\CurrentVersion\Themes\Personalize",
                "/v",
                "AppsUseLightTheme",
            ])
            .output()
        {
            let stdout = String::from_utf8_lossy(&output.stdout);
            if stdout.contains("0x0") {
                return true;
            }
        }
    }

    // 7. macOS defaults
    #[cfg(target_os = "macos")]
    {
        if let Ok(output) = std::process::Command::new("defaults")
            .args(["read", "-g", "AppleInterfaceStyle"])
            .output()
        {
            let stdout = String::from_utf8_lossy(&output.stdout);
            if stdout.trim().eq_ignore_ascii_case("dark") {
                return true;
            }
        }
    }

    false
}
