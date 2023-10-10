use nannou::prelude::*;
use nannou_audio as audio;
use nannou_audio::Buffer;
use rand::random;

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

#[derive(Copy, Clone)]
struct Noise{
    previous:f32,
    value:f32,
}

struct Pink{
    noise: [Noise; 15], // updated based on trailing zeros
    white: Noise, // Updated every iteration
    pink: f32, // Actual noise
    
    pink_norm: f32,
    counter: u32,
}

struct Model {
    //stream: audio::Stream<Resonator>,
}

fn update_noise(noise:&mut Noise){
    noise.previous = noise.value;
    noise.value = (random::<f32>() * 2.0) - 1.0;
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

    let mut pink = Pink{
        noise:[Noise{previous:0.0, value:0.0}; 15],
        white: Noise{previous:0.0, value:0.0},
        pink: 0.0,
        pink_norm: 0.0,
        counter: 1,
    };
    
    update_noise(&mut pink.white);
    let generators = 15;
    for i in 0..generators{
        update_noise(&mut pink.noise[i]);
        pink.pink += pink.noise[i].value;
    }
    pink.pink += pink.white.value;

    let audio_host = audio::Host::new();
    let stream = audio_host
        .new_output_stream(pink)
        .render(audio)
        .build()
        .unwrap();

    stream.play().unwrap();
    Model{}
}

fn audio(pink:&mut Pink, buffer: &mut Buffer){
    for frame in buffer.frames_mut(){
        update_noise(&mut pink.white);
        for channel in frame{
            *channel = (random::<f32>() * 2.0) - 1.0;
        } 
    }
} 

#[allow(dead_code)]
fn aaudio(res:&mut Resonator, buffer: &mut Buffer)
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
