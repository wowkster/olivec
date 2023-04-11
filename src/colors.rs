use olivec_sys::olivec_blend_color;

/// Blends the two colors together and returns the result
pub fn blend_color(color1: u32, color2: u32) -> u32 {
    let mut res = color1;

    unsafe {
        olivec_blend_color(&mut res, color2);
    }

    res
}

/// Blends the two colors together and returns the result back in the first argument
///
/// This function more closely matches the original implementation of `olivec_blend_color`
pub fn blend_color_in_place(color1: &mut u32, color2: u32) {
    unsafe {
        olivec_blend_color(color1, color2);
    }
}

/// Macro used to extract the red component of the packed RGBA color
#[macro_export]
macro_rules! red {
    ($color:expr) => {
        (($color & 0x000000FF) >> (8 * 0)) as u8
    };
}

/// Macro used to extract the green component of the packed RGBA color
#[macro_export]
macro_rules! green {
    ($color:expr) => {
        (($color & 0x0000FF00) >> (8 * 1)) as u8
    };
}

/// Macro used to extract the blue component of the packed RGBA color
#[macro_export]
macro_rules! blue {
    ($color:expr) => {
        (($color & 0x00FF0000) >> (8 * 2)) as u8
    };
}

/// Macro used to extract the alpha component of the packed RGBA color
#[macro_export]
macro_rules! alpha {
    ($color:expr) => {
        (($color & 0xFF000000) >> (8 * 3)) as u8
    };
}

/// Macro to pack RGBA values into a u32
#[macro_export]
macro_rules! rgba {
    ($r:expr, $g:expr, $b:expr, $a:expr) => {
        (($r & 0xFF) << (8 * 0))
            | (($g & 0xFF) << (8 * 1))
            | (($b & 0xFF) << (8 * 2))
            | (($a & 0xFF) << (8 * 3))
    };
}

/// Macro to pack RGB values into a u32 with an alpha of 255
#[macro_export]
macro_rules! rgb {
    ($r:expr, $g:expr, $b:expr) => {
        (($r & 0xFF) << (8 * 0))
            | (($g & 0xFF) << (8 * 1))
            | (($b & 0xFF) << (8 * 2))
            | ((0xFF) << (8 * 3))
    };
}
