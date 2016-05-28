//
// Author: Joshua Holmes
// 

extern crate sdl2;

use cpu;
use cpu::Cpu;
use sdl2::Sdl;
use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::render::Renderer;
use sdl2::render::Texture;

/// The display scale in relation to the native resolution of the system
pub const DISPLAY_SCALE: u32 = 20;
/// The color white
pub const WHITE: Color = Color::RGB(255, 255, 255);
/// The color black
pub const BLACK: Color = Color::RGB(0, 0, 0);

/// A structure to manage displaying the screen based on the system's VRAM
pub struct Display<'a> {
    pub sdl_context: Sdl,
    renderer: Renderer<'a>,
    texture: Texture,
}

impl<'a> Display<'a> {
    /// Construct a new Display object
    pub fn new() -> Display<'a> {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window("CHIP-8: This Time In Rust", 
            DISPLAY_SCALE * cpu::VIRTUAL_DISPLAY_WIDTH as u32, 
            DISPLAY_SCALE * cpu::VIRTUAL_DISPLAY_HEIGHT as u32)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let mut renderer = window.renderer().build().unwrap(); 

        renderer.set_draw_color(BLACK);
        renderer.clear();
        renderer.present();

        let mut texture = renderer.create_texture_streaming(
            PixelFormatEnum::RGB24, cpu::VIRTUAL_DISPLAY_WIDTH as u32, cpu::VIRTUAL_DISPLAY_HEIGHT as u32).unwrap();

        Display {
            sdl_context: sdl_context,
            renderer: renderer,
            texture: texture,
        }
    }

    /// Draws the screen given a CPU object whose VRAM we can read
    pub fn draw_screen(&mut self, cpu: &Cpu) {
        // update our texture with the system's VRAM
        self.texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
            for y in 0..cpu::VIRTUAL_DISPLAY_HEIGHT {
                for x in 0..cpu::VIRTUAL_DISPLAY_WIDTH {
                    let bit = cpu.vram[y][x];
                    let offset = (y * pitch) + (x * 3);

                    buffer[offset] = 0x00;
                    buffer[offset + 1] = if bit { 0xFF } else { 0x00 };
                    buffer[offset + 2] = 0x00;
                }
            }
        }).unwrap();

        // draw the texture
        self.renderer.clear();
        self.renderer.copy(&self.texture, None, None);
        self.renderer.present();
    }
}