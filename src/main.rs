use nannou::prelude::*;
use nannou_audio as audio;
use nannou_audio::Buffer;

mod pink;
pub use crate::pink::Pink;

fn main() {
    nannou::app(model)
        .update(update)
        .view(view)
        .run();
}

struct Model {
    _stream: audio::Stream<Pink>,
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

    let pink = Pink::new();
    let audio_host = audio::Host::new();
    let stream = audio_host
        .new_output_stream(pink)
        .render(audio)
        .channels(1)
        .build()
        .unwrap();

    stream.play().unwrap();
    Model{_stream:stream}
}

fn audio(pink:&mut Pink, buffer: &mut Buffer){
    for frame in buffer.frames_mut(){
        let pink = pink.update();
        for channel in frame{
            *channel = pink;
        } 
    }
} 

fn update(_app:&App, _model: &mut Model, _update: Update) {}

fn view( _app: &App, _model: &Model, _frame: Frame) {}
