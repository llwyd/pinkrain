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

    generators: u32,
    rollover: u32,
}

struct Model {
    _stream: audio::Stream<Pink>,
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
        generators: 15,
        rollover: 0,
    };
    
    pink.rollover = 2u32.pow(pink.generators - 1);

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
        .channels(1)
        .build()
        .unwrap();

    stream.play().unwrap();
    Model{_stream:stream}
}

fn pink_update(pink:&mut Pink) -> f32{

    let index = pink.counter.trailing_zeros();

    update_noise(&mut pink.white);
    update_noise(&mut pink.noise[index as usize]);

    pink.pink = pink.pink - pink.white.previous;
    pink.pink = pink.pink + pink.white.value;
    
    pink.pink = pink.pink - pink.noise[index as usize].previous;
    pink.pink = pink.pink + pink.noise[index as usize].value;

    pink.pink_norm = pink.pink / (pink.generators as f32 + 1.0);

    pink.counter = pink.counter & (pink.rollover - 1); 
    pink.counter = pink.counter + 1;

    pink.pink_norm
}

fn audio(pink:&mut Pink, buffer: &mut Buffer){
    for frame in buffer.frames_mut(){
        let pink = pink_update(pink);
        for channel in frame{
            *channel = pink;
            //*channel = (random::<f32>() * 2.0) - 1.0;
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
