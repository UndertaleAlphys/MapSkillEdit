// Currently needed because we use these functionality, they'll be removable when the Rust language stabilizes them
#![feature(lazy_cell, ptr_sub_ptr)]

use std::{f32::consts::PI, num::NonZero};

use engage::gamedata::{skill::SkillData, unit::Unit};
use skyline::hooks::InlineCtx;
use unity::prelude::OptionalMethod;
// / This is called a proc(edural) macro. You use this to indicate that a function will be used as a hook.
// /
// / Pay attention to the argument, offset.
// / This is the address of the start of the function you would like to hook.
// / This address has to be relative to the .text section of the game.
// / If you do not know what any of this means, take the address in Ghidra and remove the starting ``71`` and the zeroes that follow it.
// / Do not forget the 0x indicator, as it denotates that you are providing a hexadecimal value.

/*
Analysis
MapSkill_Prediction
x24: *MapSkillResults
got the registers we want in ClassChangeCheck().
*/

#[unity::class("App", "MapSkill_Result")]
pub struct MapSkillResult {
    pub moved: bool,
    pub unit: *mut Unit,
    pub x: i32,
    pub z: i32,
}
trait MapSkillResultTrait {
    fn set_moved(&mut self, moved: bool);
    fn get_moved(&self) -> bool;
    fn set_unit(&mut self, unit: Option<&Unit>);
    fn get_unit(&self) -> Option<&Unit>;
    fn set_x(&mut self, x: i32);
    fn get_x(&self) -> i32;
    fn set_z(&mut self, z: i32);
    fn get_z(&self) -> i32;
}

impl MapSkillResultTrait for MapSkillResult {
    fn set_moved(&mut self, moved: bool) {
        let p_moved = self as *mut MapSkillResult as *mut bool;
        unsafe { *p_moved = moved }
    }
    fn get_moved(&self) -> bool {
        let p_moved = self as *const MapSkillResult as *const bool;
        unsafe { *p_moved }
    }
    fn set_unit(&mut self, unit: Option<&Unit>) {
        if let Some(unit) = unit {
            let pp_unit = self as *mut MapSkillResult as *mut *const Unit;
            unsafe { *pp_unit.byte_add(0x8) = unit as *const Unit }
        }
    }
    fn get_unit(&self) -> Option<&Unit> {
        let pp_unit = self as *const MapSkillResult as *const *const Unit;
        let p_unit = unsafe { *pp_unit.byte_add(0x8) };
        if p_unit.is_null() {
            None
        } else {
            unsafe { Some(&*p_unit) }
        }
    }
    fn set_x(&mut self, x: i32) {
        let p_x = self as *mut MapSkillResult as *mut i32;
        unsafe { *p_x.byte_add(0x10) = x }
    }
    fn get_x(&self) -> i32 {
        let p_x = self as *const MapSkillResult as *const i32;
        unsafe { *p_x.byte_add(0x10) }
    }
    fn set_z(&mut self, z: i32) {
        let p_z = self as *mut MapSkillResult as *mut i32;
        unsafe { *p_z.byte_add(0x14) = z }
    }
    fn get_z(&self) -> i32 {
        let p_z = self as *const MapSkillResult as *const i32;
        unsafe { *p_z.byte_add(0x14) }
    }
}

#[unity::class("App", "MapSkill_Results")]
pub struct MapSkillResults {
    pub skill: *mut SkillData,
    pub current: *mut MapSkillResult,
    pub reverse: *mut MapSkillResult,
}

trait MapSkillResultsTrait {
    fn set_skill(&mut self, skill: &SkillData);
    fn get_skill(&self) -> Option<&SkillData>;
    fn set_current(&mut self, current: &MapSkillResult);
    fn get_current(&self) -> &MapSkillResult;
    fn set_reverse(&mut self, current: &MapSkillResult);
    fn get_reverse(&self) -> &MapSkillResult;
}

impl MapSkillResultsTrait for MapSkillResults {
    fn set_skill(&mut self, skill: &SkillData) {
        let pp_skill = self as *mut MapSkillResults as *mut *const SkillData;
        unsafe { *pp_skill = skill as *const SkillData }
    }
    fn get_skill(&self) -> Option<&SkillData> {
        let pp_skill = self as *const MapSkillResults as *const *const SkillData;
        let p_skill = unsafe { *pp_skill };
        if p_skill.is_null() {
            None
        } else {
            unsafe { Some(&*p_skill) }
        }
    }
    fn set_current(&mut self, current: &MapSkillResult) {
        let mut p_result = self as *mut MapSkillResults as *mut MapSkillResult;
        unsafe {
            p_result = p_result.byte_add(0x8);
            let p_moved = p_result as *mut bool;
            *p_moved = current.get_moved();
            let p_unit = p_result.byte_add(0x8) as *mut *const Unit;
            *p_unit = match current.get_unit() {
                Some(unit) => unit as *const Unit,
                None => std::ptr::null(),
            };
            let p_x = p_result.byte_add(0x10) as *mut i32;
            *p_x = current.get_x();
            let p_z = p_result.byte_add(0x14) as *mut i32;
            *p_z = current.get_z();
        }
    }
    fn get_current(&self) -> &MapSkillResult {
        let p_result = self as *const MapSkillResults as *const MapSkillResult;
        unsafe { &*p_result.byte_add(0x8) }
    }

    fn set_reverse(&mut self, reverse: &MapSkillResult) {
        let mut p_result = self as *mut MapSkillResults as *mut MapSkillResult;
        unsafe {
            p_result = p_result.byte_add(0x20);
            let p_moved = p_result as *mut bool;
            *p_moved = reverse.get_moved();
            let p_unit = p_result.byte_add(0x8) as *mut *const Unit;
            *p_unit = match reverse.get_unit() {
                Some(unit) => unit as *const Unit,
                None => std::ptr::null(),
            };
            let p_x = p_result.byte_add(0x10) as *mut i32;
            *p_x = reverse.get_x();
            let p_z = p_result.byte_add(0x14) as *mut i32;
            *p_z = reverse.get_z();
        }
    }
    fn get_reverse(&self) -> &MapSkillResult {
        let p_result = self as *const MapSkillResults as *const MapSkillResult;
        unsafe { &*p_result.byte_add(0x20) }
    }
}

unsafe fn map_skill_prediction_get_results(ctx: &InlineCtx) -> &MapSkillResults {
    unsafe { &*(*ctx.registers[24].x.as_ref() as *const MapSkillResults) }
}

fn print_unit_status(unit: &Unit) {
    println!(
        "Name: {}",
        unit.get_person()
            .get_name()
            .unwrap_or("None".into())
            .to_string()
    );
    println!("X: {}; Z: {};", unit.get_x(), unit.get_z());
}

#[skyline::hook(offset = 0x1F4E160)]
pub fn map_skill_prediction(
    current: &mut Unit,
    reverse: &mut Unit,
    skill: &SkillData,
    results: &mut MapSkillResults,
    method: OptionalMethod,
) -> bool {
    let result = call_original!(current, reverse, skill, results, method);
    println!("Get current_result");
    let current_r = results.get_current();
    println!("Get current_unit");
    let current_u = current_r.get_unit();
    println!("Unwrap");
    if current_u.is_some() {
        println!("Is not none");
        print_unit_status(current_u.unwrap());
    }
    // print_unit_status(results.get_reverse().unwrap().get_unit().unwrap());
    result
}

/// The internal name of your plugin. This will show up in crash logs. Make it 8 characters long at max.
#[skyline::main(name = "MpSklEdt")]
pub fn main() {
    // Install a panic handler for your plugin, allowing you to customize what to do if there's an issue in your code.
    std::panic::set_hook(Box::new(|info| {
        let location = info.location().unwrap();

        // Some magic thing to turn what was provided to the panic into a string. Don't mind it too much.
        // The message will be stored in the msg variable for you to use.
        let msg = match info.payload().downcast_ref::<&'static str>() {
            Some(s) => *s,
            None => match info.payload().downcast_ref::<String>() {
                Some(s) => &s[..],
                None => "Box<Any>",
            },
        };

        // This creates a new String with a message of your choice, writing the location of the panic and its message inside of it.
        // Note the \0 at the end. This is needed because show_error is a C function and expects a C string.
        // This is actually just a result of bad old code and shouldn't be necessary most of the time.
        let err_msg = format!(
            "Custom plugin has panicked at '{}' with the following message:\n{}\0",
            location, msg
        );

        // We call the native Error dialog of the Nintendo Switch with this convenient method.
        // The error code is set to 69 because we do need a value, while the first message displays in the popup and the second shows up when pressing Details.
        skyline::error::show_error(
            69,
            "Custom plugin has panicked! Please open the details and send a screenshot to the developer, then close the game.\n\0",
            err_msg.as_str(),
        );
    }));

    // This is what you call to install your hook(s).
    // If you do not install your hook(s), they will just not execute and nothing will be done with them.
    // It is common to install then in ``main`` but nothing stops you from only installing a hook if some conditions are fulfilled.
    // Do keep in mind that hooks cannot currently be uninstalled, so proceed accordingly.
    //
    // A ``install_hooks!`` variant exists to let you install multiple hooks at once if separated by a comma.
    skyline::install_hooks!(map_skill_prediction);
}
