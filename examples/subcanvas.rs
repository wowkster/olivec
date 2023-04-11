// Demonstrates the usage of SubCanvas
//
// This example creates a blue box with a red outline
// by first filling in the canvas with red and then filling
// in a subcanvas with blue

use image::{save_buffer, ColorType};
use olivec::{rgb, Canvas, Drawable};

const CANVAS_WIDTH: usize = 900;
const CANVAS_HEIGHT: usize = 600;

fn main() {
    let mut canvas = Canvas::new(CANVAS_WIDTH, CANVAS_HEIGHT);

    // Fill main canvas with red
    canvas.fill(rgb!(255, 0, 0));

    // Define a subspace of the canvas that is inset 20 pixels from each side
    let mut sub = canvas
        .subcanvas(20, 20, CANVAS_WIDTH as i32 - 40, CANVAS_HEIGHT as i32 - 40)
        .unwrap();

    // Fill subcanvas with blue
    sub.fill(rgb!(50, 50, 255));

    /* Save image buffer to disk */

    save_buffer(
        "subcanvas.png",
        canvas.get_data(),
        canvas.get_width() as u32,
        canvas.get_height() as u32,
        ColorType::Rgba8,
    )
    .expect("could not save image");
}
