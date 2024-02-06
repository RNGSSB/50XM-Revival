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


pub fn install(agent: &mut smashline::Agent) {
    //agent.status(smashline::Main, *FIGHTER_STATUS_KIND_SPECIAL_LW, special_lw_init);
}