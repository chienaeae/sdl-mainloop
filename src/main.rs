extern crate sdl2;

use std::time::{Duration, Instant};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::libc::can_err_mask_t;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Texture, TextureCreator, WindowCanvas};
use sdl2::surface::Surface;
use sdl2::ttf::{self, Font};
use sdl2::video::WindowContext;

const NANO_SEC_PER_SEC: u128 = 1000000000;
const UPDATE_TIMES_PER_SEC: u128 = 60;
const UPDATE_TIMES_PER_NANO_SEC: u128 = NANO_SEC_PER_SEC / UPDATE_TIMES_PER_SEC;

const WHITE: Color = Color::RGB(255,255,255);
const GREEN: Color = Color::RGB(24, 181, 79);

fn main() {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let font_subsystem = ttf::init().unwrap();
    let window = video_subsystem
        .window("GameLoop Test", 900, 700)
        .resizable()
        .build()
        .unwrap();
    let font = font_subsystem
        .load_font("assets/OpenSans-Semibold.ttf", 22)
        .unwrap();
    // For Event
    let mut event_pump = sdl.event_pump().unwrap();
    // For Render
    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();
    let mut point = Point::new(0, 0);
    // For time loop
    let mut updated_counter: i128 = 0;
    let mut painted_counter: i128 = 0;
    let mut current_fps = 0;
    let mut fps_timer = Instant::now();
    let mut duration = Duration::new(0, 0);
    let mut started_time = Instant::now();
    'main: loop {
        duration = Instant::now().duration_since(started_time);
        let target_updated_times = (duration.as_nanos() / UPDATE_TIMES_PER_NANO_SEC as u128) as i128;
        while updated_counter < target_updated_times {
            for event in event_pump.poll_iter() {
                if let Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } = event {
                    break 'main;
                }
                match event {
                    Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                        point = point.offset(0, -4);
                    }
                    Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                        point = point.offset(0, 4);
                    }
                    Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                        point = point.offset(-4, 0);
                    }
                    Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                        point = point.offset(4, 0);
                    }
                    _ => {}
                }
            }
            updated_counter += 1;
        }

        if Instant::now().duration_since(fps_timer).as_nanos() >= NANO_SEC_PER_SEC {
            current_fps = painted_counter;
            fps_timer = Instant::now();
            painted_counter = 0;
        }

        // Paint
        render_bg(&mut canvas, WHITE);
        let text = format!("Time: {}(ms), FPS: {}", duration.as_secs(), current_fps);
        render_text(&mut canvas, &texture_creator, &font, text.as_str(), Point::new(10, 10), GREEN);
        render_rect(&mut canvas, point);
        canvas.present();
        painted_counter += 1;
        ::std::thread::sleep(Duration::new(0, UPDATE_TIMES_PER_NANO_SEC as u32));
    }
}

fn render_bg(
    canvas: &mut WindowCanvas,
    color: Color
){
    canvas.set_draw_color(color);
    canvas.clear();
}

fn render_text(
    canvas: &mut WindowCanvas,
    texture_creator: &TextureCreator<WindowContext>,
    font: &Font,
    text: &str,
    position: Point,
    color: Color
) {
    let font_render_surface = font
        .render(&text)
        .blended(color)
        .unwrap();
    let width = font_render_surface.width();
    let height = font_render_surface.height();
    let font_texture = font_render_surface.as_texture(&texture_creator).unwrap();
    canvas.set_draw_color(color);
    canvas.copy(&font_texture, None, Rect::from_center(position + Point::new(width as i32 / 2, height as i32 / 2), width, height));
}


fn render_rect(
    canvas: &mut WindowCanvas,
    point: Point
) {
    let (width, height) = canvas.output_size().unwrap();
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    let p = point.offset(50, 50);
    canvas.fill_rect(Rect::from_center(p + Point::new(width as i32 / 2, height as i32 / 2), 100, 100));
}