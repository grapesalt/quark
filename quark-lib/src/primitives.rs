use crate::anim::Animated;
use crate::style::{Font, Style};

use kurbo::{Affine, BezPath, Point, Shape, Vec2};
use skrifa::{raw::FileRef, MetadataProvider};
use std::cell::Cell;
use std::f64::consts::{FRAC_PI_2, PI, TAU};
use std::rc::Rc;
use vello::peniko::{Brush, Fill};

pub(crate) trait Object {
    fn draw_at(&self, t: f64, scene: &mut vello::Scene);
}

pub struct Circle {
    pub center: Animated<(f64, f64)>,
    pub radius: Animated<f64>,
    pub style: Style,

    cursor: Rc<Cell<f64>>,
    visible_from: f64,
    hidden_at: f64,
}

impl Circle {
    pub(crate) fn new(center: (f64, f64), radius: f64, cursor: Rc<Cell<f64>>) -> Self {
        Circle {
            center: Animated::new(center, Rc::clone(&cursor)),
            radius: Animated::new(radius, Rc::clone(&cursor)),
            style: Style::new(Rc::clone(&cursor)),
            visible_from: cursor.get(),
            hidden_at: f64::INFINITY,
            cursor,
        }
    }

    pub fn hide(&mut self) {
        self.hidden_at = self.cursor.get();
    }

    pub fn show(&mut self) {
        self.visible_from = self.cursor.get();
        self.hidden_at = f64::INFINITY;
    }
}

impl Object for Circle {
    fn draw_at(&self, t: f64, scene: &mut vello::Scene) {
        if t < self.visible_from || self.hidden_at <= t {
            return;
        }

        let (cx, cy) = self.center.value_at(t);
        let r = self.radius.value_at(t);
        let shape = kurbo::Circle::new(Point::new(cx, cy), r);

        self.style.apply_fill(t, scene, &shape, Fill::NonZero);
        self.style.apply_stroke(t, scene, &shape);
    }
}

pub struct Rect {
    pub corner: Animated<(f64, f64)>,
    pub width: Animated<f64>,
    pub height: Animated<f64>,
    pub style: Style,

    cursor: Rc<Cell<f64>>,
    visible_from: f64,
    hidden_at: f64,
}

impl Rect {
    pub(crate) fn new(corner: (f64, f64), width: f64, height: f64, cursor: Rc<Cell<f64>>) -> Self {
        Rect {
            corner: Animated::new(corner, Rc::clone(&cursor)),
            width: Animated::new(width, Rc::clone(&cursor)),
            height: Animated::new(height, Rc::clone(&cursor)),
            style: Style::new(Rc::clone(&cursor)),
            visible_from: cursor.get(),
            hidden_at: f64::INFINITY,
            cursor,
        }
    }

    pub fn hide(&mut self) {
        self.hidden_at = self.cursor.get();
    }

    pub fn show(&mut self) {
        self.visible_from = self.cursor.get();
        self.hidden_at = f64::INFINITY;
    }
}

impl Object for Rect {
    fn draw_at(&self, t: f64, scene: &mut vello::Scene) {
        if t < self.visible_from || self.hidden_at <= t {
            return;
        }

        let (x, y) = self.corner.value_at(t);
        let w = self.width.value_at(t);
        let h = self.height.value_at(t);

        let shape = kurbo::Rect::from_origin_size(Point::new(x, y), (w, h));
        self.style.apply_fill(t, scene, &shape, Fill::NonZero);
        self.style.apply_stroke(t, scene, &shape);
    }
}

pub struct RoundedRect {
    pub corner: Animated<(f64, f64)>,
    pub width: Animated<f64>,
    pub height: Animated<f64>,
    pub radius: Animated<f64>,
    pub style: Style,

    cursor: Rc<Cell<f64>>,
    visible_from: f64,
    hidden_at: f64,
}

impl RoundedRect {
    pub(crate) fn new(
        corner: (f64, f64),
        width: f64,
        height: f64,
        radius: f64,
        cursor: Rc<Cell<f64>>,
    ) -> Self {
        RoundedRect {
            corner: Animated::new(corner, Rc::clone(&cursor)),
            width: Animated::new(width, Rc::clone(&cursor)),
            height: Animated::new(height, Rc::clone(&cursor)),
            radius: Animated::new(radius, Rc::clone(&cursor)),
            style: Style::new(Rc::clone(&cursor)),
            visible_from: cursor.get(),
            hidden_at: f64::INFINITY,
            cursor,
        }
    }

    pub fn hide(&mut self) {
        self.hidden_at = self.cursor.get();
    }

    pub fn show(&mut self) {
        self.visible_from = self.cursor.get();
        self.hidden_at = f64::INFINITY;
    }
}

impl Object for RoundedRect {
    fn draw_at(&self, t: f64, scene: &mut vello::Scene) {
        if t < self.visible_from || self.hidden_at <= t {
            return;
        }

        let (x, y) = self.corner.value_at(t);
        let w = self.width.value_at(t);
        let h = self.height.value_at(t);
        let r = self.radius.value_at(t);

        let shape = kurbo::RoundedRect::from_origin_size(Point::new(x, y), (w, h), r);
        self.style.apply_fill(t, scene, &shape, Fill::NonZero);
        self.style.apply_stroke(t, scene, &shape);
    }
}

pub struct Ellipse {
    pub center: Animated<(f64, f64)>,
    pub a: Animated<f64>,
    pub b: Animated<f64>,
    pub style: Style,

    cursor: Rc<Cell<f64>>,
    visible_from: f64,
    hidden_at: f64,
}

impl Ellipse {
    pub(crate) fn new(center: (f64, f64), a: f64, b: f64, cursor: Rc<Cell<f64>>) -> Self {
        Ellipse {
            center: Animated::new(center, Rc::clone(&cursor)),
            a: Animated::new(a, Rc::clone(&cursor)),
            b: Animated::new(b, Rc::clone(&cursor)),
            style: Style::new(Rc::clone(&cursor)),
            visible_from: cursor.get(),
            hidden_at: f64::INFINITY,
            cursor,
        }
    }

    pub fn hide(&mut self) {
        self.hidden_at = self.cursor.get();
    }

    pub fn show(&mut self) {
        self.visible_from = self.cursor.get();
        self.hidden_at = f64::INFINITY;
    }
}

impl Object for Ellipse {
    fn draw_at(&self, t: f64, scene: &mut vello::Scene) {
        if t < self.visible_from || self.hidden_at <= t {
            return;
        }

        let (cx, cy) = self.center.value_at(t);
        let a = self.a.value_at(t);
        let b = self.b.value_at(t);

        let shape = kurbo::Ellipse::new(Point::new(cx, cy), Vec2::new(a, b), 0.0);
        self.style.apply_fill(t, scene, &shape, Fill::NonZero);
        self.style.apply_stroke(t, scene, &shape);
    }
}

pub struct Arc {
    pub center: Animated<(f64, f64)>,
    pub radius: Animated<f64>,
    pub start_angle: Animated<f64>,
    pub end_angle: Animated<f64>,
    pub style: Style,

    cursor: Rc<Cell<f64>>,
    visible_from: f64,
    hidden_at: f64,
}

impl Arc {
    pub(crate) fn new(
        center: (f64, f64),
        radius: f64,
        start_angle: f64,
        end_angle: f64,
        cursor: Rc<Cell<f64>>,
    ) -> Self {
        Arc {
            center: Animated::new(center, Rc::clone(&cursor)),
            radius: Animated::new(radius, Rc::clone(&cursor)),
            start_angle: Animated::new(start_angle, Rc::clone(&cursor)),
            end_angle: Animated::new(end_angle, Rc::clone(&cursor)),
            style: Style::new(Rc::clone(&cursor)),
            visible_from: cursor.get(),
            hidden_at: f64::INFINITY,
            cursor,
        }
    }

    pub fn hide(&mut self) {
        self.hidden_at = self.cursor.get();
    }

    pub fn show(&mut self) {
        self.visible_from = self.cursor.get();
        self.hidden_at = f64::INFINITY;
    }
}

impl Object for Arc {
    fn draw_at(&self, t: f64, scene: &mut vello::Scene) {
        if t < self.visible_from || self.hidden_at <= t {
            return;
        }

        let (cx, cy) = self.center.value_at(t);
        let r = self.radius.value_at(t);
        let start = self.start_angle.value_at(t);
        let end = self.end_angle.value_at(t);

        let shape = kurbo::Arc::new(Point::new(cx, cy), Vec2::new(r, r), start, end - start, 0.0);
        self.style.apply_stroke(t, scene, &shape);
    }
}

pub struct Line {
    pub point1: Animated<(f64, f64)>,
    pub point2: Animated<(f64, f64)>,
    pub style: Style,

    cursor: Rc<Cell<f64>>,
    visible_from: f64,
    hidden_at: f64,
}

impl Line {
    pub(crate) fn new(point1: (f64, f64), point2: (f64, f64), cursor: Rc<Cell<f64>>) -> Self {
        Line {
            point1: Animated::new(point1, Rc::clone(&cursor)),
            point2: Animated::new(point2, Rc::clone(&cursor)),
            style: Style::new(Rc::clone(&cursor)),
            visible_from: cursor.get(),
            hidden_at: f64::INFINITY,
            cursor,
        }
    }

    pub fn hide(&mut self) {
        self.hidden_at = self.cursor.get();
    }

    pub fn show(&mut self) {
        self.visible_from = self.cursor.get();
        self.hidden_at = f64::INFINITY;
    }
}

impl Object for Line {
    fn draw_at(&self, t: f64, scene: &mut vello::Scene) {
        if t < self.visible_from || self.hidden_at <= t {
            return;
        }

        let (x1, y1) = self.point1.value_at(t);
        let (x2, y2) = self.point2.value_at(t);

        let shape = kurbo::Line::new(Point::new(x1, y1), Point::new(x2, y2));
        self.style.apply_stroke(t, scene, &shape);
    }
}

pub struct Ring {
    pub center: Animated<(f64, f64)>,
    pub inner_radius: Animated<f64>,
    pub outer_radius: Animated<f64>,
    pub style: Style,

    cursor: Rc<Cell<f64>>,
    visible_from: f64,
    hidden_at: f64,
}

impl Ring {
    pub(crate) fn new(
        center: (f64, f64),
        inner_radius: f64,
        outer_radius: f64,
        cursor: Rc<Cell<f64>>,
    ) -> Self {
        Ring {
            center: Animated::new(center, Rc::clone(&cursor)),
            inner_radius: Animated::new(inner_radius, Rc::clone(&cursor)),
            outer_radius: Animated::new(outer_radius, Rc::clone(&cursor)),
            style: Style::new(Rc::clone(&cursor)),
            visible_from: cursor.get(),
            hidden_at: f64::INFINITY,
            cursor,
        }
    }

    pub fn hide(&mut self) {
        self.hidden_at = self.cursor.get();
    }

    pub fn show(&mut self) {
        self.visible_from = self.cursor.get();
        self.hidden_at = f64::INFINITY;
    }
}

impl Object for Ring {
    fn draw_at(&self, t: f64, scene: &mut vello::Scene) {
        if t < self.visible_from || self.hidden_at <= t {
            return;
        }

        let (cx, cy) = self.center.value_at(t);
        let inner = self.inner_radius.value_at(t);
        let outer = self.outer_radius.value_at(t);
        let center = Point::new(cx, cy);

        let mut path = BezPath::new();
        path.extend(kurbo::Circle::new(center, outer).path_elements(0.1));
        path.extend(kurbo::Circle::new(center, inner).path_elements(0.1));

        self.style.apply_fill(t, scene, &path, Fill::EvenOdd);
        self.style.apply_stroke(t, scene, &path);
    }
}

pub struct Capsule {
    pub center1: Animated<(f64, f64)>,
    pub center2: Animated<(f64, f64)>,
    pub radius: Animated<f64>,
    pub style: Style,

    cursor: Rc<Cell<f64>>,
    visible_from: f64,
    hidden_at: f64,
}

impl Capsule {
    pub(crate) fn new(
        center1: (f64, f64),
        center2: (f64, f64),
        radius: f64,
        cursor: Rc<Cell<f64>>,
    ) -> Self {
        Capsule {
            center1: Animated::new(center1, Rc::clone(&cursor)),
            center2: Animated::new(center2, Rc::clone(&cursor)),
            radius: Animated::new(radius, Rc::clone(&cursor)),
            style: Style::new(Rc::clone(&cursor)),
            visible_from: cursor.get(),
            hidden_at: f64::INFINITY,
            cursor,
        }
    }

    pub fn hide(&mut self) {
        self.hidden_at = self.cursor.get();
    }

    pub fn show(&mut self) {
        self.visible_from = self.cursor.get();
        self.hidden_at = f64::INFINITY;
    }
}

impl Object for Capsule {
    fn draw_at(&self, t: f64, scene: &mut vello::Scene) {
        if t < self.visible_from || self.hidden_at <= t {
            return;
        }

        let (cx1, cy1) = self.center1.value_at(t);
        let (cx2, cy2) = self.center2.value_at(t);
        let r = self.radius.value_at(t);
        let angle = (cy2 - cy1).atan2(cx2 - cx1);
        let perp = angle + FRAC_PI_2;
        let (px, py) = (r * perp.cos(), r * perp.sin());

        let mut path = BezPath::new();

        path.move_to(Point::new(cx1 + px, cy1 + py));
        path.line_to(Point::new(cx2 + px, cy2 + py));

        let arc2 = kurbo::Arc::new(Point::new(cx2, cy2), Vec2::new(r, r), perp, -PI, 0.0);
        let mut arc2_els = arc2.path_elements(0.1);
        arc2_els.next();
        path.extend(arc2_els);

        path.line_to(Point::new(cx1 - px, cy1 - py));

        let arc1 = kurbo::Arc::new(Point::new(cx1, cy1), Vec2::new(r, r), perp + PI, -PI, 0.0);
        let mut arc1_els = arc1.path_elements(0.1);
        arc1_els.next();
        path.extend(arc1_els);

        path.close_path();
        self.style.apply_fill(t, scene, &path, Fill::NonZero);
        self.style.apply_stroke(t, scene, &path);
    }
}

pub struct RegularPolygon {
    pub center: Animated<(f64, f64)>,
    pub radius: Animated<f64>,
    pub sides: u32,
    pub style: Style,

    cursor: Rc<Cell<f64>>,
    visible_from: f64,
    hidden_at: f64,
}

impl RegularPolygon {
    pub(crate) fn new(center: (f64, f64), radius: f64, sides: u32, cursor: Rc<Cell<f64>>) -> Self {
        RegularPolygon {
            center: Animated::new(center, Rc::clone(&cursor)),
            radius: Animated::new(radius, Rc::clone(&cursor)),
            sides,
            style: Style::new(Rc::clone(&cursor)),
            visible_from: cursor.get(),
            hidden_at: f64::INFINITY,
            cursor,
        }
    }

    pub fn hide(&mut self) {
        self.hidden_at = self.cursor.get();
    }

    pub fn show(&mut self) {
        self.visible_from = self.cursor.get();
        self.hidden_at = f64::INFINITY;
    }
}

impl Object for RegularPolygon {
    fn draw_at(&self, t: f64, scene: &mut vello::Scene) {
        if t < self.visible_from || self.hidden_at <= t {
            return;
        }

        let (cx, cy) = self.center.value_at(t);
        let r = self.radius.value_at(t);
        let n = self.sides;
        let mut path = BezPath::new();

        for i in 0..n {
            let angle = TAU * (i as f64) / (n as f64) - FRAC_PI_2;
            let p = Point::new(cx + r * angle.cos(), cy + r * angle.sin());
            if i == 0 {
                path.move_to(p);
            } else {
                path.line_to(p);
            }
        }

        path.close_path();
        self.style.apply_fill(t, scene, &path, Fill::NonZero);
        self.style.apply_stroke(t, scene, &path);
    }
}

pub struct Text {
    pub position: Animated<(f64, f64)>,
    pub color: Animated<crate::Color>,
    pub font_size: Animated<f64>,
    pub style: Style,
    pub content: String,

    pub(crate) font_data: vello::peniko::FontData,

    cursor: Rc<Cell<f64>>,
    visible_from: f64,
    hidden_at: f64,
}

impl Text {
    pub(crate) fn new(
        content: &str,
        position: (f64, f64),
        font: Font,
        cursor: Rc<Cell<f64>>,
    ) -> Self {
        let mut style = Style::new(Rc::clone(&cursor));

        style.fill.set_immediate(crate::Color::TRANSPARENT);
        style.stroke_width.set_immediate(0.0);

        Text {
            content: content.into(),
            position: Animated::new(position, Rc::clone(&cursor)),
            color: Animated::new(crate::Color::WHITE, Rc::clone(&cursor)),
            font_size: Animated::new(font.initial_size, Rc::clone(&cursor)),
            style,
            font_data: font.data,
            visible_from: cursor.get(),
            hidden_at: f64::INFINITY,
            cursor,
        }
    }

    pub fn hide(&mut self) {
        self.hidden_at = self.cursor.get();
    }

    pub fn show(&mut self) {
        self.visible_from = self.cursor.get();
        self.hidden_at = f64::INFINITY;
    }
}

impl Object for Text {
    fn draw_at(&self, t: f64, scene: &mut vello::Scene) {
        if t < self.visible_from || self.hidden_at <= t {
            return;
        }

        let font_ref = match FileRef::new(self.font_data.data.as_ref()).ok() {
            Some(FileRef::Font(f)) => f,
            Some(FileRef::Collection(c)) => match c.get(self.font_data.index).ok() {
                Some(f) => f,
                None => return,
            },
            None => return,
        };

        let size = self.font_size.value_at(t) as f32;
        let color = self.color.value_at(t).to_vello_color();
        let (x, y) = self.position.value_at(t);

        let font_size = skrifa::instance::Size::new(size);

        let var_loc = font_ref.axes().location(std::iter::empty::<(&str, f32)>());
        let charmap = font_ref.charmap();
        let metrics = font_ref.metrics(font_size, &var_loc);

        let line_height = metrics.ascent - metrics.descent + metrics.leading;
        let glyph_metrics = font_ref.glyph_metrics(font_size, &var_loc);

        let mut line_width = 0f32;
        let mut max_width = 0f32;

        let mut num_newlines = 0u32;

        for ch in self.content.chars() {
            if ch == '\n' {
                max_width = max_width.max(line_width);
                line_width = 0.0;
                num_newlines += 1;
            } else {
                let gid = charmap.map(ch).unwrap_or_default();
                line_width += glyph_metrics.advance_width(gid).unwrap_or_default();
            }
        }

        max_width = max_width.max(line_width);
        let total_height = metrics.ascent - metrics.descent + line_height * num_newlines as f32;

        let rect = kurbo::Rect::new(
            x,
            y - metrics.ascent as f64,
            x + max_width as f64,
            y - metrics.ascent as f64 + total_height as f64,
        );

        self.style.apply_fill(t, scene, &rect, Fill::NonZero);
        self.style.apply_stroke(t, scene, &rect);

        let mut pen_x = 0f32;
        let mut pen_y = 0f32;

        scene
            .draw_glyphs(&self.font_data)
            .font_size(size)
            .transform(Affine::translate((x, y)))
            .brush(Brush::Solid(color))
            .draw(
                Fill::NonZero,
                self.content.chars().filter_map(|ch| {
                    if ch == '\n' {
                        pen_y += line_height;
                        pen_x = 0.0;
                        return None;
                    }

                    let gid = charmap.map(ch).unwrap_or_default();
                    let advance = glyph_metrics.advance_width(gid).unwrap_or_default();
                    let x = pen_x;
                    pen_x += advance;

                    Some(vello::Glyph {
                        id: gid.to_u32(),
                        x,
                        y: pen_y,
                    })
                }),
            );
    }
}
