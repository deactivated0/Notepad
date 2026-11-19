use iced::widget::{image, svg, Image, Svg};
use iced::{Color, Theme};

const NOTEPAD_ICON_PNG: &[u8] = include_bytes!("../assets/icon_128.png");

const SETTINGS_GEAR_SVG: &[u8] = br##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.65" stroke-linecap="round" stroke-linejoin="round">
  <circle cx="12" cy="12" r="3.2"/>
  <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 1 1-2.83 2.83l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 1 1-4 0v-.09a1.65 1.65 0 0 0-1-1.51 1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 1 1-2.83-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 1 1 0-4h.09a1.65 1.65 0 0 0 1.51-1 1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 1 1 2.83-2.83l.06.06a1.65 1.65 0 0 0 1.82.33h.08a1.65 1.65 0 0 0 1-1.51V3a2 2 0 1 1 4 0v.09a1.65 1.65 0 0 0 1 1.51h.08a1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 1 1 2.83 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82v.08a1.65 1.65 0 0 0 1.51 1H21a2 2 0 1 1 0 4h-.09a1.65 1.65 0 0 0-1.51 1z"/>
</svg>"##;

const PLUS_SVG: &[u8] = br##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.4" stroke-linecap="round">
  <line x1="8" y1="3" x2="8" y2="13"/>
  <line x1="3" y1="8" x2="13" y2="8"/>
</svg>"##;

const CLOSE_SVG: &[u8] = br##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 12 12" fill="none" stroke="currentColor" stroke-width="1.25" stroke-linecap="round">
  <line x1="2" y1="2" x2="10" y2="10"/>
  <line x1="10" y1="2" x2="2" y2="10"/>
</svg>"##;

const MINIMIZE_SVG: &[u8] = br##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 12 12">
  <rect x="1" y="5.3" width="10" height="1.4" rx="0.3" fill="currentColor"/>
</svg>"##;

const MAXIMIZE_SVG: &[u8] = br##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 12 12" fill="none" stroke="currentColor" stroke-width="1.2">
  <rect x="1.5" y="1.5" width="9" height="9" rx="0.5"/>
</svg>"##;

const LIST_SVG: &[u8] = br##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.4" stroke-linecap="round">
  <circle cx="3" cy="4.5" r="0.8" fill="currentColor"/>
  <line x1="6" y1="4.5" x2="13.5" y2="4.5"/>
  <circle cx="3" cy="8" r="0.8" fill="currentColor"/>
  <line x1="6" y1="8" x2="13.5" y2="8"/>
  <circle cx="3" cy="11.5" r="0.8" fill="currentColor"/>
  <line x1="6" y1="11.5" x2="13.5" y2="11.5"/>
</svg>"##;

const LINK_SVG: &[u8] = br##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.4" stroke-linecap="round" stroke-linejoin="round">
  <path d="M6.5 9.5a3.2 3.2 0 0 0 4.5 0l1.8-1.8a3.2 3.2 0 0 0-4.5-4.5L7.3 4.2"/>
  <path d="M9.5 6.5a3.2 3.2 0 0 0-4.5 0L3.2 8.3a3.2 3.2 0 0 0 4.5 4.5l1-1"/>
</svg>"##;

#[allow(dead_code)]
const BACK_ARROW_SVG: &[u8] = br##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
  <line x1="13" y1="8" x2="3.5" y2="8"/>
  <polyline points="7.5 4 3.5 8 7.5 12"/>
</svg>"##;

pub fn notepad_logo<'a>(size: f32) -> Image<image::Handle> {
    image(image::Handle::from_bytes(NOTEPAD_ICON_PNG))
        .width(size)
        .height(size)
}

fn tinted_icon<'a>(bytes: &'static [u8], size: f32, color: Color) -> Svg<'a, Theme> {
    svg(svg::Handle::from_memory(bytes))
        .width(size)
        .height(size)
        .style(move |_theme, _status| svg::Style { color: Some(color) })
}

pub fn settings_gear<'a>(size: f32, color: Color) -> Svg<'a, Theme> {
    tinted_icon(SETTINGS_GEAR_SVG, size, color)
}

pub fn plus<'a>(size: f32, color: Color) -> Svg<'a, Theme> {
    tinted_icon(PLUS_SVG, size, color)
}

pub fn close<'a>(size: f32, color: Color) -> Svg<'a, Theme> {
    tinted_icon(CLOSE_SVG, size, color)
}

pub fn minimize<'a>(size: f32, color: Color) -> Svg<'a, Theme> {
    tinted_icon(MINIMIZE_SVG, size, color)
}

pub fn maximize<'a>(size: f32, color: Color) -> Svg<'a, Theme> {
    tinted_icon(MAXIMIZE_SVG, size, color)
}

pub fn list<'a>(size: f32, color: Color) -> Svg<'a, Theme> {
    tinted_icon(LIST_SVG, size, color)
}

pub fn link<'a>(size: f32, color: Color) -> Svg<'a, Theme> {
    tinted_icon(LINK_SVG, size, color)
}

#[allow(dead_code)]
pub fn back_arrow<'a>(size: f32, color: Color) -> Svg<'a, Theme> {
    tinted_icon(BACK_ARROW_SVG, size, color)
}
