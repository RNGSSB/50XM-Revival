use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*, *},
        lib::lua_const::*
    },
    smash_script::*
};

unsafe extern "C" fn game_specialn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_MARIO_GENERATE_ARTICLE_FIREBALL, false, -1);
    }
    frame(agent.lua_state_agent, 41.0);
    if macros::is_excute(agent) {
            CancelModule::enable_cancel(agent.module_accessor);
    }
}

unsafe extern "C" fn game_specialairn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_MARIO_GENERATE_ARTICLE_FIREBALL, false, -1);
    }
    frame(agent.lua_state_agent, 41.0);
    if macros::is_excute(agent) {
            CancelModule::enable_cancel(agent.module_accessor);
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.game_acmd("game_specialn", game_specialn);
    agent.game_acmd("game_specialairn", game_specialairn);
}