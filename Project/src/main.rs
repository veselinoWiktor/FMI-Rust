use nannou::prelude::*;

struct Model {
    growth: f32,
}

fn model(app: &App) -> Model {
    app.new_window().size(800, 800).view(view).build().unwrap();
    Model { growth: 0.0 }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.growth += 0.01;
    if model.growth > 1.0 {
        model.growth = 0.0;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    let petal_count = 8;
    let max_radius = 300.0;

    for i in 0..petal_count {
        let angle = i as f32 * TAU / petal_count as f32;
        let x = angle.cos() * model.growth * max_radius;
        let y = angle.sin() * model.growth * max_radius;

        draw.ellipse()
            .x_y(x, y)
            .w_h(80.0 * model.growth, 150.0 * model.growth)
            .rotate(angle)
            .rgba(1.0, 0.5, 0.8, 0.6);
    }

    draw.to_frame(app, &frame).unwrap();
}

fn main() {
    nannou::app(model).update(update).run();
}