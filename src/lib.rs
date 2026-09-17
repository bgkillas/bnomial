use crate::bnomial::make_image;
use image::RgbaImage;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, ImageData};
pub mod bnomial;
#[wasm_bindgen(start)]
pub fn start() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    let window = web_sys::window().unwrap();
    let hash = window.location().hash().unwrap();
    let mut base = 3;
    let mut n = window.inner_height().unwrap().as_f64().unwrap() as u16;
    let mut layer = None;
    if !hash.is_empty() {
        let mut split = hash[1..].split(",");
        if let Some(next) = split.next().and_then(|s| s.parse().ok()) {
            base = next;
        }
        if let Some(next) = split.next().and_then(|s| s.parse().ok()) {
            n = next;
        }
        layer = split.next().and_then(|s| s.parse().ok());
    }
    let image = make_image(base, n, layer);
    draw_image(&image);
}
pub fn draw_image(image: &RgbaImage) {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();
    let (width, height) = image.dimensions();
    canvas.set_width(width);
    canvas.set_height(height);
    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();
    let data = ImageData::new_with_u8_clamped_array_and_sh(
        wasm_bindgen::Clamped(image.as_raw()),
        width,
        height,
    )
    .unwrap();
    context.set_image_smoothing_enabled(false);
    context.put_image_data(&data, 0.0, 0.0).unwrap();
}
