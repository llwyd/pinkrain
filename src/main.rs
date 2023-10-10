use nannou::prelude::*;
use nannou_audio as audio;
use nannou_audio::Buffer;
use std::f32::consts::PI;

fn main() {
    nannou::app(model)
        .update(update)
        .view(view)
        .run();
}


struct Resonator
{
    a1: f32,
    a2: f32,
    y: [f32;2],
}

struct Model {
    //stream: audio::Stream<Resonator>,
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(640,480)
        .min_size(640,480)
        .max_size(640,480)
        //.decorations(false)
        .resizable(false)
        .build()
        .unwrap();

    let omega:f32 = (2.0 * PI * 500.0) / 44100.0;
    let amplitude = 0.9;
    let b0:f32 =  amplitude * omega.sin();

    let res = Resonator{
        a1: -2.0 * omega.cos(),
        a2: 1.0,
        y: [b0, 0.0],
    };

    let audio_host = audio::Host::new();
    let stream = audio_host
        .new_output_stream(res)
        .render(audio)
        .build()
        .unwrap();

    stream.play().unwrap();

    Model{}
}

fn audio(res:&mut Resonator, buffer: &mut Buffer)
{
    for frame in buffer.frames_mut(){
        let y = -(res.a1 * res.y[0]) - (res.a2 * res.y[1]);
        for channel in frame{
            *channel = y;
        }
        
        res.y[1] = res.y[0];
        res.y[0] = y;
    }
}

fn update(_app:&App, _model: &mut Model, _update: Update) {}

fn view( _app: &App, _model: &Model, _frame: Frame) {}
