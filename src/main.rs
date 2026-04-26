use quark_lib;

fn main() {
    let mut scene = quark_lib::Scene::new();

    scene.width = 1280;
    scene.height = 720;

    let c = scene.circle((0.0, 0.0), 10.0);
    c.borrow_mut()
        .center
        .set_dynamic(|t| (t * 100.0 - 630.0, t.sin() * 100.0));

    c.borrow_mut().radius.set_dynamic(|t| 15.0 * t.sin().abs());

    let _ = scene.preview();
}
