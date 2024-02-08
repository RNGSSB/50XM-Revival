use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*},
        lib::lua_const::*
    },
    smash_script::*
};

unsafe extern "C" fn game_aircatchlanding(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 0.88);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_SAMUS_INSTANCE_WORK_ID_FLAG_ARTICLE_MOTION_RATE_SYNC);
    }
}


pub fn install(fighter: &mut smashline::Agent) {
    fighter.game_acmd("game_aircatchlanding", game_aircatchlanding);
    //fighter.effect_acmd("effect_attackairf", effect_attackairf);
    //fighter.sound_acmd("sound_attackairf", sound_attackairf);
}