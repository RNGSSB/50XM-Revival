use {
    smash::{
        lua2cpp::*,
        app::lua_bind::*,
        lib::lua_const::*
    },
    crate::custom::global_fighter_frame
};

use smash::app::utility::get_kind;


pub unsafe fn landCancels(boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, fighter_kind: i32) { //Fox & Falco Lasers
    if [*FIGHTER_KIND_SAMUS].contains(&fighter_kind) {
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S {
            if StatusModule::prev_situation_kind(boma) == *SITUATION_KIND_AIR && situation_kind == *SITUATION_KIND_GROUND {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, true);
            }
        }
    }
}

// What used to be known as a "Once-Per-Fighter-Frame". On-Line functions can be set to run on any status condition.
unsafe extern "C" fn char_on_main(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = &mut *fighter.module_accessor;
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let fighter_kind = get_kind(module_accessor);
        let situation_kind = StatusModule::situation_kind(module_accessor);
        landCancels(module_accessor, status_kind, situation_kind, fighter_kind);
        global_fighter_frame(fighter);
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_line(smashline::Main, char_on_main);
}