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

pub unsafe extern "C" fn attack_air_n_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub unsafe extern "C" fn attack_air_f_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(Init, *FIGHTER_TRAIL_STATUS_KIND_ATTACK_AIR_N, attack_air_n_init);
    agent.status(Init, *FIGHTER_TRAIL_STATUS_KIND_ATTACK_AIR_F, attack_air_f_init);
}