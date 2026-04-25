# Quark

Create GPU-accelerated programmatic animations.

## Quick start

```rust
use quark_lib::{Color, Easing, FontWeight, Scene};

fn main() {
    let mut scene = Scene::new();

    scene.width = 1280;
    scene.height = 720;

    // Add a circle at (400, 300) with radius 80
    let circle = scene.circle((scene.width as f64 / 2.0, scene.height as f64 / 2.0), 80.0);

    scene.wait(1.0); // advance the cursor by 1 second

    // Animate the fill color to red over 2 seconds with smooth easing
    circle.borrow_mut()
        .style.fill
        .set_with(Color::RED, 2.0, Easing::SMOOTH);

    scene.wait(2.0);

    // Animate the radius
    circle.borrow_mut().radius.set(160.0, 1.0);

    scene.wait(1.0);

    let _ = scene.preview();
}
```

## Animating properties

Every field (position, radius, color, etc.) is an `Animated<T>`. There are three ways to update it:

```rust
// Tween to a new value over `duration` seconds (linear)
obj.field.set(target, duration);

// Tween with a specific easing
obj.field.set_with(target, duration, Easing::SMOOTH);

// Jump to a value immediately
obj.field.set_immediate(value);
```

## Preview window controls

| Key         | Action                            |
| ----------- | --------------------------------- |
| `space`     | pause / resume                    |
| `0`         | restart from beginning            |
| `←` / `→`   | seek backward / forward 2 seconds |
| `q` / `esc` | quit                              |

## Scene settings

```rust
let mut scene = Scene::new(); // defaults: 800×600, 60 fps, white background
scene.width = 1920;
scene.height = 1080;
scene.fps = 30;
scene.background = Color::BLACK;
```

## Roadmap

- [ ] Better co-ordinate system (relative co-ordinates?)
- [ ] SVG, Graphs and Typst/Latex as primitives
- [ ] Make video export possible
- [ ] Scripting support with lua

## License

This project is licensed under the MIT license, see [LICENSE](LICENSE).
