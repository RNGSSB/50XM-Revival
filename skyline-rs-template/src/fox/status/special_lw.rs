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

// FIGHTER_STATUS_KIND_SPECIAL_LW

unsafe extern "C" fn special_lw_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::situation_kind(fighter.module_accessor) == SITUATION_KIND_GROUND {
        // Returning here allows for running shine
        return 0.into();
    }
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_FOX_INSTANCE_WORK_ID_FLAG_REFLECTOR_LANDING) {
        sv_kinetic_energy!(
            set_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            0.0
        );
        sv_kinetic_energy!(
            set_accel,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            0.0
        );
    }
    let original = smashline::original_status(smashline::Main, fighter, *FIGHTER_STATUS_KIND_SPECIAL_LW);
    original(fighter)
}


pub fn install(agent: &mut smashline::Agent) {
    agent.status(smashline::Main, *FIGHTER_STATUS_KIND_SPECIAL_LW, special_lw_init);
}