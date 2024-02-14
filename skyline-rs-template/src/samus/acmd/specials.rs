use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*, *},
        lib::lua_const::*
    },
    smash_script::*,
    smashline::*,
    macros::*
};
use super::*;
use smashline::Pre;
use smash::lib::{L2CValue, L2CAgent};


unsafe extern "C" fn game_special(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_S_WORK_FLAG_WEAPON);
    }
}

unsafe extern "C" fn game_specialair(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_S_WORK_FLAG_WEAPON);
    }
    frame(agent.lua_state_agent, 35.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_S_WORK_FLAG_AIR_CONTROL);
    }
}

pub fn install(fighter: &mut smashline::Agent) {
    fighter.game_acmd("game_special", game_special);
    fighter.game_acmd("game_specialair", game_specialair);
}