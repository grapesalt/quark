use anyhow::Result;
use parley::fontique;
use primitives::Object;
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use vello;

pub(crate) mod anim;
pub(crate) mod primitives;
pub(crate) mod render;
pub(crate) mod style;

pub use anim::Easing;
pub use primitives::PathSegment;
pub use style::{Color, Font, FontWeight};

pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub fps: u32,
    pub background: Color,

    cursor: Rc<Cell<f64>>,
    objects: Vec<Rc<RefCell<dyn Object>>>,
    font_collection: fontique::Collection,
    font_source_cache: fontique::SourceCache,
}

impl Scene {
    pub fn new() -> Self {
        let mut font_collection = fontique::Collection::default();
        font_collection.load_system_fonts();

        Scene {
            width: 800,
            height: 600,
            fps: 60,
            background: Color::WHITE,
            cursor: Rc::new(Cell::new(0.0)),
            objects: Vec::new(),
            font_collection,
            font_source_cache: fontique::SourceCache::default(),
        }
    }

    pub fn circle(&mut self, center: (f64, f64), radius: f64) -> Rc<RefCell<primitives::Circle>> {
        let o = Rc::new(RefCell::new(primitives::Circle::new(
            center,
            radius,
            Rc::clone(&self.cursor),
        )));

        self.objects.push(Rc::clone(&o) as Rc<RefCell<dyn Object>>);
        o
    }

    pub fn rect(
        &mut self,
        corner: (f64, f64),
        width: f64,
        height: f64,
    ) -> Rc<RefCell<primitives::Rect>> {
        let o = Rc::new(RefCell::new(primitives::Rect::new(
            corner,
            width,
            height,
            Rc::clone(&self.cursor),
        )));

        self.objects.push(Rc::clone(&o) as Rc<RefCell<dyn Object>>);
        o
    }

    pub fn rounded_rect(
        &mut self,
        corner: (f64, f64),
        width: f64,
        height: f64,
        radius: f64,
    ) -> Rc<RefCell<primitives::RoundedRect>> {
        let o = Rc::new(RefCell::new(primitives::RoundedRect::new(
            corner,
            width,
            height,
            radius,
            Rc::clone(&self.cursor),
        )));

        self.objects.push(Rc::clone(&o) as Rc<RefCell<dyn Object>>);
        o
    }

    pub fn ellipse(
        &mut self,
        center: (f64, f64),
        a: f64,
        b: f64,
    ) -> Rc<RefCell<primitives::Ellipse>> {
        let o = Rc::new(RefCell::new(primitives::Ellipse::new(
            center,
            a,
            b,
            Rc::clone(&self.cursor),
        )));

        self.objects.push(Rc::clone(&o) as Rc<RefCell<dyn Object>>);
        o
    }

    pub fn arc(
        &mut self,
        center: (f64, f64),
        radius: f64,
        start_angle: f64,
        end_angle: f64,
    ) -> Rc<RefCell<primitives::Arc>> {
        let o = Rc::new(RefCell::new(primitives::Arc::new(
            center,
            radius,
            start_angle,
            end_angle,
            Rc::clone(&self.cursor),
        )));

        self.objects.push(Rc::clone(&o) as Rc<RefCell<dyn Object>>);
        o
    }

    pub fn line(
        &mut self,
        point1: (f64, f64),
        point2: (f64, f64),
    ) -> Rc<RefCell<primitives::Line>> {
        let o = Rc::new(RefCell::new(primitives::Line::new(
            point1,
            point2,
            Rc::clone(&self.cursor),
        )));

        self.objects.push(Rc::clone(&o) as Rc<RefCell<dyn Object>>);
        o
    }

    pub fn ring(
        &mut self,
        center: (f64, f64),
        inner_radius: f64,
        outer_radius: f64,
    ) -> Rc<RefCell<primitives::Ring>> {
        let o = Rc::new(RefCell::new(primitives::Ring::new(
            center,
            inner_radius,
            outer_radius,
            Rc::clone(&self.cursor),
        )));

        self.objects.push(Rc::clone(&o) as Rc<RefCell<dyn Object>>);
        o
    }

    pub fn capsule(
        &mut self,
        center1: (f64, f64),
        center2: (f64, f64),
        radius: f64,
    ) -> Rc<RefCell<primitives::Capsule>> {
        let o = Rc::new(RefCell::new(primitives::Capsule::new(
            center1,
            center2,
            radius,
            Rc::clone(&self.cursor),
        )));

        self.objects.push(Rc::clone(&o) as Rc<RefCell<dyn Object>>);
        o
    }

    pub fn text(
        &mut self,
        content: &str,
        position: (f64, f64),
        font_name: &str,
        font_size: f64,
        font_weight: FontWeight,
    ) -> Rc<RefCell<primitives::Text>> {
        let font = style::Font::from_collection(
            font_name,
            font_size,
            font_weight,
            &mut self.font_collection,
            &mut self.font_source_cache,
        )
        .unwrap_or_else(|e| panic!("Failed to load font '{}': {}", font_name, e));
        let o = Rc::new(RefCell::new(primitives::Text::new(
            content,
            position,
            font,
            Rc::clone(&self.cursor),
        )));

        self.objects.push(Rc::clone(&o) as Rc<RefCell<dyn Object>>);
        o
    }

    pub fn path(&mut self) -> Rc<RefCell<primitives::Path>> {
        let o = Rc::new(RefCell::new(primitives::Path::new(Rc::clone(&self.cursor))));

        self.objects.push(Rc::clone(&o) as Rc<RefCell<dyn Object>>);
        o
    }

    pub fn regular_polygon(
        &mut self,
        center: (f64, f64),
        radius: f64,
        sides: u32,
    ) -> Rc<RefCell<primitives::RegularPolygon>> {
        let o = Rc::new(RefCell::new(primitives::RegularPolygon::new(
            center,
            radius,
            sides,
            Rc::clone(&self.cursor),
        )));

        self.objects.push(Rc::clone(&o) as Rc<RefCell<dyn Object>>);
        o
    }

    pub fn wait(&mut self, secs: f64) {
        self.cursor.set(self.cursor.get() + secs);
    }

    pub(crate) fn draw_at(&self, t: f64, vscene: &mut vello::Scene) {
        vscene.reset();

        for obj in &self.objects {
            obj.borrow().draw_at(t, vscene);
        }
    }

    pub fn preview(self) -> Result<()> {
        render::run_preview(self)
    }

    pub fn export(&mut self) -> Result<()> {
        anyhow::bail!("Exporting is not implemented yet.")
    }
}
