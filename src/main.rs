extern crate sdl2;

mod test;
mod game_of_life;

use sdl2::rect::{Rect, Point};
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::mouse::MouseButton;
use sdl2::keyboard::Keycode;
use sdl2::video::{Window, WindowContext};
use sdl2::render::{Canvas, Texture, TextureCreator};

use game_of_life::{ State, GameOfLife, SQUARE_SIZE, PLAYGROUND_WIDTH, PLAYGROUND_HEIGHT };

fn dummy_texture<'a>(
    canvas: &mut Canvas<Window>, 
    texture_creator: &'a TextureCreator<WindowContext>
    ) -> (Texture<'a>, Texture<'a>) {

    enum TextureColor {
        Yellow,
        White,
    };

    let mut square_texture1: Texture = 
        texture_creator.create_texture_target(None, SQUARE_SIZE, SQUARE_SIZE).unwrap();
    let mut square_texture2: Texture = 
        texture_creator.create_texture_target(None, SQUARE_SIZE, SQUARE_SIZE).unwrap();
    
    {
        let textures = vec![
            (&mut square_texture1, TextureColor::Yellow),
            (&mut square_texture2, TextureColor::White)
        ];

        canvas.with_multiple_texture_canvas(textures.iter(), |texture_canvas, user_context| {
            texture_canvas.set_draw_color(Color::RGB(0, 0, 0));
            texture_canvas.clear();
            match *user_context {
                TextureColor::Yellow => {
                    for i in 0..SQUARE_SIZE {
                        for j in 0..SQUARE_SIZE {
                            if (i+j) % 4 == 0 {
                                texture_canvas.set_draw_color(Color::RGB(255, 255, 0));
                                texture_canvas.draw_point(Point::new(i as i32, j as i32)).unwrap();
                            }
                            if (i+j*2) % 9 == 0 {
                                texture_canvas.set_draw_color(Color::RGB(200, 200, 0));
                                texture_canvas.draw_point(Point::new(i as i32, j as i32)).unwrap();
                            }
                        }
                    }
                },
                TextureColor::White => {
                    for i in 0..SQUARE_SIZE {
                        for j in 0..SQUARE_SIZE {
                            if (i+j) % 7 == 0 {
                                texture_canvas.set_draw_color(Color::RGB(192, 192, 192));
                                texture_canvas.draw_point(Point::new(i as i32, j as i32)).unwrap();
                            }
                            if (i+j*2) % 5 == 0 {
                                texture_canvas.set_draw_color(Color::RGB(64, 64, 64));
                                texture_canvas.draw_point(Point::new(i as i32, j as i32)).unwrap();
                            }
                        }
                    }
                }
            };
            for i in 0..SQUARE_SIZE {
                for j in 0..SQUARE_SIZE {
                    if (i+j) % 7 == 0 {
                        texture_canvas.set_draw_color(Color::RGB(192, 192, 192));
                        texture_canvas.draw_point(Point::new(i as i32, j as i32)).unwrap();
                    }
                    if (i+j*2) % 5 == 0 {
                        texture_canvas.set_draw_color(Color::RGB(64, 64, 64));
                        texture_canvas.draw_point(Point::new(i as i32, j as i32)).unwrap();
                    }
                }
            }
        }).unwrap();
    }
    (square_texture1, square_texture2)
}

fn main() {

    let test_t1 = test::t1::new(false);
    let res = test_t1.get();
    println!("Starting {}", res);

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
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

    // set up game
    let (square_texture1, square_texture2) = dummy_texture(&mut canvas, &texture_creator);
    let mut game = game_of_life::GameOfLife::new();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut frame: u32 = 0;

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
            println!("GOL TICK");
        }

        //drawe
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        for (i, unit) in (&game).into_iter().enumerate() {
            let i = i as u32;
            let square_texture = if frame >= 15 {
                &square_texture1
            } else {
                &square_texture2
            };
            if *unit {
                canvas.copy(square_texture,
                            None,
                            Rect::new(((i % PLAYGROUND_WIDTH) * SQUARE_SIZE) as i32,
                                      ((i / PLAYGROUND_WIDTH) * SQUARE_SIZE) as i32,
                                      SQUARE_SIZE,
                                      SQUARE_SIZE)).unwrap();
            }
        }
        canvas.present();

        if let game_of_life::State::Playing = game.state() {
            frame += 1;
        }
    }
}
