const BITMAPCOL_B_SHIFT: u8 = 0;
const BITMAPCOL_G_SHIFT: u8 = 8;
const BITMAPCOL_R_SHIFT: u8 = 16;
const BITMAPCOL_A_SHIFT: u8 = 24;

pub fn bitmap_col_r(col: u32) -> u8 {
    (col >> BITMAPCOL_R_SHIFT) as u8
}
pub fn bitmap_col_g(col: u32) -> u8 {
    (col >> BITMAPCOL_G_SHIFT) as u8
}
pub fn bitmap_col_b(col: u32) -> u8 {
    (col >> BITMAPCOL_B_SHIFT) as u8
}
pub fn bitmap_col_a(col: u32) -> u8 {
    (col >> BITMAPCOL_A_SHIFT) as u8
}
