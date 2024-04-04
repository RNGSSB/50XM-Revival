use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*},
        lib::lua_const::*
    },
    smash_script::*
};

unsafe extern "C" fn game_escapeairslide(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE_ENABLE_CONTROL);
    }

    frame(agent.lua_state_agent, 42.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

unsafe extern "C" fn effect_escapeairslide(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_flash"), Hash40::new("top"), 0, 12, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
    }
}

unsafe extern "C" fn effect_dash(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -3, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 27.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

pub fn install(fighter: &mut smashline::Agent) {
    fighter.game_acmd("game_escapeairslide", game_escapeairslide);
    fighter.effect_acmd("effect_escapeairslide", effect_escapeairslide);
    fighter.effect_acmd("effect_dash", effect_dash);
}