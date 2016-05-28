//
// Author: Joshua Holmes
// 

extern crate sdl2;

use cpu;
use cpu::Cpu;
use sdl2::Sdl;
use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::Renderer;
use sdl2::render::Texture;

/// The display scale in relation to the native resolution of the system
pub const DISPLAY_SCALE: u32 = 30;

/// A structure to manage displaying the screen based on the system's VRAM
pub struct Display<'a> {
    renderer: Renderer<'a>,
    texture: Texture,
}

impl<'a> Display<'a> {
    /// Construct a new Display object
    pub fn new(sdl_context: &Sdl) -> Display<'a> {
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window("CHIP-8: This Time In Rust", 
            DISPLAY_SCALE * cpu::VIRTUAL_DISPLAY_WIDTH as u32, 
            DISPLAY_SCALE * cpu::VIRTUAL_DISPLAY_HEIGHT as u32)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let mut renderer = window.renderer().build().unwrap(); 

        renderer.set_draw_color(Color::RGB(16, 113, 145));
        renderer.clear();
        renderer.present();

        let texture = renderer.create_texture_streaming(
            PixelFormatEnum::RGB24, cpu::VIRTUAL_DISPLAY_WIDTH as u32, cpu::VIRTUAL_DISPLAY_HEIGHT as u32).unwrap();

        Display {
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

                    buffer[offset] = if bit { 255 } else { 16 };
                    buffer[offset + 1] = if bit { 255 } else { 113 };
                    buffer[offset + 2] = if bit { 255 } else { 145 };
                }
            }
        }).unwrap();

        // draw the texture
        self.renderer.copy(&self.texture, None, None);
        self.renderer.present();
    }
}