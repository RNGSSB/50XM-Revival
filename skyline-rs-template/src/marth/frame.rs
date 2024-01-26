use {
    smash::{
        lua2cpp::*,
        app::lua_bind::*,
        lib::lua_const::*
    },
    crate::custom::global_fighter_frame
};

use smash::app::BattleObjectModuleAccessor;
use smash::app::utility::get_kind;
use smash::phx::*;

pub unsafe fn sword_length(boma: &mut BattleObjectModuleAccessor, fighter_kind: i32) {
    let long_sword_scale = Vector3f{x: 1.015, y: 1.115, z: 1.045};
    if [*FIGHTER_KIND_MARTH].contains(&fighter_kind){
        ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("havel"), &long_sword_scale);
        ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("haver"), &long_sword_scale);
    }
}

// What used to be known as a "Once-Per-Fighter-Frame". On-Line functions can be set to run on any status condition.
unsafe extern "C" fn marth_on_main(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = &mut *fighter.module_accessor;
        let fighter_kind = get_kind(module_accessor);
        
        global_fighter_frame(fighter);
        sword_length(module_accessor, fighter_kind);
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_line(smashline::Main, marth_on_main);
}