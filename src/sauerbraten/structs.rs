use crate::utils::math::*;
use winapi::ctypes;

#[repr(C)]
pub struct Entity {
    pad_0000: [ctypes::c_char; 48], //0x0000
    pub pos: Vec3, //0x0030
    pub angle: Vec3, //0x003C
    pad_0048: [ctypes::c_char; 47], //0x0048
    pub is_dead: bool, //0x0077
    pad_0078: [ctypes::c_char; 280], //0x0078
    pub delay: i32, //0x0190
    pad_0194: [ctypes::c_char; 788], //0x0194
} //Size: 0x04A8