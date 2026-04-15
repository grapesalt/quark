use crate::anim::Animated;
use std::cell::Cell;
use std::rc::Rc;

use kurbo::{Affine, Shape, Stroke};
use vello::peniko::{self, Fill};

#[derive(Clone, Debug, PartialEq)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}

impl Color {
    pub const WHITE: Color = Color {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    };
    pub const BLACK: Color = Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };
    pub const RED: Color = Color {
        r: 1.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };
    pub const GREEN: Color = Color {
        r: 0.0,
        g: 1.0,
        b: 0.0,
        a: 1.0,
    };
    pub const BLUE: Color = Color {
        r: 0.0,
        g: 0.0,
        b: 1.0,
        a: 1.0,
    };
    pub const TRANSPARENT: Color = Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 0.0,
    };

    pub fn rgba(r: f64, g: f64, b: f64, a: f64) -> Self {
        Color { r, g, b, a }
    }

    pub fn hex(hex: u32) -> Self {
        Color {
            r: ((hex >> 24) & 0xFF) as f64 / 255.0,
            g: ((hex >> 16) & 0xFF) as f64 / 255.0,
            b: ((hex >> 8) & 0xFF) as f64 / 255.0,
            a: (hex & 0xFF) as f64 / 255.0,
        }
    }

    pub fn hsv(h: f64, s: f64, v: f64) -> Self {
        let i = (h * 6.0).floor() as u32;
        let f = h * 6.0 - i as f64;
        let p = v * (1.0 - s);
        let q = v * (1.0 - f * s);
        let t = v * (1.0 - (1.0 - f) * s);
        let (r, g, b) = match i % 6 {
            0 => (v, t, p),
            1 => (q, v, p),
            2 => (p, v, t),
            3 => (p, q, v),
            4 => (t, p, v),
            _ => (v, p, q),
        };
        Color { r, g, b, a: 1.0 }
    }

    pub fn lerp(&self, other: &Color, t: f64) -> Color {
        Color {
            r: self.r + (other.r - self.r) * t,
            g: self.g + (other.g - self.g) * t,
            b: self.b + (other.b - self.b) * t,
            a: self.a + (other.a - self.a) * t,
        }
    }

    pub(crate) fn to_vello_color(&self) -> peniko::Color {
        peniko::Color::new([self.r as f32, self.g as f32, self.b as f32, self.a as f32])
    }
}

pub struct Style {
    pub fill: Animated<Color>,
    pub stroke: Animated<Color>,
    pub stroke_width: Animated<f64>,
}

impl Style {
    pub(crate) fn new(cursor: Rc<Cell<f64>>) -> Self {
        Style {
            fill: Animated::new(Color::WHITE, Rc::clone(&cursor)),
            stroke: Animated::new(Color::BLACK, Rc::clone(&cursor)),
            stroke_width: Animated::new(2.0, cursor),
        }
    }

    pub(crate) fn apply_fill(
        &self,
        t: f64,
        scene: &mut vello::Scene,
        shape: &impl Shape,
        fill_rule: Fill,
    ) {
        let fill = self.fill.value_at(t).to_vello_color();
        scene.fill(fill_rule, Affine::IDENTITY, fill, None, shape);
    }

    pub(crate) fn apply_stroke(&self, t: f64, scene: &mut vello::Scene, shape: &impl Shape) {
        let stroke = self.stroke.value_at(t).to_vello_color();
        let stroke_width = self.stroke_width.value_at(t);

        if stroke_width > 0.0 {
            scene.stroke(
                &Stroke::new(stroke_width),
                Affine::IDENTITY,
                stroke,
                None,
                shape,
            );
        }
    }
}
