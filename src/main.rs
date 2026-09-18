use bnomial_lib::bnomial::{bnomials, make_image};
use image::ImageFormat;
use std::env::args;
use std::io::{Cursor, IsTerminal, Write, stdout};
fn main() {
    let mut args = args();
    let Some(base) = args.nth(1).and_then(|s| s.parse::<u8>().ok()) else {
        return;
    };
    let Some(n) = args.next().and_then(|s| s.parse::<u32>().ok()) else {
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
