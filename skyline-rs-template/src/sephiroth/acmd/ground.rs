use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*, *},
        lib::lua_const::*
    },
    smash_script::*
};

use smash::hash40;


unsafe extern "C" fn game_attack11(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 361, 70, 0, 55, 3.8, 0.0, 7.5, 5.8, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 8.0, 361, 70, 0, 55, 4.8, 0.0, 7.5, 11.4, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 1.1);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 1, 1.1);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
            CancelModule::enable_cancel(agent.module_accessor);
    }
}

unsafe extern "C" fn effect_attack11(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(agent, 1.4);
    }
    if get_value_float(agent.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 4, 10, -11, 2, -12, 0, 0.45, true);
            macros::LAST_EFFECT_SET_ALPHA(agent, 0.5);
        }
        else {
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 4, 10, -12, 2, -12, 0, 0.55, true);
            macros::LAST_EFFECT_SET_ALPHA(agent, 0.8);
            macros::LAST_EFFECT_SET_RATE(agent, 1.2);
        }
    }
}
if macros::is_excute(agent) {
    macros::EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 3, 8, -2, 1.2, -12, 0, 0.7, true);
    macros::EFFECT(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), -0.5, 9, 13, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
    macros::LAST_EFFECT_SET_RATE(agent, 1.8);
    macros::EFFECT_FOLLOW(agent, Hash40::new("edge_attack_line"), Hash40::new("top"), 6.2, 10.5, -20.8, 2, -12, 0, 1.03, true);
    macros::LAST_EFFECT_SET_ALPHA(agent, 1.5);
}
}

unsafe extern "C" fn game_attackdash(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 0.8);
    frame(agent.lua_state_agent, 11.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 13.0, 53, 82, 0, 67, 3.0, 0.0, 8.0, 4.0, Some(0.0), Some(8.0), Some(8.5), 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 13.0, 53, 82, 0, 67, 6.0, 0.0, 8.0, 13.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 361, 82, 0, 66, 2.5, 0.0, 8.0, 4.0, Some(0.0), Some(8.0), Some(8.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 7.0, 361, 82, 0, 66, 4.0, 0.0, 8.0, 13.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 43.0);
    if macros::is_excute(agent) {
            CancelModule::enable_cancel(agent.module_accessor);
    }
}

pub fn install(fighter: &mut smashline::Agent) {
    fighter.game_acmd("game_attack11", game_attack11);
    fighter.game_acmd("game_attackdash", game_attackdash);
    fighter.effect_acmd("effect_attack11", effect_attack11);
    //fighter.sound_acmd("sound_attackairf", sound_attackairf);
}