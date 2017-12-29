extern crate sdl2;

mod game_of_life;

use sdl2::rect::{Rect, Point};
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::mouse::MouseButton;
use sdl2::keyboard::Keycode;
use sdl2::image::{ LoadTexture, INIT_JPG, INIT_PNG };
use sdl2::video::{Window, WindowContext};
use sdl2::render::{Canvas, Texture, TextureCreator, TextureQuery};

use game_of_life::{ State, GameOfLife, SQUARE_SIZE, PLAYGROUND_WIDTH, PLAYGROUND_HEIGHT };

fn main() {

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let _image_context = sdl2::image::init(INIT_PNG | INIT_JPG).unwrap();

    let ttf_context = sdl2::ttf::init().unwrap();

    let window = video_subsystem
            .window("GOL",
                    SQUARE_SIZE*PLAYGROUND_WIDTH,
                    SQUARE_SIZE*PLAYGROUND_HEIGHT)
            .position_centered()
            .build()
            .unwrap();
    
    let mut canvas = window.into_canvas()
        .target_texture()
        .present_vsync()
        .build()
        .unwrap();

    println!("Using SDL Renderer \"{}\"", canvas.info().name);
    canvas.set_draw_color(Color::RGB(0,0,0));
    canvas.clear();
    canvas.present();

    let texture_creator: TextureCreator<_> = canvas.texture_creator();

    let mut font = ttf_context.load_font("resource//font//DroidSans.ttf", 20).unwrap();

    let bg = texture_creator.load_texture("resource//img//bg.png").unwrap();
    let square_texture1 = texture_creator.load_texture("resource//img//one.png").unwrap();
    let square_texture2 = texture_creator.load_texture("resource//img//two.png").unwrap();
    let slat = texture_creator.load_texture("resource//img//slat.png").unwrap();

    let mut game = game_of_life::GameOfLife::new();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut frame: u32 = 0;

    let mut ticks: u32 = 0;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {.. } | Event::KeyDown { keycode: Some(Keycode::Escape), ..} => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::Space), repeat: false, .. } => {
                    game.toggle_state();
                },
                Event::MouseButtonDown { x, y, mouse_btn: MouseButton::Left, .. } => {
                    let x = (x as u32) / SQUARE_SIZE;
                    let y = (y as u32) / SQUARE_SIZE;
                    match game.get_mut(x as i32, y as i32) {
                        Some(square) => {*square = !(*square);},
                        None => { panic!()}
                    };
                },
                _ => {}
            }
        }

        if frame >= 30 {
            game.update();
            frame = 0;
            ticks += 1;
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        for (i, unit) in (&game).into_iter().enumerate() {
            let i = i as u32;
            let square_texture = if frame >= 15 {
                &square_texture1
            } else {
                &square_texture2
            };

            let x = ((i % PLAYGROUND_WIDTH) * SQUARE_SIZE) as i32;
            let y = ((i / PLAYGROUND_WIDTH) * SQUARE_SIZE) as i32;

            if *unit {
                canvas.copy(square_texture,
                            None,
                            Rect::new(x, y, 
                                      SQUARE_SIZE,
                                      SQUARE_SIZE)).unwrap();
            } else {
                canvas.copy(&bg,
                            None,
                            Rect::new(x, y, SQUARE_SIZE, SQUARE_SIZE)).unwrap();
            }
        }

        let surface = font.render(&format!("Game ticks = {}", ticks)).blended(Color::RGBA(255, 0, 0, 255)).unwrap();
        let texture = texture_creator.create_texture_from_surface(&surface).unwrap();
        let TextureQuery { width, height, .. } = texture.query();

        canvas.copy(&slat, None, Rect::new(10-3, 10-3, width+6, height+6)).unwrap();
        canvas.copy(&texture, None, Rect::new(10, 10, width, height)).unwrap();

        canvas.present();

        if let game_of_life::State::Playing = game.state() {
            frame += 1;
        }
    }
}
