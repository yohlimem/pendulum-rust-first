use nannou::prelude::*;
use nannou::state::mouse::Mouse;

struct Bob{
    pos: Vec2,
    mass: f32,
}

struct Rope{
    pos: Vec2,
    length: f32,
    angle: f32,
    angular_velocity: f32,
    angular_acceleration: f32,
}

struct Model {
    _window: window::Id,
    rope: Rope,
    bob: Bob,
}


const GRAVITY: f32 = -9.8;
const DRAG: f32 = 0.999;

fn main() {
    nannou::app(model).update(update).run();
}


fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();
    let rope = Rope{
        pos: vec2(0.0,0.0),
        length: 200.0,
        angle: 5.0,
        angular_velocity: 0.0,
        angular_acceleration: 0.0,
    };
    let bob = Bob{
        pos: vec2(0.0,0.0),
        mass: 100.0,
    };
    Model {
        _window,
        rope,
        bob,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    // set the acceleration GRAVITY
    model.rope.angular_acceleration = (-GRAVITY / model.rope.length) * model.rope.angle.sin() * model.bob.mass;

    model.rope.angular_velocity += model.rope.angular_acceleration * 0.1;
    model.rope.angle += model.rope.angular_velocity* 0.1;

    model.bob.pos.x = model.rope.angle.sin() * model.rope.length;
    model.bob.pos.y = model.rope.angle.cos() * model.rope.length;


    // add drag to the pendulum
    model.rope.angular_velocity *= DRAG;

}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(PLUM);

    draw.ellipse().color(STEELBLUE).xy(_model.bob.pos).w_h(_model.bob.mass, _model.bob.mass);

    draw.line().start(_model.rope.pos).end(_model.bob.pos).weight(4.0).color(DODGERBLUE);

    draw.to_frame(app, &frame).unwrap();
}