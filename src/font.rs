use olivec_sys::{olivec_default_font, olivec_default_glyphs, size_t, Olivec_Font};

const FONT_GLYPH_COUNT: usize = 128;
const DEFAULT_FONT_WIDTH: usize = 6;
const DEFAULT_FONT_HEIGHT: usize = 6;

pub struct Font<const W: usize, const H: usize> {
    /// Must own the underlying memory that is referenced by the Olivec_Font struct
    _glyphs: [[[i8; W]; H]; FONT_GLYPH_COUNT],
    pub(crate) font: Olivec_Font,
}

impl<const W: usize, const H: usize> Font<W, H> {
    pub fn new(glyphs: [[[i8; W]; H]; FONT_GLYPH_COUNT]) -> Self {
        Self {
            _glyphs: glyphs,
            font: Olivec_Font {
                width: W as size_t,
                height: H as size_t,
                glyphs: glyphs.as_ptr() as *const i8,
            },
        }
    }

    pub fn default() -> Font<DEFAULT_FONT_WIDTH, DEFAULT_FONT_HEIGHT> {
        Font {
            _glyphs: unsafe { olivec_default_glyphs },
            font: unsafe { olivec_default_font },
        }
    }
}
