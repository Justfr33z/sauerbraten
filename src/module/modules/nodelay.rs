use winapi::ctypes;
use crate::module::module::{
    Category,
    Module,
    Context
};

pub struct NoDelay<'a> {
    pub name: &'a str,
    pub toggled: bool,
    pub category: &'a Category,
    pub hotkey: ctypes::c_int,
}

impl Module for NoDelay<'_> {
    fn name(&self) -> &str {
        self.name
    }

    fn toggled(&self) -> bool {
        self.toggled
    }

    fn category(&self) -> &Category {
        self.category
    }

    fn hotkey(&self) -> i32 {
        self.hotkey
    }

    fn toggle(&mut self) {
        self.toggled = !self.toggled;

        if self.toggled {
            self.on_enable();
        } else {
            self.on_disable();
        }
    }

    fn on_enable(&self) {
        println!("{} enabled", self.name);
    }

    fn on_disable(&self) {
        println!("{} disabled", self.name)
    }

    unsafe fn on_update(&self, ctx: &mut Context) {
        if !*ctx.in_game || ctx.local_entity.is_null() || (*ctx.local_entity).is_dead {
            return
        }

        (*ctx.local_entity).delay = 0;
    }
}