use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*, *},
        lib::lua_const::*
    },
    smash_script::*
};

unsafe extern "C" fn game_throwhi(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 2.0, 90, 120, 0, 75, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::CHECK_FINISH_CAMERA(agent, 2, 20);
        lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.6);
        lua_bind::FighterCutInManager::set_throw_finish_offset(singletons::FighterCutInManager(), Vector3f{x: 0.0, y: 4.0, z: 0.0});
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        let target = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        macros::ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::IS_EXIST_ARTICLE(agent, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER) {
        if macros::is_excute(agent) {
            ArticleModule::change_motion(agent.module_accessor, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER, Hash40::new("open"), false, -1.0);
        }
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER_BULLET, false, -1);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER_BULLET, false, -1);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER_BULLET, false, -1);
    }
    frame(agent.lua_state_agent, 27.0);
    if macros::IS_EXIST_ARTICLE(agent, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER) {
        if macros::is_excute(agent) {
            ArticleModule::change_motion(agent.module_accessor, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER, Hash40::new("close"), false, -1.0);
        }
    }
    frame(agent.lua_state_agent, 39.0);
    if macros::is_excute(agent) {
            CancelModule::enable_cancel(agent.module_accessor); 
    }
}

pub fn install(fighter: &mut smashline::Agent) {
    fighter.game_acmd("game_throwhi", game_throwhi);
}