use quark_lib::anim::Easing;
use quark_lib::style::Color;
use quark_lib::Scene;
use std::f64::consts::{PI, TAU};

fn main() {
    let mut scene = Scene::new();
    scene.width = 1920;
    scene.height = 1080;
    scene.background = Color::BLACK;

    let cx = scene.width as f64 / 2.0;
    let cy = scene.height as f64 / 2.0;

    scene.wait(2.0);

    let count = 2000;
    let mut circles = Vec::new();
    for i in 0..count {
        let angle = TAU * (i as f64) / (count as f64);
        let ring = (i % 8) as f64;
        let dist = 80.0 + ring * 55.0;
        let c = scene.circle((cx + dist * angle.cos(), cy + dist * angle.sin()), 2.5);
        {
            let mut c = c.borrow_mut();
            c.style
                .fill
                .set_immediate(Color::hsv(i as f64 / count as f64, 1.0, 1.0));
            c.style.stroke_width.set_immediate(0.0);

            let phase = (i as f64 / count as f64) * 0.8;

            c.radius.set_with(7.0, 0.4 + phase, Easing::EASE_OUT);
            c.radius.set_with(2.5, 0.4, Easing::EASE_IN);
        }
        circles.push(c);
    }

    scene.wait(2.0);
    for c in &circles {
        c.borrow_mut().hide();
    }

    let cols = 25usize;
    let rows = 20usize;
    let tw = scene.width as f64 / cols as f64;
    let th = scene.height as f64 / rows as f64;
    let mut rects = Vec::new();
    for row in 0..rows {
        for col in 0..cols {
            let x = col as f64 * tw;
            let y = row as f64 * th;
            let r = scene.rect((x, y), tw - 2.0, th - 2.0);
            {
                let mut r = r.borrow_mut();
                let hue = (row * cols + col) as f64 / (rows * cols) as f64;
                r.style.fill.set_immediate(Color::hsv(hue, 0.8, 0.5));
                r.style.stroke_width.set_immediate(0.0);
                let delay = ((row + col) as f64 / (rows + cols) as f64) * 1.0;
                r.width.set_with(tw * 1.5, 0.3 + delay, Easing::EASE_IN_OUT);
                r.height
                    .set_with(th * 1.5, 0.3 + delay, Easing::EASE_IN_OUT);
            }
            rects.push(r);
        }
    }

    scene.wait(2.0);
    for r in &rects {
        r.borrow_mut().hide();
    }

    let ne = 300usize;
    let mut ellipses = Vec::new();
    for i in 0..ne {
        let angle = TAU * (i as f64) / (ne as f64);
        let dist = 100.0 + (i % 6) as f64 * 70.0;
        let e = scene.ellipse(
            (cx + dist * angle.cos(), cy + dist * angle.sin()),
            30.0,
            8.0,
        );
        {
            let mut e = e.borrow_mut();
            e.style
                .fill
                .set_immediate(Color::hsv(i as f64 / ne as f64, 1.0, 0.9));
            e.style.stroke_width.set_immediate(0.0);
            e.a.set_with(8.0, 1.0, Easing::EASE_IN_OUT);
            e.b.set_with(30.0, 1.0, Easing::EASE_IN_OUT);
        }
        ellipses.push(e);
    }

    scene.wait(1.5);
    for e in &ellipses {
        e.borrow_mut().hide();
    }

    let na = 200usize;
    let mut arcs = Vec::new();
    for i in 0..na {
        let angle_offset = TAU * (i as f64) / (na as f64);
        let r = 50.0 + (i % 5) as f64 * 80.0;
        let a = scene.arc((cx, cy), r, angle_offset, angle_offset + PI * 0.1);
        {
            let mut a = a.borrow_mut();
            a.style
                .stroke
                .set_immediate(Color::hsv(i as f64 / na as f64, 1.0, 1.0));
            a.style.stroke_width.set_immediate(3.0);
            a.end_angle
                .set_with(angle_offset + TAU, 1.5, Easing::EASE_IN_OUT);
        }
        arcs.push(a);
    }

    scene.wait(1.5);
    for a in &arcs {
        a.borrow_mut().hide();
    }

    let nl = 400usize;
    let mut lines = Vec::new();
    for i in 0..nl {
        let angle = TAU * (i as f64) / (nl as f64);
        let x1 = cx + 20.0 * angle.cos();
        let y1 = cy + 20.0 * angle.sin();
        let x2 = cx + 480.0 * angle.cos();
        let y2 = cy + 480.0 * angle.sin();
        let l = scene.line((x1, y1), (x2, y2));
        {
            let mut l = l.borrow_mut();
            l.style
                .stroke
                .set_immediate(Color::hsv(i as f64 / nl as f64, 0.7, 1.0));
            l.style.stroke_width.set_immediate(1.5);
            l.point2.set_with(
                (cx + 800.0 * angle.cos(), cy + 800.0 * angle.sin()),
                1.0,
                Easing::EASE_OUT,
            );
        }
        lines.push(l);
    }

    scene.wait(1.0);
    for l in &lines {
        l.borrow_mut().hide();
    }

    let nr = 150usize;
    let mut rings = Vec::new();
    for i in 0..nr {
        let angle = TAU * (i as f64) / (nr as f64);
        let dist = 200.0 + (i % 4) as f64 * 100.0;
        let x = cx + dist * angle.cos();
        let y = cy + dist * angle.sin();
        let ring = scene.ring((x, y), 5.0, 20.0);
        {
            let mut ring = ring.borrow_mut();
            ring.style
                .fill
                .set_immediate(Color::hsv(i as f64 / nr as f64, 1.0, 1.0));
            ring.style.stroke_width.set_immediate(0.0);
            ring.outer_radius.set_with(50.0, 0.8, Easing::EASE_OUT);
            ring.inner_radius.set_with(35.0, 0.8, Easing::EASE_OUT);
        }
        rings.push(ring);
    }

    scene.wait(1.0);
    for r in &rings {
        r.borrow_mut().hide();
    }

    let nc = 100usize;
    let mut capsules = Vec::new();
    for i in 0..nc {
        let angle = TAU * (i as f64) / (nc as f64);
        let dist = 150.0 + (i % 3) as f64 * 120.0;
        let x1 = cx + dist * angle.cos();
        let y1 = cy + dist * angle.sin();
        let x2 = x1 + 60.0 * (angle + PI * 0.5).cos();
        let y2 = y1 + 60.0 * (angle + PI * 0.5).sin();
        let cap = scene.capsule((x1, y1), (x2, y2), 10.0);
        {
            let mut cap = cap.borrow_mut();
            cap.style
                .fill
                .set_immediate(Color::hsv(i as f64 / nc as f64, 0.9, 1.0));
            cap.style.stroke_width.set_immediate(0.0);
            let x2_far = x1 + 160.0 * (angle + PI * 0.5).cos();
            let y2_far = y1 + 160.0 * (angle + PI * 0.5).sin();
            cap.center2
                .set_with((x2_far, y2_far), 1.0, Easing::EASE_IN_OUT);
            cap.radius.set_with(5.0, 1.0, Easing::EASE_IN_OUT);
        }
        capsules.push(cap);
    }

    scene.wait(1.0);
    for c in &capsules {
        c.borrow_mut().hide();
    }

    let np = 200usize;
    let mut polygons = Vec::new();
    for i in 0..np {
        let angle = TAU * (i as f64) / (np as f64);
        let dist = 100.0 + (i % 7) as f64 * 60.0;
        let x = cx + dist * angle.cos();
        let y = cy + dist * angle.sin();
        let sides = 3 + (i % 6) as u32;
        let poly = scene.regular_polygon((x, y), 10.0, sides);
        {
            let mut poly = poly.borrow_mut();
            poly.style
                .fill
                .set_immediate(Color::hsv(i as f64 / np as f64, 1.0, 1.0));
            poly.style.stroke_width.set_immediate(0.0);
            poly.radius.set_with(35.0, 0.8, Easing::SMOOTH);
        }
        polygons.push(poly);
    }

    scene.wait(1.0);
    for p in &polygons {
        p.borrow_mut().hide();
    }

    let nrr = 100usize;
    let mut rounded_rects = Vec::new();
    for i in 0..nrr {
        let angle = TAU * (i as f64) / (nrr as f64);
        let dist = 180.0 + (i % 4) as f64 * 80.0;
        let x = cx + dist * angle.cos() - 30.0;
        let y = cy + dist * angle.sin() - 20.0;
        let rr = scene.rounded_rect((x, y), 60.0, 40.0, 5.0);
        {
            let mut rr = rr.borrow_mut();
            rr.style
                .fill
                .set_immediate(Color::hsv(i as f64 / nrr as f64, 0.8, 1.0));
            rr.style.stroke_width.set_immediate(0.0);
            rr.radius.set_with(20.0, 0.8, Easing::EASE_IN_OUT);
            rr.width.set_with(80.0, 0.8, Easing::EASE_IN_OUT);
        }
        rounded_rects.push(rr);
    }

    scene.wait(1.0);
    for rr in &rounded_rects {
        rr.borrow_mut().hide();
    }

    scene.wait(0.5);
    let _ = scene.preview();
}
