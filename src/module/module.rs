use crate::sauerbraten::structs::*;
use winapi::ctypes;

pub trait Module {
    fn name(&self) -> &str;
    fn toggled(&self) -> bool;
    fn category(&self) -> &Category;
    fn hotkey(&self) -> ctypes::c_int;
    fn toggle(&mut self);
    fn on_enable(&self);
    fn on_disable(&self);
    unsafe fn on_update(&self, ctx: &mut Context);
}

pub struct Context {
    pub local_entity: *mut Entity,
    pub entity_list_ptr: *mut u64,
    pub entity_list_count_ptr: *mut i32,
    pub game_mode_ptr: *mut i32,
    pub in_game: *mut bool,
}

pub enum Category {
    Combat
}