extern crate sdl2;
extern crate gl;

use std::time::{Duration, Instant};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::libc::can_err_mask_t;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Texture, TextureCreator, WindowCanvas};
use sdl2::surface::Surface;
use sdl2::ttf::{self, Font};
use sdl2::gfx::{self};
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::video::WindowContext;
use gl::*;

const NANO_SEC_PER_SEC: u128 = 1000000000;
const UPDATE_TIMES_PER_SEC: u128 = 60;
const UPDATE_TIMES_PER_NANO_SEC: u128 = NANO_SEC_PER_SEC / UPDATE_TIMES_PER_SEC;

const WHITE: Color = Color::RGB(255, 255, 255);
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
                    Event::MouseMotion { x, y, .. } => {
                        println!("motion on 0{:?}", Point::new(x, y));
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
        let (o_width, o_height) = canvas.output_size().unwrap();
        render_bg(&mut canvas, WHITE);
        let text = format!("Time: {}(ms), FPS: {}", duration.as_secs(), current_fps);
        let (text_texture, width, height) = create_text_texture(&texture_creator, &font, GREEN, text.as_str());
        render_text(&mut canvas, &text_texture, width, height, Point::new(0,0));

        render(&mut canvas, Color::RGB(0, 0, 0), point + Point::new(o_width as i32 / 2, o_height as i32 / 2));
        let text = format!("(x: {}, y: {})", point.x(), point.y());
        let (text_texture, width, height) = create_text_texture(&texture_creator, &font, GREEN, text.as_str());
        render_text(&mut canvas, &text_texture, width, height, point.offset(o_width as i32 / 2 - 50, o_height as i32 / 2 + 50));
        canvas.present();
        painted_counter += 1;
        ::std::thread::sleep(Duration::new(0, UPDATE_TIMES_PER_NANO_SEC as u32));
    }
}

fn render_bg(
    canvas: &mut WindowCanvas,
    color: Color,
) {
    canvas.set_draw_color(color);
    canvas.clear();
}

fn create_text_texture<'a>(
    texture_creator: &'a TextureCreator<WindowContext>,
    font: &Font,
    color: Color,
    text: &str,
) -> (Texture<'a>, u32, u32) {
    let font_render_surface = font
        .render(&text)
        .blended(color)
        .unwrap();
    let width = font_render_surface.width();
    let height = font_render_surface.height();
    let font_texture = font_render_surface.as_texture(&texture_creator).unwrap();
    (font_texture, width, height)
}

fn render(
    canvas: &mut WindowCanvas,
    color: Color,
    position: Point) {
    canvas.set_draw_color(color);
    canvas.fill_rect(Rect::from_center(position, 100, 100));
}

fn render_text(
    canvas: &mut WindowCanvas,
    texture: &Texture,
    texture_width: u32,
    texture_height: u32,
    position: Point) {
    let render_pos = position.offset(texture_width as i32 / 2, texture_height as i32 / 2);
    canvas.copy(&texture,None, Rect::from_center(render_pos, texture_width, texture_height));
}