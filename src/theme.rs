use iced::widget::{button, container, pick_list, rule, text_editor, text_input, toggler};
use iced::{Background, Border, Color, Shadow, Theme, Vector};
use serde::{Deserialize, Serialize};

/// Theme preference matching Notepad Settings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum AppTheme {
    Light,
    Dark,
    Black,
    #[default]
    System,
}

impl AppTheme {
    pub const ALL: [AppTheme; 4] = [
        AppTheme::Light,
        AppTheme::Dark,
        AppTheme::Black,
        AppTheme::System,
    ];

    pub fn label(self) -> &'static str {
        match self {
            AppTheme::Light => "Light",
            AppTheme::Dark => "Dark",
            AppTheme::Black => "Black (AMOLED)",
            AppTheme::System => "Use system setting",
        }
    }

    #[allow(dead_code)]
    pub fn is_dark(self, system_dark: bool) -> bool {
        match self {
            AppTheme::Light => false,
            AppTheme::Dark | AppTheme::Black => true,
            AppTheme::System => system_dark,
        }
    }

    pub fn palette(self, system_dark: bool) -> FluentPalette {
        match self {
            AppTheme::Light => FluentPalette::LIGHT,
            AppTheme::Dark => FluentPalette::DARK,
            AppTheme::Black => FluentPalette::AMOLED,
            AppTheme::System => {
                if system_dark {
                    FluentPalette::DARK
                } else {
                    FluentPalette::LIGHT
                }
            }
        }
    }
}

impl std::fmt::Display for AppTheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.label())
    }
}

/// Monochrome color tokens (Light, Dark, and AMOLED Black).
#[derive(Debug, Clone, Copy)]
pub struct FluentPalette {
    pub is_dark: bool,
    /// Top titlebar / tab strip background.
    pub titlebar_bg: Color,
    /// Menu bar & active tab surface.
    pub surface_bg: Color,
    /// Main text editor canvas background.
    pub editor_bg: Color,
    /// Bottom status bar background.
    pub statusbar_bg: Color,
    /// Elevated overlay surface (dropdown menus, Find/Replace, Settings cards).
    pub overlay_bg: Color,
    /// Primary text color.
    pub text_primary: Color,
    /// Secondary / muted text color (inactive tabs, shortcuts, status bar).
    pub text_secondary: Color,
    /// Disabled / placeholder text color.
    pub text_muted: Color,
    /// Hairline divider & subtle border.
    pub border: Color,
    /// Elevated menu border.
    pub overlay_border: Color,
    /// Hover fill for subtle chrome buttons.
    pub hover_bg: Color,
    /// Pressed fill for subtle chrome buttons.
    pub pressed_bg: Color,
    /// Active/selected toggle button fill.
    pub active_toggle_bg: Color,
    /// Clean monochrome accent.
    pub accent: Color,
    /// Selection highlight in text editor & inputs.
    pub selection: Color,
    /// Close window button hover red (#C42B1C).
    pub close_hover_bg: Color,
    /// Input background (Find/Replace).
    pub input_bg: Color,
    /// Link color in formatted view.
    pub link: Color,
}

impl FluentPalette {
    pub const LIGHT: Self = Self {
        is_dark: false,
        titlebar_bg: Color::from_rgb8(0xEA, 0xEA, 0xEA),
        surface_bg: Color::from_rgb8(0xF6, 0xF6, 0xF6),
        editor_bg: Color::from_rgb8(0xFF, 0xFF, 0xFF),
        statusbar_bg: Color::from_rgb8(0xEE, 0xEE, 0xEE),
        overlay_bg: Color::from_rgb8(0xFA, 0xFA, 0xFA),
        text_primary: Color::from_rgb8(0x18, 0x18, 0x18),
        text_secondary: Color::from_rgb8(0x5A, 0x5A, 0x5A),
        text_muted: Color::from_rgb8(0x8A, 0x8A, 0x8A),
        border: Color::from_rgb8(0xDC, 0xDC, 0xDC),
        overlay_border: Color::from_rgb8(0xCE, 0xCE, 0xCE),
        hover_bg: Color::from_rgba8(0x00, 0x00, 0x00, 0.06),
        pressed_bg: Color::from_rgba8(0x00, 0x00, 0x00, 0.11),
        active_toggle_bg: Color::from_rgba8(0x00, 0x00, 0x00, 0.12),
        accent: Color::from_rgb8(0x18, 0x18, 0x18),
        selection: Color::from_rgba8(0x00, 0x00, 0x00, 0.18),
        close_hover_bg: Color::from_rgb8(0xC4, 0x2B, 0x1C),
        input_bg: Color::from_rgb8(0xFF, 0xFF, 0xFF),
        link: Color::from_rgb8(0x22, 0x22, 0x22),
    };

    pub const DARK: Self = Self {
        is_dark: true,
        titlebar_bg: Color::from_rgb8(0x1C, 0x1C, 0x1C),
        surface_bg: Color::from_rgb8(0x24, 0x24, 0x24),
        editor_bg: Color::from_rgb8(0x1F, 0x1F, 0x1F),
        statusbar_bg: Color::from_rgb8(0x1A, 0x1A, 0x1A),
        overlay_bg: Color::from_rgb8(0x26, 0x26, 0x26),
        text_primary: Color::from_rgb8(0xF2, 0xF2, 0xF2),
        text_secondary: Color::from_rgb8(0xB0, 0xB0, 0xB0),
        text_muted: Color::from_rgb8(0x75, 0x75, 0x75),
        border: Color::from_rgb8(0x33, 0x33, 0x33),
        overlay_border: Color::from_rgb8(0x40, 0x40, 0x40),
        hover_bg: Color::from_rgba8(0xFF, 0xFF, 0xFF, 0.08),
        pressed_bg: Color::from_rgba8(0xFF, 0xFF, 0xFF, 0.13),
        active_toggle_bg: Color::from_rgba8(0xFF, 0xFF, 0xFF, 0.18),
        accent: Color::from_rgb8(0xF0, 0xF0, 0xF0),
        selection: Color::from_rgba8(0xFF, 0xFF, 0xFF, 0.22),
        close_hover_bg: Color::from_rgb8(0xC4, 0x2B, 0x1C),
        input_bg: Color::from_rgb8(0x16, 0x16, 0x16),
        link: Color::from_rgb8(0xDD, 0xDD, 0xDD),
    };

    pub const AMOLED: Self = Self {
        is_dark: true,
        titlebar_bg: Color::from_rgb8(0x00, 0x00, 0x00),
        surface_bg: Color::from_rgb8(0x07, 0x07, 0x07),
        editor_bg: Color::from_rgb8(0x00, 0x00, 0x00),
        statusbar_bg: Color::from_rgb8(0x00, 0x00, 0x00),
        overlay_bg: Color::from_rgb8(0x0D, 0x0D, 0x0D),
        text_primary: Color::from_rgb8(0xFF, 0xFF, 0xFF),
        text_secondary: Color::from_rgb8(0xAD, 0xAD, 0xAD),
        text_muted: Color::from_rgb8(0x66, 0x66, 0x66),
        border: Color::from_rgb8(0x1D, 0x1D, 0x1D),
        overlay_border: Color::from_rgb8(0x2C, 0x2C, 0x2C),
        hover_bg: Color::from_rgba8(0xFF, 0xFF, 0xFF, 0.09),
        pressed_bg: Color::from_rgba8(0xFF, 0xFF, 0xFF, 0.15),
        active_toggle_bg: Color::from_rgba8(0xFF, 0xFF, 0xFF, 0.22),
        accent: Color::from_rgb8(0xFF, 0xFF, 0xFF),
        selection: Color::from_rgba8(0xFF, 0xFF, 0xFF, 0.26),
        close_hover_bg: Color::from_rgb8(0xC4, 0x2B, 0x1C),
        input_bg: Color::from_rgb8(0x04, 0x04, 0x04),
        link: Color::from_rgb8(0xEE, 0xEE, 0xEE),
    };

    #[allow(dead_code)]
    #[inline]
    pub const fn for_mode(dark: bool) -> Self {
        if dark { Self::DARK } else { Self::LIGHT }
    }

    pub fn iced_theme(self) -> Theme {
        Theme::custom(
            String::from(if self.is_dark {
                "Monochrome Dark"
            } else {
                "Monochrome Light"
            }),
            iced::theme::Palette {
                background: self.editor_bg,
                text: self.text_primary,
                primary: self.accent,
                success: Color::from_rgb8(0x10, 0x7C, 0x10),
                warning: Color::from_rgb8(0x9D, 0x5D, 0x00),
                danger: Color::from_rgb8(0xC4, 0x2B, 0x1C),
            },
        )
    }
}

pub fn titlebar_container(p: FluentPalette) -> impl Fn(&Theme) -> container::Style {
    move |_| container::Style {
        background: Some(Background::Color(p.titlebar_bg)),
        text_color: Some(p.text_primary),
        ..Default::default()
    }
}

pub fn surface_container(p: FluentPalette) -> impl Fn(&Theme) -> container::Style {
    move |_| container::Style {
        background: Some(Background::Color(p.surface_bg)),
        text_color: Some(p.text_primary),
        ..Default::default()
    }
}

pub fn editor_container(p: FluentPalette) -> impl Fn(&Theme) -> container::Style {
    move |_| container::Style {
        background: Some(Background::Color(p.editor_bg)),
        text_color: Some(p.text_primary),
        ..Default::default()
    }
}

pub fn statusbar_container(p: FluentPalette) -> impl Fn(&Theme) -> container::Style {
    move |_| container::Style {
        background: Some(Background::Color(p.statusbar_bg)),
        text_color: Some(p.text_secondary),
        ..Default::default()
    }
}

pub fn overlay_card(p: FluentPalette) -> impl Fn(&Theme) -> container::Style {
    move |_| container::Style {
        background: Some(Background::Color(p.overlay_bg)),
        text_color: Some(p.text_primary),
        border: Border {
            color: p.overlay_border,
            width: 1.0,
            radius: 8.0.into(),
        },
        shadow: Shadow {
            color: Color::from_rgba8(0, 0, 0, if p.is_dark { 0.42 } else { 0.14 }),
            offset: Vector::new(0.0, 6.0),
            blur_radius: 18.0,
        },
        ..Default::default()
    }
}

#[allow(dead_code)]
pub fn settings_card(p: FluentPalette) -> impl Fn(&Theme) -> container::Style {
    move |_| container::Style {
        background: Some(Background::Color(p.surface_bg)),
        text_color: Some(p.text_primary),
        border: Border {
            color: p.border,
            width: 1.0,
            radius: 6.0.into(),
        },
        ..Default::default()
    }
}

pub fn active_tab_container(p: FluentPalette) -> impl Fn(&Theme) -> container::Style {
    move |_| container::Style {
        background: Some(Background::Color(p.surface_bg)),
        text_color: Some(p.text_primary),
        border: Border {
            color: p.border,
            width: 0.0,
            radius: iced::border::Radius {
                top_left: 8.0,
                top_right: 8.0,
                bottom_right: 0.0,
                bottom_left: 0.0,
            },
        },
        ..Default::default()
    }
}

pub fn ghost_button(p: FluentPalette) -> impl Fn(&Theme, button::Status) -> button::Style {
    move |_, status| {
        let bg = match status {
            button::Status::Hovered => Some(Background::Color(p.hover_bg)),
            button::Status::Pressed => Some(Background::Color(p.pressed_bg)),
            _ => None,
        };
        button::Style {
            background: bg,
            text_color: p.text_primary,
            border: Border {
                radius: 4.0.into(),
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

pub fn menu_bar_button(
    p: FluentPalette,
    is_open: bool,
) -> impl Fn(&Theme, button::Status) -> button::Style {
    move |_, status| {
        let bg = if is_open {
            Some(Background::Color(p.pressed_bg))
        } else {
            match status {
                button::Status::Hovered => Some(Background::Color(p.hover_bg)),
                button::Status::Pressed => Some(Background::Color(p.pressed_bg)),
                _ => None,
            }
        };
        button::Style {
            background: bg,
            text_color: p.text_primary,
            border: Border {
                radius: 4.0.into(),
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

pub fn toggle_toolbar_button(
    p: FluentPalette,
    active: bool,
) -> impl Fn(&Theme, button::Status) -> button::Style {
    move |_, status| {
        let bg = if active {
            Some(Background::Color(p.active_toggle_bg))
        } else {
            match status {
                button::Status::Hovered => Some(Background::Color(p.hover_bg)),
                button::Status::Pressed => Some(Background::Color(p.pressed_bg)),
                _ => None,
            }
        };
        button::Style {
            background: bg,
            text_color: if active { p.accent } else { p.text_primary },
            border: Border {
                radius: 4.0.into(),
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

pub fn window_control_button(
    p: FluentPalette,
    is_close: bool,
) -> impl Fn(&Theme, button::Status) -> button::Style {
    move |_, status| {
        let (bg, text_color) = match status {
            button::Status::Hovered => {
                if is_close {
                    (Some(Background::Color(p.close_hover_bg)), Color::WHITE)
                } else {
                    (Some(Background::Color(p.hover_bg)), p.text_primary)
                }
            }
            button::Status::Pressed => {
                if is_close {
                    (
                        Some(Background::Color(Color::from_rgb8(0xB3, 0x26, 0x1E))),
                        Color::WHITE,
                    )
                } else {
                    (Some(Background::Color(p.pressed_bg)), p.text_primary)
                }
            }
            _ => (None, p.text_primary),
        };
        button::Style {
            background: bg,
            text_color,
            border: Border::default(),
            ..Default::default()
        }
    }
}

pub fn statusbar_button(p: FluentPalette) -> impl Fn(&Theme, button::Status) -> button::Style {
    move |_, status| {
        let bg = match status {
            button::Status::Hovered => Some(Background::Color(p.hover_bg)),
            button::Status::Pressed => Some(Background::Color(p.pressed_bg)),
            _ => None,
        };
        button::Style {
            background: bg,
            text_color: p.text_secondary,
            border: Border {
                radius: 3.0.into(),
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

pub fn tab_close_button(p: FluentPalette) -> impl Fn(&Theme, button::Status) -> button::Style {
    move |_, status| {
        let (bg, text_color) = match status {
            button::Status::Hovered => (Some(Background::Color(p.hover_bg)), p.text_primary),
            button::Status::Pressed => (Some(Background::Color(p.pressed_bg)), p.text_primary),
            _ => (None, p.text_secondary),
        };
        button::Style {
            background: bg,
            text_color,
            border: Border {
                radius: 4.0.into(),
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

pub fn accent_button(p: FluentPalette) -> impl Fn(&Theme, button::Status) -> button::Style {
    move |_, status| {
        let (bg, text_color) = match status {
            button::Status::Hovered => (
                if p.is_dark {
                    Color::from_rgb8(0xFF, 0xFF, 0xFF)
                } else {
                    Color::from_rgb8(0x05, 0x05, 0x05)
                },
                if p.is_dark {
                    Color::from_rgb8(0x10, 0x10, 0x10)
                } else {
                    Color::WHITE
                },
            ),
            button::Status::Pressed => (
                if p.is_dark {
                    Color::from_rgb8(0xCC, 0xCC, 0xCC)
                } else {
                    Color::from_rgb8(0x33, 0x33, 0x33)
                },
                if p.is_dark {
                    Color::from_rgb8(0x10, 0x10, 0x10)
                } else {
                    Color::WHITE
                },
            ),
            _ => (
                p.accent,
                if p.is_dark {
                    Color::from_rgb8(0x10, 0x10, 0x10)
                } else {
                    Color::WHITE
                },
            ),
        };
        button::Style {
            background: Some(Background::Color(bg)),
            text_color,
            border: Border {
                radius: 4.0.into(),
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

pub fn fluent_toggler(p: FluentPalette) -> impl Fn(&Theme, toggler::Status) -> toggler::Style {
    move |_, status| {
        let is_toggled = match status {
            toggler::Status::Active { is_toggled } => is_toggled,
            toggler::Status::Hovered { is_toggled } => is_toggled,
            toggler::Status::Disabled { is_toggled } => is_toggled,
        };
        let is_hovered = matches!(status, toggler::Status::Hovered { .. });

        if is_toggled {
            let pill_color = if is_hovered {
                if p.is_dark {
                    Color::from_rgb8(0xFF, 0xFF, 0xFF)
                } else {
                    Color::from_rgb8(0x05, 0x05, 0x05)
                }
            } else {
                p.accent
            };
            let knob_color = if p.is_dark {
                Color::from_rgb8(0x10, 0x10, 0x10)
            } else {
                Color::from_rgb8(0xFF, 0xFF, 0xFF)
            };

            toggler::Style {
                background: Background::Color(pill_color),
                background_border_width: 0.0,
                background_border_color: Color::TRANSPARENT,
                foreground: Background::Color(knob_color),
                foreground_border_width: 0.0,
                foreground_border_color: Color::TRANSPARENT,
                text_color: Some(p.text_primary),
                border_radius: None,
                padding_ratio: 0.08,
            }
        } else {
            let border_color = if is_hovered {
                p.text_secondary
            } else {
                p.border
            };
            let knob_color = if is_hovered {
                p.text_primary
            } else {
                p.text_secondary
            };

            toggler::Style {
                background: Background::Color(Color::TRANSPARENT),
                background_border_width: 1.0,
                background_border_color: border_color,
                foreground: Background::Color(knob_color),
                foreground_border_width: 0.0,
                foreground_border_color: Color::TRANSPARENT,
                text_color: Some(p.text_primary),
                border_radius: None,
                padding_ratio: 0.08,
            }
        }
    }
}

pub fn fluent_pick_list(
    p: FluentPalette,
) -> impl Fn(&Theme, pick_list::Status) -> pick_list::Style {
    move |_, status| {
        let border_color = match status {
            pick_list::Status::Hovered => p.overlay_border,
            pick_list::Status::Opened { .. } => p.accent,
            pick_list::Status::Active => p.border,
        };

        pick_list::Style {
            text_color: p.text_primary,
            placeholder_color: p.text_muted,
            handle_color: p.text_secondary,
            background: Background::Color(p.input_bg),
            border: Border {
                color: border_color,
                width: 1.0,
                radius: 4.0.into(),
            },
        }
    }
}

pub fn notepad_editor_style(
    p: FluentPalette,
) -> impl Fn(&Theme, text_editor::Status) -> text_editor::Style {
    move |_, _| text_editor::Style {
        background: Background::Color(p.editor_bg),
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: 0.0.into(),
        },
        placeholder: p.text_muted,
        value: p.text_primary,
        selection: p.selection,
    }
}

pub fn fluent_input_style(
    p: FluentPalette,
) -> impl Fn(&Theme, text_input::Status) -> text_input::Style {
    move |_, status| {
        let border_color = match status {
            text_input::Status::Focused { .. } => p.accent,
            text_input::Status::Hovered => p.overlay_border,
            _ => p.border,
        };
        text_input::Style {
            background: Background::Color(p.input_bg),
            border: Border {
                color: border_color,
                width: 1.0,
                radius: 4.0.into(),
            },
            icon: p.text_secondary,
            placeholder: p.text_muted,
            value: p.text_primary,
            selection: p.selection,
        }
    }
}

pub fn hairline_rule(p: FluentPalette) -> impl Fn(&Theme) -> rule::Style {
    move |_| rule::Style {
        color: p.border,
        radius: 0.0.into(),
        fill_mode: rule::FillMode::Full,
        snap: true,
    }
}
