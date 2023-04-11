// Translated from the example in https://github.com/tsoding/olive.c/blob/master/README.md

use image::{save_buffer, ColorType};
use olivec::{rgb, Canvas, Drawable};

const CANVAS_WIDTH: usize = 900;
const CANVAS_HEIGHT: usize = 600;

fn main() {
    let mut canvas = Canvas::new(CANVAS_WIDTH, CANVAS_HEIGHT);

    // Taken from https://upload.wikimedia.org/wikipedia/en/9/9e/Flag_of_Japan.svg
    canvas.fill(rgb!(0xFF, 0xFF, 0xFF));
    canvas.circle(
        (CANVAS_WIDTH / 2) as i32,
        (CANVAS_HEIGHT / 2) as i32,
        180,
        rgb!(0xBC, 0x00, 0x2D),
    );

    /* Save image buffer to disk */

    save_buffer(
        "flag_jp.png",
        canvas.get_data(),
        canvas.get_width() as u32,
        canvas.get_height() as u32,
        ColorType::Rgba8,
    )
    .expect("could not save image");
}
