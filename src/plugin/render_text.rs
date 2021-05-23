use crate::{
    error::*,
    plugin::helpers::{bitmap_col_a, bitmap_col_b, bitmap_col_g, bitmap_col_r},
};
use classicube_sys::{
    Bitmap, DrawTextArgs, Drawer2D_DrawText, Drawer2D_MakeFont, Drawer2D_TextHeight,
    Drawer2D_TextWidth, FontDesc, OwnedString, FONT_FLAGS_FONT_FLAGS_NONE,
};
use rayon::prelude::*;
use std::{cell::RefCell, collections::HashMap, mem};

fn make_font(size: u8) -> FontDesc {
    unsafe {
        let mut font = mem::zeroed();
        Drawer2D_MakeFont(&mut font, size as _, FONT_FLAGS_FONT_FLAGS_NONE as _);
        font
    }
}

thread_local!(
    static FONTS: RefCell<HashMap<u8, FontDesc>> = RefCell::new({
        let mut map = HashMap::new();
        map.insert(16, make_font(16));
        map
    });
);

pub fn make_text_bitmap(
    text: &str,
    text_size: u8,
    shadow: bool,
) -> Result<(Vec<u8>, usize, usize)> {
    FONTS.with(move |cell| {
        let fonts = &mut *cell.borrow_mut();
        let font = fonts
            .entry(text_size)
            .or_insert_with(|| make_font(text_size));

        let s = OwnedString::new(text);
        let cc_string = *s.as_cc_string();

        let mut args = DrawTextArgs {
            text: cc_string,
            font,
            useShadow: if shadow { 1 } else { 0 },
        };

        let width = unsafe { Drawer2D_TextWidth(&mut args) } as usize;
        if width == 0 {
            return Err("make_text_bitmap: 0 width".into());
        }
        let height = unsafe { Drawer2D_TextHeight(&mut args) } as usize;

        let mut pixels: Vec<u32> = vec![0x00FFFFFF; width * height];

        let mut bmp = Bitmap {
            scan0: pixels.as_mut_ptr(),
            width: width as i32,
            height: height as i32,
        };

        unsafe { Drawer2D_DrawText(&mut bmp, &mut args, 0, 0) };

        // make sure buffer lasts long enough
        drop(s);

        let vec8 = pixels
            .par_iter()
            .flat_map(|&n| {
                [
                    bitmap_col_r(n),
                    bitmap_col_g(n),
                    bitmap_col_b(n),
                    bitmap_col_a(n),
                ]
            })
            .collect::<Vec<u8>>();

        Ok((vec8, width, height))
    })
}
