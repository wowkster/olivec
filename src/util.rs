use olivec_sys::{
    olivec_barycentric, olivec_normalize_rect, olivec_normalize_triangle, size_t,
    Olivec_Normalized_Rect,
};

pub fn barycentric(
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    x3: i32,
    y3: i32,
    xp: i32,
    yp: i32,
    u1: &mut i32,
    u2: &mut i32,
    det: &mut i32,
) -> bool {
    unsafe { olivec_barycentric(x1, y1, x2, y2, x3, y3, xp, yp, u1, u2, det) }
}

pub fn normalize_triangle(
    width: usize,
    height: usize,
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    x3: i32,
    y3: i32,
    lx: &mut i32,
    hx: &mut i32,
    ly: &mut i32,
    hy: &mut i32,
) -> bool {
    unsafe {
        olivec_normalize_triangle(
            width as size_t,
            height as size_t,
            x1,
            y1,
            x2,
            y2,
            x3,
            y3,
            lx,
            hx,
            ly,
            hy,
        )
    }
}

#[derive(Default)]
pub struct NormalizedRect {
    pub x1: i32,
    pub x2: i32,
    pub y1: i32,
    pub y2: i32,
    ox1: i32,
    ox2: i32,
    oy1: i32,
    oy2: i32,
}

impl From<Olivec_Normalized_Rect> for NormalizedRect {
    fn from(value: Olivec_Normalized_Rect) -> Self {
        Self {
            x1: value.x1,
            x2: value.x2,
            y1: value.y1,
            y2: value.y2,
            ox1: value.ox1,
            ox2: value.ox2,
            oy1: value.oy1,
            oy2: value.oy2,
        }
    }
}

impl From<NormalizedRect> for Olivec_Normalized_Rect {
    fn from(value: NormalizedRect) -> Self {
        Self {
            x1: value.x1,
            x2: value.x2,
            y1: value.y1,
            y2: value.y2,
            ox1: value.ox1,
            ox2: value.ox2,
            oy1: value.oy1,
            oy2: value.oy2,
        }
    }
}

/// The point of this function is to produce two ranges `x1..=x2` and `y1..=y2` that
/// are guaranteed to be safe to iterate over the canvas of size canvas_width
/// by canvas_height without any boundary checks.
///
/// ```
/// use olivec::{ Canvas, Drawable, normalize_rect };
///
/// const WIDTH: usize = 100;
/// const HEIGHT: usize = 100;
///
/// let mut canvas = Canvas::new(WIDTH, HEIGHT);
///
/// if let Some(nr) = normalize_rect(20, 30, 90, 80, WIDTH, HEIGHT) {
///     for x in nr.x1..=nr.x2 {
///         for y in nr.y1..=nr.y2 {
///             println!("{x}, {y}");
///             canvas.set_pixel(x, y, 0x69696969);
///         }
///     }
/// } else {
///     // Rectangle is invisible cause it's completely out-of-bounds
/// }
/// ```

pub fn normalize_rect(
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    canvas_width: usize,
    canvas_height: usize,
) -> Option<NormalizedRect> {
    let mut onr = NormalizedRect::default().into();

    unsafe {
        olivec_normalize_rect(
            x,
            y,
            w,
            h,
            canvas_width as size_t,
            canvas_height as size_t,
            &mut onr,
        )
        .then_some(onr.into())
    }
}
