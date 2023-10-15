use nannou::prelude::*;
use nannou_audio as audio;
use nannou_audio::Buffer;
use rand::Rng;

mod pink;
mod noise;
pub use crate::pink::Pink;


const MAX_RAINDROPS: u32 = 1000;

/* Scaling factors based on 640 * 480 res */
const RAIN_MIN_PERCENT: f32 = 0.0417;
const RAIN_MAX_PERCENT: f32 = 4.17;
const RAIN_THICKNESS: f32 = 0.45;
const RAIN_SPEED_PERCENT: f32 = 0.084;

fn main() {
    nannou::app(model)
        .update(update)
        .view(view)
        .run();
}

struct RainDrop{
    position: Point2,
    size: f32,
    speed: f32,
}

struct StereoPink{
    l: Pink,
    r: Pink,
}

struct Model {
    _stream: audio::Stream<StereoPink>,
    raindrop: Vec<RainDrop>,
}


fn model(app: &App) -> Model {
    let window = app.new_window()
        .size(640,480)
        .decorations(false)
        .resizable(true)
        .resized(resized)
        .build()
        .unwrap();

    app.window(window)
        .expect("Failed to get window")
        .set_cursor_visible(false); 
    
    let pink = StereoPink{
        l: Pink::new(),
        r: Pink::new(),
    };
    
    let audio_host = audio::Host::new();
    let stream = audio_host
        .new_output_stream(pink)
        .render(audio)
        .channels(2)
        .sample_rate(44_100)
        .build()
        .unwrap();

    stream.play().unwrap();
    let win = app.window_rect();
    let mut model = Model{
        _stream:stream,
        raindrop: Vec::new(),
    };


    let mut rng = rand::thread_rng();
    for _i in 0..MAX_RAINDROPS{
        let x = rng.gen_range(win.left()..win.right());
        let y = rng.gen_range(win.bottom()..win.top());
        
        let rain_max = win.wh() * RAIN_MAX_PERCENT;
        let rain_min = win.wh() * RAIN_MIN_PERCENT;
        let rain_speed = win.wh().y * RAIN_SPEED_PERCENT;

        let raindrop = RainDrop{
            position: pt2(x,y),
            size: rng.gen_range(rain_min.y..rain_max.y),
            speed: rain_speed,
        };
        model.raindrop.push(raindrop);
    }

    model
}

fn audio(pink:&mut StereoPink, buffer: &mut Buffer){
    for frame in buffer.frames_mut(){
        let pink_l = pink.l.update();
        let pink_r = pink.r.update();

        frame[0] = pink_l;
        frame[1] = pink_r;
    }
} 

fn resized(_app: &App, model: &mut Model, dim: Vec2){
    
    let rain_max = dim * RAIN_MAX_PERCENT;
    let rain_min = dim * RAIN_MIN_PERCENT;
    let rain_speed = dim.y * RAIN_SPEED_PERCENT;
    let mut rng = rand::thread_rng();
    
    for raindrop in &mut model.raindrop{
        raindrop.size = rng.gen_range(rain_min.y..rain_max.y);
        raindrop.speed = rain_speed;
    }
}

fn update(app:&App, model: &mut Model, _update: Update) {
    let win = app.window_rect();
    
    for raindrop in &mut model.raindrop{
        if(raindrop.position.y + raindrop.size) <= win.bottom(){
            let mut rng = rand::thread_rng();
            raindrop.position.y = win.top();
            raindrop.position.x = rng.gen_range(win.left()..win.right());
        }
        else{
            raindrop.position.y -= raindrop.speed;
        }
    }
}

fn view( app: &App, model: &Model, frame: Frame) {

    let draw = app.draw();
    draw.background().color(BLACK);

    for raindrop in &model.raindrop{
        draw.line()
            .start(raindrop.position)
            .end(pt2(raindrop.position.x, raindrop.position.y + raindrop.size))
            .weight(RAIN_THICKNESS)
            .color(GREY);
    }
    draw.to_frame(app, &frame).unwrap();
}

