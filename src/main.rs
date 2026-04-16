use quark_lib;
use quark_lib::Scene;

fn main() {
    let mut scene = Scene::new();

    scene.width = 1280;
    scene.height = 720;

    let text = scene.text(
        "Hello, World!",
        (400.0, 300.0),
        "VictorMono Nerd Font",
        32.0,
        quark_lib::FontWeight::Regular,
    );

    scene.wait(2.0);

    text.borrow_mut()
        .color
        .set_with(quark_lib::Color::RED, 2.0, quark_lib::Easing::SMOOTH);

    scene.wait(2.0);

    text.borrow_mut()
        .position
        .set_with((400.0, 100.0), 2.0, quark_lib::Easing::SMOOTH);

    text.borrow_mut().font_size.set(64.0, 2.0);

    scene.wait(1.0);

    text.borrow_mut().style.stroke_width.set(2.0, 2.0);
    text.borrow_mut()
        .style
        .stroke
        .set(quark_lib::Color::BLACK, 2.0);

    let _ = scene.preview();
}
