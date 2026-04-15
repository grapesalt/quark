use crate::style::Color;
use std::cell::Cell;
use std::rc::Rc;

pub trait Animate: Clone {
    fn lerp(&self, other: &Self, t: f64) -> Self;
}

impl Animate for f64 {
    fn lerp(&self, other: &f64, t: f64) -> f64 {
        self + (other - self) * t
    }
}

impl Animate for (f64, f64) {
    fn lerp(&self, other: &(f64, f64), t: f64) -> (f64, f64) {
        (self.0.lerp(&other.0, t), self.1.lerp(&other.1, t))
    }
}

impl Animate for Color {
    fn lerp(&self, other: &Color, t: f64) -> Color {
        Color {
            r: self.r.lerp(&other.r, t),
            g: self.g.lerp(&other.g, t),
            b: self.b.lerp(&other.b, t),
            a: self.a.lerp(&other.a, t),
        }
    }
}

#[derive(Clone, Copy)]
pub struct Easing {
    func: fn(f64) -> f64,
}

impl Easing {
    pub const LINEAR: Easing = Easing { func: |t| t };
    pub const EASE_IN: Easing = Easing { func: |t| t * t };
    pub const EASE_OUT: Easing = Easing {
        func: |t| t * (2.0 - t),
    };
    pub const EASE_IN_OUT: Easing = Easing {
        func: |t| t * t * (3.0 - 2.0 * t),
    };
    pub const SMOOTH: Easing = Easing {
        func: |t| t * t * t * (t * (6.0 * t - 15.0) + 10.0),
    };

    pub fn new(func: fn(f64) -> f64) -> Self {
        Self { func }
    }

    pub fn apply(&self, t: f64) -> f64 {
        (self.func)(t)
    }
}

enum AnimKind<T: Animate> {
    Tween {
        from: T,
        to: T,
        duration: f64,
        easing: Easing,
    },
}

struct Keyframe<T: Animate> {
    start_time: f64,
    kind: AnimKind<T>,
}

pub struct Animated<T: Animate> {
    cursor: Rc<Cell<f64>>,
    initial: T,
    keyframes: Vec<Keyframe<T>>,
}

impl<T: Animate> Animated<T> {
    pub(crate) fn new(value: T, cursor: Rc<Cell<f64>>) -> Self {
        Animated {
            cursor,
            initial: value,
            keyframes: Vec::new(),
        }
    }

    pub fn set(&mut self, target: T, duration: f64) {
        self.set_with(target, duration, Easing::LINEAR);
    }

    pub fn set_with(&mut self, target: T, duration: f64, easing: Easing) {
        let cursor = self.cursor.get();
        let from = self.value_at(cursor);

        self.keyframes.push(Keyframe {
            start_time: cursor,
            kind: AnimKind::Tween {
                from,
                to: target,
                duration,
                easing,
            },
        });
    }

    pub fn set_immediate(&mut self, value: T) {
        let cursor = self.cursor.get();

        self.keyframes.push(Keyframe {
            start_time: cursor,
            kind: AnimKind::Tween {
                from: value.clone(),
                to: value,
                duration: 0.0,
                easing: Easing::LINEAR,
            },
        });
    }

    pub fn value_at(&self, t: f64) -> T {
        let kf = self.keyframes.iter().rev().find(|kf| kf.start_time <= t);

        match kf {
            None => self.initial.clone(),
            Some(kf) => {
                let elapsed = t - kf.start_time;

                match &kf.kind {
                    AnimKind::Tween {
                        from,
                        to,
                        duration,
                        easing,
                    } => {
                        if *duration == 0.0 {
                            return to.clone();
                        }
                        from.lerp(to, easing.apply((elapsed / duration).min(1.0)))
                    }
                }
            }
        }
    }
}
