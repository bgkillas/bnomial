use image::{ImageFormat, RgbImage};
use num_bigint::BigInt;
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
    let layer = args.next().and_then(|s| s.parse::<u8>().ok());
    if stdout().is_terminal() {
        let bnomials = bnomials(base, n);
        for row in bnomials {
            println!("{row:?}");
        }
    } else {
        let image = make_image(base, n, layer);
        let mut buf = Cursor::new(Vec::new());
        image.write_to(&mut buf, ImageFormat::Png).unwrap();
        stdout().write_all(buf.get_ref()).unwrap();
    }
}
fn make_image(base: u8, n: u16, layer: Option<u8>) -> RgbImage {
    let bnomials = bnomials(base, n);
    let width_mult: usize = 2;
    let height_mult = width_mult;
    let width = width_mult as u32 * (base as u32 - 1) * n as u32;
    let height = height_mult as u32 * n as u32;
    let mut image = RgbImage::new(width, height);
    for (i, row) in bnomials.into_iter().enumerate() {
        for (j, v) in row.iter().enumerate() {
            let mut color = Oklch::from_color(Srgb::new(1.0, 0.0, 0.0));
            if let Some(layer) = layer {
                if v.rem_euclid(base as Num) == layer {
                    color = color.shift_hue(180.0);
                }
            } else {
                let rem = v.rem_euclid(base as Num);
                if rem == 0 {
                    color = Oklch::from_color(Srgb::new(0.0, 0.0, 0.0));
                } else {
                    color = color.shift_hue((360.0 * rem as f32) / (base - 1) as f32);
                }
            }
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
#[allow(unused)]
fn bnomial(base: u8, n: u16, k: u16) -> BigInt {
    if n == 0 {
        return BigInt::new_const(1);
    }
    let inner = |i: u16| binomial(n, i) * binomial(n + k - (i * base as u16 + 1), n - 1);
    let mapped = |i: u16| if i.is_multiple_of(2) { 1 } else { -1 } * inner(i);
    (0..=k / base as u16).map(mapped).sum()
}
#[allow(unused)]
fn binomial(n: u16, k: u16) -> BigInt {
    let mut value = BigInt::new_const(1);
    for v in (n + 1) - k..=n {
        value *= v;
    }
    for v in 2..=k {
        value /= v;
    }
    value
}
#[test]
fn test() {
    use primes::{PrimeSet, Sieve};
    let mut pset = Sieve::new();
    for base in pset.iter().take(3) {
        for n in 0..=3 {
            let m = (base.pow(n) - 1) / (base - 1);
            for k in 0..=m * (base - 1) {
                assert_eq!(
                    bnomial(base as u8, m as u16, k as u16) % base,
                    BigInt::new_const(1)
                );
            }
            let m = base.pow(n);
            let sum = (0..=m * (base - 1))
                .map(|k| bnomial(base as u8, m as u16, k as u16) % base)
                .sum::<BigInt>();
            assert_eq!(sum, BigInt::from(base))
        }
    }
}
