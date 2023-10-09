use nannou::prelude::*;
use nannou_audio as audio;
use nannou_audio::Buffer;

fn main() {
    nannou::app(model)
        .update(update)
        .view(view)
        .run();
}

struct Model {}

fn model(app: &App) -> Model {
    app.new_window()
        .size(640,480)
        .min_size(640,480)
        .max_size(640,480)
        //.decorations(false)
        .resizable(false)
        .build()
        .unwrap();

    Model{}
}

fn update(_app:&App, _model: &mut Model, _update: Update) {}

fn view( _app: &App, _model: &Model, _frame: Frame) {}
