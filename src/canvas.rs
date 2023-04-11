use std::{marker::PhantomData, mem::size_of};

use olivec_sys::{
    olivec_canvas, olivec_circle, olivec_ellipse, olivec_fill, olivec_frame, olivec_line,
    olivec_pixel_bilinear, olivec_rect, olivec_sprite_blend, olivec_sprite_copy,
    olivec_sprite_copy_bilinear, olivec_subcanvas, olivec_text, olivec_triangle, olivec_triangle3c,
    olivec_triangle3uv, olivec_triangle3uv_bilinear, olivec_triangle3z, size_t, Olivec_Canvas,
};

use crate::{normalize_rect, Font, NormalizedRect};

/// Abstraction over the Olivec_Canvas struct which provides the necessary methods to mutate it,
/// and owns the underlying allocated pixel buffer
pub struct Canvas {
    /// Olivec_Canvas stores a pointer to this pixel buffer, so it is important that
    /// OwnedCanvas owns the underlying memory, and handles it properly to prevent UAF errors
    _pixels: Vec<u32>,
    canvas: Olivec_Canvas,
}

impl Canvas {
    /// Creates a new Canvas and allocates a pixel buffer for it
    pub fn new(width: usize, height: usize) -> Self {
        let pixels = vec![0u32; width * height];

        Self::from_buffer(pixels, width, height).unwrap()
    }

    /// Creates a new canvas from a preallocated pixel buffer
    ///
    /// Returns `None` if the provided pixel buffer is too small
    /// i.e. the capacity is less than `width * height`
    pub fn from_buffer(mut pixels: Vec<u32>, width: usize, height: usize) -> Option<Self> {
        if pixels.capacity() < width * height {
            return None;
        }

        let canvas = unsafe {
            olivec_canvas(
                pixels.as_mut_ptr(),
                width as size_t,
                height as size_t,
                width as size_t,
            )
        };

        Some(Self {
            _pixels: pixels,
            canvas,
        })
    }
}

/// Represents a subspace of a Canvas object
///
/// Modifying the SubCanvas will mutate the original Canvas object as internally
/// it stores a pointer to the same memory. As such, the parent canvas must live
/// at least as long as the subcanvas
pub struct SubCanvas<'a> {
    parent: PhantomData<&'a Canvas>,
    canvas: Olivec_Canvas,
}

/* Allows both Canvas and SubCanvas to share the same API */

pub trait ToOlivecCanvas {
    fn get_olivec_canvas(&self) -> Olivec_Canvas;
}

impl ToOlivecCanvas for Canvas {
    #[inline]
    fn get_olivec_canvas(&self) -> Olivec_Canvas {
        self.canvas
    }
}

impl<'a> ToOlivecCanvas for SubCanvas<'a> {
    #[inline]
    fn get_olivec_canvas(&self) -> Olivec_Canvas {
        self.canvas
    }
}

/// A common interface for anything that stores a reference to an Olivec_Canvas
///
/// Many of the methods are safe wrappers for the olivec c functions, but there
/// are also additional helper methods that make using the Canvas more convenient
pub trait Drawable {
    /* Original Interface */

    /// Creates a subspace of the original canvas
    ///
    /// Modifying the SubCanvas will mutate the original Canvas object as internally
    /// it stores a pointer to the same memory. As such, the parent canvas must live
    /// at least as long as the subcanvas
    ///
    /// Returns `None` if the provided dimensions are completely outside the bounds
    /// of the canvas or if the created subcanvas would have a size of 0
    fn subcanvas<'a>(&mut self, x: i32, y: i32, w: i32, h: i32) -> Option<SubCanvas<'a>>;

    /// Fills the canvas withe the provided color
    fn fill(&mut self, color: u32);

    /// Draws a rectangle at the provided coordinates with a width of `w`, and height of `h`
    /// using the provided color
    fn rect(&mut self, x: i32, y: i32, w: i32, h: i32, color: u32);

    /// Draws a rectangular outline with the provided thiccness and color
    fn frame(&mut self, x: i32, y: i32, w: i32, h: i32, thiccness: usize, color: u32);

    /// Draws a circle with its center at `(cx, cy)` and a radius `r`
    fn circle(&mut self, cx: i32, cy: i32, r: i32, color: u32);

    /// Draws an ellipse with its center at `(cx, cy)` and radii `rx` and `ry`
    fn ellipse(&mut self, cx: i32, cy: i32, rx: i32, ry: i32, color: u32);

    /// Draws a line starting at `(x1, y1) and ending at `(x2, y2)`
    fn line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, color: u32);

    /// Draws a triangle bound by the points `(x1, y1)`, `(x2, y2)`, and `(x3, y3)`
    fn triangle(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32, color: u32);

    fn triangle3c(
        &mut self,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        x3: i32,
        y3: i32,
        c1: u32,
        c2: u32,
        c3: u32,
    );
    fn triangle3z(
        &mut self,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        x3: i32,
        y3: i32,
        z1: f32,
        z2: f32,
        z3: f32,
    );
    fn triangle3uv(
        &mut self,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        x3: i32,
        y3: i32,
        tx1: f32,
        ty1: f32,
        tx2: f32,
        ty2: f32,
        tx3: f32,
        ty3: f32,
        z1: f32,
        z2: f32,
        z3: f32,
        texture: &mut impl ToOlivecCanvas,
    );
    fn triangle3uv_bilinear(
        &mut self,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        x3: i32,
        y3: i32,
        tx1: f32,
        ty1: f32,
        tx2: f32,
        ty2: f32,
        tx3: f32,
        ty3: f32,
        z1: f32,
        z2: f32,
        z3: f32,
        texture: &mut impl ToOlivecCanvas,
    );
    fn text<'a, T: Into<&'a str>, const W: usize, const H: usize>(
        &mut self,
        text: T,
        x: i32,
        y: i32,
        font: Font<W, H>,
        size: usize,
        color: u32,
    );
    fn sprite_blend(&mut self, x: i32, y: i32, w: i32, h: i32, sprite: &mut impl ToOlivecCanvas);
    fn sprite_copy(&mut self, x: i32, y: i32, w: i32, h: i32, sprite: &mut impl ToOlivecCanvas);
    fn sprite_copy_bilinear(
        &mut self,
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        sprite: &mut impl ToOlivecCanvas,
    );
    fn pixel_bilinear(&mut self, nx: i32, ny: i32, w: i32, h: i32) -> u32;

    /* Additional Helpers */

    /// Sets an individual pixel in the canvas to the given color
    ///
    /// This function does a bounds check to make sure that the provided coordinates
    /// are within the bounds of the canvas to avoid accessing invalid memory
    ///
    /// ```
    /// use olivec::{ Canvas, Drawable, rgba };
    ///
    /// let mut canvas = Canvas::new(100, 100);
    ///
    /// canvas.set_pixel(20, 20, rgba!(69, 69, 69, 255));
    /// ```
    fn set_pixel(&mut self, x: i32, y: i32, color: u32);

    /// The point of this function is to produce two ranges `x1..=x2` and `y1..=y2` that
    /// are guaranteed to be safe to iterate over the canvas of size canvas_width
    /// by canvas_height without any boundary checks.
    ///
    /// ```
    /// use olivec::{ Canvas, Drawable };
    ///
    /// let mut canvas = Canvas::new(100, 100);
    ///
    /// if let Some(nr) = canvas.normalize_rect(20, 30, 90, 80) {
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
    fn normalize_rect(&self, x: i32, y: i32, w: i32, h: i32) -> Option<NormalizedRect>;

    /// Returns the width of the underlying canvas
    fn get_width(&self) -> usize;

    /// Returns the height of the underlying canvas
    fn get_height(&self) -> usize;

    /// Gets a slice which contains the bytes of the underlying pixel buffer
    fn get_data(&self) -> &[u8];
}

impl<T: ToOlivecCanvas> Drawable for T {
    #[inline]
    fn subcanvas<'a>(&mut self, x: i32, y: i32, w: i32, h: i32) -> Option<SubCanvas<'a>> {
        let canvas = unsafe { olivec_subcanvas(self.get_olivec_canvas(), x, y, w, h) };

        // olivec_subcanvas returned OLIVEC_CANVAS_NULL
        if canvas.height == 0 && canvas.width == 0 {
            None
        } else {
            Some(SubCanvas {
                parent: PhantomData,
                canvas,
            })
        }
    }

    #[inline]
    fn fill(&mut self, color: u32) {
        unsafe { olivec_fill(self.get_olivec_canvas(), color) }
    }

    #[inline]
    fn rect(&mut self, x: i32, y: i32, w: i32, h: i32, color: u32) {
        unsafe { olivec_rect(self.get_olivec_canvas(), x, y, w, h, color) }
    }

    #[inline]
    fn frame(&mut self, x: i32, y: i32, w: i32, h: i32, thiccness: usize, color: u32) {
        unsafe {
            olivec_frame(
                self.get_olivec_canvas(),
                x,
                y,
                w,
                h,
                thiccness as size_t,
                color,
            )
        }
    }

    #[inline]
    fn circle(&mut self, cx: i32, cy: i32, r: i32, color: u32) {
        unsafe { olivec_circle(self.get_olivec_canvas(), cx, cy, r, color) }
    }

    #[inline]
    fn ellipse(&mut self, cx: i32, cy: i32, rx: i32, ry: i32, color: u32) {
        unsafe { olivec_ellipse(self.get_olivec_canvas(), cx, cy, rx, ry, color) }
    }

    #[inline]
    fn line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, color: u32) {
        unsafe { olivec_line(self.get_olivec_canvas(), x1, y1, x2, y2, color) }
    }

    #[inline]
    fn triangle(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32, color: u32) {
        unsafe { olivec_triangle(self.get_olivec_canvas(), x1, y1, x2, y2, x3, y3, color) }
    }

    #[inline]
    fn triangle3c(
        &mut self,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        x3: i32,
        y3: i32,
        c1: u32,
        c2: u32,
        c3: u32,
    ) {
        unsafe { olivec_triangle3c(self.get_olivec_canvas(), x1, y1, x2, y2, x3, y3, c1, c2, c3) }
    }

    #[inline]
    fn triangle3z(
        &mut self,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        x3: i32,
        y3: i32,
        z1: f32,
        z2: f32,
        z3: f32,
    ) {
        unsafe { olivec_triangle3z(self.get_olivec_canvas(), x1, y1, x2, y2, x3, y3, z1, z2, z3) }
    }

    #[inline]
    fn triangle3uv(
        &mut self,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        x3: i32,
        y3: i32,
        tx1: f32,
        ty1: f32,
        tx2: f32,
        ty2: f32,
        tx3: f32,
        ty3: f32,
        z1: f32,
        z2: f32,
        z3: f32,
        texture: &mut impl ToOlivecCanvas,
    ) {
        unsafe {
            olivec_triangle3uv(
                self.get_olivec_canvas(),
                x1,
                y1,
                x2,
                y2,
                x3,
                y3,
                tx1,
                ty1,
                tx2,
                ty2,
                tx3,
                ty3,
                z1,
                z2,
                z3,
                texture.get_olivec_canvas(),
            )
        }
    }

    #[inline]
    fn triangle3uv_bilinear(
        &mut self,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        x3: i32,
        y3: i32,
        tx1: f32,
        ty1: f32,
        tx2: f32,
        ty2: f32,
        tx3: f32,
        ty3: f32,
        z1: f32,
        z2: f32,
        z3: f32,
        texture: &mut impl ToOlivecCanvas,
    ) {
        unsafe {
            olivec_triangle3uv_bilinear(
                self.get_olivec_canvas(),
                x1,
                y1,
                x2,
                y2,
                x3,
                y3,
                tx1,
                ty1,
                tx2,
                ty2,
                tx3,
                ty3,
                z1,
                z2,
                z3,
                texture.get_olivec_canvas(),
            )
        }
    }

    #[inline]
    fn text<'a, U: Into<&'a str>, const W: usize, const H: usize>(
        &mut self,
        text: U,
        x: i32,
        y: i32,
        font: Font<W, H>,
        size: usize,
        color: u32,
    ) {
        let string: &str = text.into();

        unsafe {
            olivec_text(
                self.get_olivec_canvas(),
                string.as_ptr() as *const i8,
                x,
                y,
                font.font,
                size as size_t,
                color,
            )
        }
    }

    #[inline]
    fn sprite_blend(&mut self, x: i32, y: i32, w: i32, h: i32, sprite: &mut impl ToOlivecCanvas) {
        unsafe {
            olivec_sprite_blend(
                self.get_olivec_canvas(),
                x,
                y,
                w,
                h,
                sprite.get_olivec_canvas(),
            )
        }
    }

    #[inline]
    fn sprite_copy(&mut self, x: i32, y: i32, w: i32, h: i32, sprite: &mut impl ToOlivecCanvas) {
        unsafe {
            olivec_sprite_copy(
                self.get_olivec_canvas(),
                x,
                y,
                w,
                h,
                sprite.get_olivec_canvas(),
            )
        }
    }

    #[inline]
    fn sprite_copy_bilinear(
        &mut self,
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        sprite: &mut impl ToOlivecCanvas,
    ) {
        unsafe {
            olivec_sprite_copy_bilinear(
                self.get_olivec_canvas(),
                x,
                y,
                w,
                h,
                sprite.get_olivec_canvas(),
            )
        }
    }

    fn pixel_bilinear(&mut self, nx: i32, ny: i32, w: i32, h: i32) -> u32 {
        unsafe { olivec_pixel_bilinear(self.get_olivec_canvas(), nx, ny, w, h) }
    }

    #[inline]
    fn set_pixel(&mut self, x: i32, y: i32, color: u32) {
        let oc = self.get_olivec_canvas();

        if x > 0 && x < oc.width as i32 && y > 0 && y < oc.height as i32 {
            // Safety: As long as coords are within the canvas width and height,
            // the following address calculation should be a safe index into the
            // allocated pixel buffer
            unsafe {
                *oc.pixels.offset((y * oc.stride as i32 + x) as isize) = color;
            }
        }
    }

    #[inline]
    fn normalize_rect(&self, x: i32, y: i32, w: i32, h: i32) -> Option<NormalizedRect> {
        normalize_rect(x, y, w, h, self.get_width(), self.get_height())
    }

    #[inline]
    fn get_width(&self) -> usize {
        self.get_olivec_canvas().width as usize
    }

    #[inline]
    fn get_height(&self) -> usize {
        self.get_olivec_canvas().height as usize
    }

    #[inline]
    fn get_data(&self) -> &[u8] {
        let oc = self.get_olivec_canvas();

        unsafe {
            std::slice::from_raw_parts(
                oc.pixels as *const u8,
                size_of::<u32>() * self.get_width() * self.get_height(),
            )
        }
    }
}
