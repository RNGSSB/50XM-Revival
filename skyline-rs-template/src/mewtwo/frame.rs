use {
    smash::{
        lua2cpp::*,
        app::lua_bind::*,
        lib::lua_const::*
    },
    crate::custom::global_fighter_frame
};

use smash::app::utility::get_kind;
use smash::phx::Hash40;
use smash_script::macros;

unsafe extern "C" fn tail_hurtbox(fighter: &mut L2CAgentBase) {
        
}

// What used to be known as a "Once-Per-Fighter-Frame". On-Line functions can be set to run on any status condition.
unsafe extern "C" fn char_on_main(fighter: &mut L2CFighterCommon) {
    unsafe {
        global_fighter_frame(fighter);

        let module_accessor = &mut *fighter.module_accessor;
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let fighter_kind = get_kind(module_accessor);
        let situation_kind = StatusModule::situation_kind(module_accessor);
        let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);

        if ![*FIGHTER_STATUS_KIND_ATTACK_S3, *FIGHTER_STATUS_KIND_ATTACK_LW3, 
        *FIGHTER_STATUS_KIND_ATTACK_HI3, *FIGHTER_STATUS_KIND_ATTACK_AIR].contains(&status_kind){
            macros::HIT_NODE(fighter, Hash40::new("s_tail1"), *HIT_STATUS_XLU);
            macros::HIT_NODE(fighter, Hash40::new("s_tail2"), *HIT_STATUS_XLU);
            macros::HIT_NODE(fighter, Hash40::new("s_tail3"), *HIT_STATUS_XLU);
            macros::HIT_NODE(fighter, Hash40::new("s_tail4"), *HIT_STATUS_XLU);
            macros::HIT_NODE(fighter, Hash40::new("s_tail5"), *HIT_STATUS_XLU);
            macros::HIT_NODE(fighter, Hash40::new("s_tail6"), *HIT_STATUS_XLU);
            macros::HIT_NODE(fighter, Hash40::new("s_tail7"), *HIT_STATUS_XLU);
        }
        else{
            macros::HIT_NODE(fighter, Hash40::new("s_tail1"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("s_tail2"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("s_tail3"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("s_tail4"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("s_tail5"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("s_tail6"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("s_tail7"), *HIT_STATUS_NORMAL);

        }
        
        
        
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_line(smashline::Main, char_on_main);
}