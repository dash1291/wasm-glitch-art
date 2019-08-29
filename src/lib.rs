mod utils;

extern crate rand;
extern crate js_sys;

use wasm_bindgen::prelude::*;
use rand::Rng;
use wasm_bindgen::Clamped;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use web_sys::{CanvasRenderingContext2d, ImageData, HtmlCanvasElement};

#[wasm_bindgen]
pub struct GlitchImage {
    raw_pixels: Vec<u8>,
    width: u32,
    height: u32
}

#[wasm_bindgen]
impl GlitchImage {
    pub fn new(canvas: &HtmlCanvasElement, ctx: &CanvasRenderingContext2d) -> GlitchImage {

        let width = canvas.width();
        let height = canvas.height();

        let data = ctx.get_image_data(0.0, 0.0, width as f64, height as f64).unwrap();

        return GlitchImage {raw_pixels: data.data().to_vec(), width: width as u32, height: height as u32}
    }

    fn shift_x(&mut self, clip_x: u32, clip_y: u32, clip_h: u32, shift_w: u32) {
        let decrements = [
            40.0 * js_sys::Math::random(),
            40.0 * js_sys::Math::random(),
            40.0 * js_sys::Math::random()
        ];
        let mut buf8 = self.raw_pixels.to_vec();

        let mut y = clip_y;
        let mut x = 0;

        let mut idx = 0;
        let mut idx_orig = 0;
        let mut idx2 = 0;

        while y < clip_y + clip_h {
            x = 0;
            
            while x < clip_x + shift_w {
                idx = ((y * self.width) + x) * 4;
                buf8[idx as usize] = 255 - (decrements[0] as u8);
                buf8[(idx + 1) as usize] = 255 - (decrements[1] as u8);
                buf8[(idx + 2) as usize] = 255 - (decrements[2] as u8);
                buf8[(idx + 3) as usize] = 255;
                x += 1;
            }
            y += 1;
        }

        y = clip_y;

        while y < clip_y + clip_h {
            x = 0;
            
            while x < self.width - (clip_x + shift_w) {
                idx_orig = ((y * self.width) + x) * 4;
                idx2 =  ((y * self.width) + (x + clip_x + shift_w)) * 4;
                buf8[(idx2) as usize] = self.raw_pixels[idx_orig as usize] - (decrements[0] as u8);
                x += 1;
            }
            y += 1;
        }

        self.raw_pixels = buf8.to_vec();
    }

    pub fn paint_image(&mut self, canvas: &HtmlCanvasElement, ctx: &CanvasRenderingContext2d) {
        let new_img_data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut self.raw_pixels), canvas.width(), canvas.height());
        ctx.put_image_data(&new_img_data.unwrap(), 0.0, 0.0);
    }

    pub fn glitch_image(&mut self) {
        // iterate through pixel array
        let segments = [0.1, 0.05, 0.12, 0.08, 0.15, 0.1, 0.1, 0.1, 0.1, 0.1];
        let max_shift = 0.015 * self.width as f32;
        let base_x = 0.05 * self.width as f32;
        let mut shift_phase = 1.0;

        let clip_x = 0;
        let mut clip_y = 0.0;
        let mut clip_h = 0.0;
        let mut shift_w = 0.0;

        let mut i = 0;

        let mut segment = segments[i]; 
        
        while i < 9 {
            segment = segments[i];
            clip_h = segment * self.height as f32;
            let random = js_sys::Math::random() as f32;

            shift_w = random * max_shift * shift_phase;
            clip_y += clip_h;
            self.shift_x(0, clip_y as u32, clip_h as u32, (shift_w + base_x) as u32);
            shift_phase = shift_phase * -1.0;
            i += 1;
        }
    }
}

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}