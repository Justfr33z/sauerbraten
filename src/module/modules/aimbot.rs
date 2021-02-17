use crate::sauerbraten::structs::*;
use winapi::ctypes;
use crate::module::module::{
    Category,
    Module,
    Context
};

pub struct Aimbot<'a> {
    pub name: &'a str,
    pub toggled: bool,
    pub category: &'a Category,
    pub hotkey: ctypes::c_int,
}

impl Module for Aimbot<'_> {
    fn name(&self) -> &str {
        self.name
    }

    fn toggled(&self) -> bool {
        self.toggled
    }

    fn category(&self) -> &Category {
        &self.category
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
        println!("{} enabled", self.name)
    }

    fn on_disable(&self) {
        println!("{} disabled", self.name)
    }

    unsafe fn on_update(&self, ctx: &mut Context) {
        if !*ctx.in_game || ctx.local_entity.is_null() || (*ctx.local_entity).is_dead {
            return
        }

        let mut target: Option<*mut Entity> = None;
        let mut lowest_coefficient = f32::MAX;

        for i in 1..*ctx.entity_list_count_ptr {
            let entity = *((*ctx.entity_list_ptr + (i as u64 * 0x08)) as *mut *mut Entity);

            if entity.is_null() || (*entity).is_dead {
                continue
            }

            let angle_to = (*ctx.local_entity).pos
                .angle(&(*entity).pos)
                .distance(&(*ctx.local_entity).angle);

            let distance = (*ctx.local_entity).pos
                .distance(&(*entity).pos);

            let coefficient = distance * 0.3 + angle_to * 0.7;

            if coefficient < lowest_coefficient {
                lowest_coefficient = coefficient;
                target = Some(entity);
            }
        }

        if target.is_some() {
            (*ctx.local_entity).angle =
                (*ctx.local_entity).pos
                    .angle(&(*target.unwrap()).pos);
        }
    }
}