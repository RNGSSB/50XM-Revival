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

unsafe extern "C" fn game_attackhi4charge(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
    }
}

unsafe extern "C" fn game_attacklw4charge(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.game_acmd("game_attackhi4charge", game_attackhi4charge);
    agent.game_acmd("game_attacklw4charge", game_attacklw4charge);
}
