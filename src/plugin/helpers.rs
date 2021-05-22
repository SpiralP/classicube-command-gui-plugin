use crate::error::*;
use byteorder::{BigEndian, WriteBytesExt};
use classicube_sys::{
    Bitmap, DrawTextArgs, Drawer2D_DrawText, Drawer2D_MakeFont, Drawer2D_TextHeight,
    Drawer2D_TextWidth, FontDesc, OwnedString, FONT_FLAGS_FONT_FLAGS_NONE,
};
use std::{cell::RefCell, mem};

const BITMAPCOL_B_SHIFT: u8 = 0;
const BITMAPCOL_G_SHIFT: u8 = 8;
const BITMAPCOL_R_SHIFT: u8 = 16;
const BITMAPCOL_A_SHIFT: u8 = 24;

pub fn BitmapCol_R(col: u32) -> u8 {
    (col >> BITMAPCOL_R_SHIFT) as u8
}
pub fn BitmapCol_G(col: u32) -> u8 {
    (col >> BITMAPCOL_G_SHIFT) as u8
}
pub fn BitmapCol_B(col: u32) -> u8 {
    (col >> BITMAPCOL_B_SHIFT) as u8
}
pub fn BitmapCol_A(col: u32) -> u8 {
    (col >> BITMAPCOL_A_SHIFT) as u8
}

thread_local!(
    static FONT: RefCell<FontDesc> = RefCell::new(unsafe {
        let mut font = mem::zeroed();
        Drawer2D_MakeFont(&mut font, 16, FONT_FLAGS_FONT_FLAGS_NONE as _);
        font
    });
);

pub fn make_text_bitmap(text: &str) -> Result<(Vec<u8>, usize, usize)> {
    FONT.with(move |cell| {
        let font = &mut *cell.borrow_mut();

        let s = OwnedString::new(text);
        let cc_string = *s.as_cc_string();

        let mut args = DrawTextArgs {
            text: cc_string,
            font,
            useShadow: 1,
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

        let mut vec8: Vec<u8> = Vec::with_capacity(pixels.len() * 4);
        for n in pixels {
            vec8.push(BitmapCol_R(n));
            vec8.push(BitmapCol_G(n));
            vec8.push(BitmapCol_B(n));
            vec8.push(BitmapCol_A(n));
        }

        Ok((vec8, width, height))
    })
}
