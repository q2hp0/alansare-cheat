#![allow(unused)]
use egui::Color32;
use serde::{Deserialize, Serialize};

pub struct Colors;

impl Colors {
    pub const BACKDROP: Color32 = Color32::from_rgb(15, 15, 20);
    pub const BASE: Color32 = Color32::from_rgb(22, 22, 30);
    pub const HIGHLIGHT: Color32 = Color32::from_rgb(40, 40, 55);
    pub const SUBTEXT: Color32 = Color32::from_rgb(160, 160, 175);
    pub const TEXT: Color32 = Color32::from_rgb(245, 245, 250);
    pub const RED: Color32 = Color32::from_rgb(255, 75, 110);
    pub const ORANGE: Color32 = Color32::from_rgb(255, 140, 60);
    pub const YELLOW: Color32 = Color32::from_rgb(255, 210, 80);
    pub const GREEN: Color32 = Color32::from_rgb(50, 240, 140);
    pub const TEAL: Color32 = Color32::from_rgb(0, 220, 220);
    pub const BLUE: Color32 = Color32::from_rgb(80, 160, 255);
    pub const PURPLE: Color32 = Color32::from_rgb(190, 90, 255);

    pub const ACCENT_COLORS: [(&str, Color32); 7] = [
        ("Red", Self::RED),
        ("Orange", Self::ORANGE),
        ("Yellow", Self::YELLOW),
        ("Green", Self::GREEN),
        ("Teal", Self::TEAL),
        ("Blue", Self::BLUE),
        ("Purple", Self::PURPLE),
    ];
}
