use image::{ImageFormat, RgbImage};
use palette::{FromColor, Oklch, ShiftHue, Srgb};
use std::env::args;
use std::io::{Cursor, IsTerminal, Write, stdout};
type Num = u8;
fn main() {
    let mut args = args();
    let Some(base) = args.nth(1).and_then(|s| s.parse::<u8>().ok()) else {
        return;
    };
    let Some(n) = args.next().and_then(|s| s.parse::<u16>().ok()) else {
        return;
    };
    if stdout().is_terminal() {
        let bnomials = bnomials(base, n);
        for row in bnomials {
            println!("{row:?}");
        }
    } else {
        let image = make_image(base, n);
        let mut buf = Cursor::new(Vec::new());
        image.write_to(&mut buf, ImageFormat::Png).unwrap();
        stdout().write_all(buf.get_ref()).unwrap();
    }
}
fn make_image(base: u8, n: u16) -> RgbImage {
    let bnomials = bnomials(base, n);
    let width_mult: usize = 2;
    let height_mult = width_mult;
    let width = width_mult as u32 * (base as u32 - 1) * n as u32;
    let height = height_mult as u32 * n as u32;
    let mut image = RgbImage::new(width, height);
    for (i, row) in bnomials.into_iter().enumerate() {
        for (j, v) in row.iter().enumerate() {
            let mut color = Oklch::from_color(Srgb::new(1.0, 0.0, 0.0));
            color = color.shift_hue((360.0 * v.rem_euclid(base as Num) as f32) / base as f32);
            let rgb = Srgb::<f32>::from_color(color);
            let rgb = [rgb.red, rgb.green, rgb.blue].map(|v| (255.0 * v) as u8);
            for y in height_mult * i..height_mult * (i + 1) {
                for mut x in width_mult * j..width_mult * (j + 1) {
                    x += width as usize / 2;
                    x -= (width_mult * row.len()) / 2;
                    image.get_pixel_mut(x as u32, y as u32).0 = rgb;
                }
            }
        }
    }
    image
}
fn bnomials(base: u8, n: u16) -> Box<[Box<[Num]>]> {
    fn get_layer(base: u8, last: &[Num], n: u16) -> Box<[Num]> {
        let mut vec = Vec::with_capacity((base as usize - 1) * n as usize + 1);
        for k in 0..vec.capacity() {
            vec.push(
                (0..=base as usize - 1)
                    .filter_map(|i| k.checked_sub(i).and_then(|i| last.get(i)))
                    .sum::<Num>()
                    .rem_euclid(base as Num),
            );
        }
        vec.into_boxed_slice()
    }
    let mut vec = Vec::with_capacity(n as usize);
    vec.push(vec![1].into_boxed_slice());
    for n in 1..vec.capacity() {
        let last = vec.last().unwrap();
        let set = get_layer(base, last, n as u16);
        vec.push(set);
    }
    vec.into_boxed_slice()
}
