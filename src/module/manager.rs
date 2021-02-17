use crate::sauerbraten::structs::*;
use crate::module::{
    modules::{
        aimbot::Aimbot,
        nodelay::NoDelay,
    },
    module::{
        Module,
        Context,
        Category,
    },
};
use winapi::{
    um::{
        libloaderapi,
        winuser,
    },
    _core::ptr,
};


pub struct ModuleManager {
    pub modules: Vec<Box<dyn Module>>,
}

impl ModuleManager {
    pub fn start(&mut self) {
        self.register_modules();
        
        let mut ctx = unsafe {
            let module_base_addr = libloaderapi::GetModuleHandleA(ptr::null()) as u64;

            Context {
                local_entity: *((module_base_addr + 0x002A3528) as *mut *mut Entity),
                entity_list_ptr: (module_base_addr + 0x346C90) as *mut u64,
                entity_list_count_ptr: (module_base_addr + 0x3472EC) as *mut i32,
                game_mode_ptr: (module_base_addr + 0x2A5D38) as *mut i32,
                in_game: (module_base_addr + 0x2A2498) as *mut bool,
            }
        };

        unsafe {
            loop {
                for module in self.modules.iter_mut() {
                    if (winuser::GetAsyncKeyState(module.hotkey()) & 1) != 0 {
                        module.toggle();
                    }

                    if module.toggled() {
                        module.on_update(&mut ctx);
                    }
                }
            }
        }
    }

    fn register_modules(&mut self) {
        self.modules = vec![
            Box::new(Aimbot {
                name: "Aimbot",
                toggled: false,
                category: &Category::Combat,
                hotkey: 0x52,
            }),
            Box::new(NoDelay {
                name: "NoDelay",
                toggled: false,
                category: &Category::Combat,
                hotkey: 0x2E
            }),
        ];
    }
}