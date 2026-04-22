use quartz::{Color, Image};
use quartz::ShapeType;
use crate::preferences::*;

pub const DIV_W:    f32 = 2.0;
pub const HIT_HALF: f32 = 5.0;

pub const MIN_EXPLORER: f32 = 120.0;
pub const MIN_EDITOR:   f32 = 200.0;
pub const MIN_TERMINAL: f32 = 160.0;

pub const ICON_SIZE: f32 = 30.0;
pub const ICON_PAD:  f32 = 10.0;
pub const ICON_GAP:  f32 = 10.0;

pub const LOGO_H:   f32 = 80.0;
pub const LOGO_W:   f32 = 80.0;
pub const LOGO_PAD: f32 = 12.0;

pub struct Panels {
    pub explorer: (f32, f32, f32, f32),
    pub editor:   (f32, f32, f32, f32),
    pub terminal: (f32, f32, f32, f32),
}

pub fn divider_image_v(h: f32) -> Image {
    use image::RgbaImage;
    let mut img = RgbaImage::new(DIV_W as u32, 1);
    img.pixels_mut().for_each(|p| *p = image::Rgba([40, 40, 40, 255]));
    Image {
        shape: ShapeType::Rectangle(0.0, (DIV_W, h), 0.0),
        image: img.into(),
        color: None,
    }
}

pub fn divider_image_h(w: f32) -> Image {
    use image::RgbaImage;
    let mut img = RgbaImage::new(1, DIV_W as u32);
    img.pixels_mut().for_each(|p| *p = image::Rgba([40, 40, 40, 255]));
    Image {
        shape: ShapeType::Rectangle(0.0, (w, DIV_W), 0.0),
        image: img.into(),
        color: None,
    }
}

pub fn icon_rects(cw: f32) -> [(f32, f32); 2] {
    let y  = (TOPBAR_H - ICON_SIZE) * 0.5;
    let x1 = cw - ICON_PAD - ICON_SIZE;
    let x0 = x1 - ICON_GAP - ICON_SIZE;
    [(x0, y), (x1, y)]
}

pub fn hit(mx: f32, my: f32, x: f32, y: f32, w: f32, h: f32) -> bool {
    mx >= x && mx <= x + w && my >= y && my <= y + h
}