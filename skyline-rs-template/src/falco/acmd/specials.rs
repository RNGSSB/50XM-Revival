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


unsafe extern "C" fn game_speciallw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("kneer"), 8.0, 45, 1, 0, 80, 3.2, 5.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("legr"), 8.0, 45, 1, 0, 80, 3.5, 3.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 2, 0, Hash40::new("hip"), 8.0, 45, 1, 0, 80, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("reflector"), 5.0, 110, 50, 0, 110, 3.5, 1.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
    }
    frame(agent.lua_state_agent, 31.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        if !AttackModule::is_infliction_status(agent.module_accessor, *COLLISION_KIND_MASK_HIT) {
            macros::FT_MOTION_RATE(agent, 1.6);
        }
    }
}

unsafe extern "C" fn game_specialairlw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("kneer"), 8.0, 45, 1, 0, 80, 3.2, 5.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("legr"), 8.0, 45, 1, 0, 80, 3.5, 3.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 2, 0, Hash40::new("hip"), 8.0, 45, 1, 0, 80, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("reflector"), 5.0, 110, 50, 0, 110, 3.5, 1.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
    }
    frame(agent.lua_state_agent, 31.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        if !AttackModule::is_infliction_status(agent.module_accessor, *COLLISION_KIND_MASK_HIT) {
            macros::FT_MOTION_RATE(agent, 1.6);
        }
    }
}

unsafe extern "C" fn sound_speciallw(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_falco_rnd_attack"));
        macros::PLAY_SE(agent, Hash40::new("se_falco_swing_m"));
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_falco_special_l01"));
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_falco_special_l01"));
    }
    frame(agent.lua_state_agent, 31.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_falco_special_l03"));
    }
}

unsafe extern "C" fn sound_specialairlw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_falco_rnd_attack"));
        macros::PLAY_SE(agent, Hash40::new("se_falco_swing_m"));
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_falco_special_l01"));
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_falco_special_l01"));
    }
    frame(agent.lua_state_agent, 31.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_falco_special_l03"));
    }
}

unsafe extern "C" fn effect_speciallw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_attack_line"), Hash40::new("top"), -7.2, 3.8, 1, -22, 27, 0, 1, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.2);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::EFFECT_ALPHA(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 10.1, 14.5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 360, true, 0.7);
        macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT_FLW_POS(agent, Hash40::new("falco_ref_loop"), Hash40::new("reflector"), 1.4, -0.6, 0, 0, 0, 0, 1, true);
        EffectModule::preset_limit_num(agent.module_accessor, 2);
        macros::EFFECT_FOLLOW(agent, Hash40::new("falco_ref_flash"), Hash40::new("reflector"), 1.4, -0.6, 0, 0, 0, 0, 1, true);
        macros::FLASH(agent, 1, 1, 1, 0.627);
        macros::EFFECT_FLW_POS(agent, Hash40::new("falco_ref_start"), Hash40::new("reflector"), 1.4, -0.6, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 23.0);
    if macros::is_excute(agent) {
        macros::COL_NORMAL(agent);
    }
    frame(agent.lua_state_agent, 32.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("falco_ref_loop"), false, false);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("falco_ref_ref"), false, false);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("falco_ref_flash"), true, false);
        macros::EFFECT_FLW_POS(agent, Hash40::new("sys_flash"), Hash40::new("reflector"), 1.4, -0.6, -0.5, 0, 0, 0, 0.5, true);
    }
}

unsafe extern "C" fn effect_specialairlw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_attack_line"), Hash40::new("top"), -7.2, 3.8, 1, -22, 27, 0, 1, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.2);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::EFFECT_ALPHA(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 10.1, 14.5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 360, true, 0.7);
        macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FLW_POS(agent, Hash40::new("falco_ref_loop"), Hash40::new("reflector"), 1.4, -0.6, 0, 0, 0, 0, 1, true);
        EffectModule::preset_limit_num(agent.module_accessor, 2);
        macros::EFFECT_FOLLOW(agent, Hash40::new("falco_ref_flash"), Hash40::new("reflector"), 1.4, -0.6, 0, 0, 0, 0, 1, true);
        macros::FLASH(agent, 1, 1, 1, 0.627);
        macros::EFFECT_FLW_POS(agent, Hash40::new("falco_ref_start"), Hash40::new("reflector"), 1.4, -0.6, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 23.0);
    if macros::is_excute(agent) {
        macros::COL_NORMAL(agent);
    }
    frame(agent.lua_state_agent, 32.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("falco_ref_loop"), false, false);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("falco_ref_ref"), false, false);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("falco_ref_flash"), true, false);
        macros::EFFECT_FLW_POS(agent, Hash40::new("sys_flash"), Hash40::new("reflector"), 1.4, -0.6, -0.5, 0, 0, 0, 0.5, true);
    }
}

unsafe extern "C" fn game_specialsstart(agent: &mut L2CAgentBase) {
    
}


unsafe extern "C" fn game_specialhihold(agent: &mut L2CAgentBase) {
    
}

pub fn install(fighter: &mut smashline::Agent) {
    fighter.game_acmd("game_specialairlw", game_specialairlw);
    fighter.game_acmd("game_speciallw", game_speciallw);
    fighter.game_acmd("game_specialsstart", game_specialsstart);
    fighter.game_acmd("game_specialhihold", game_specialhihold);
    fighter.game_acmd("game_specialhiholdair", game_specialhihold);
    fighter.effect_acmd("effect_speciallw", effect_speciallw);
    fighter.effect_acmd("effect_specialairlw", effect_specialairlw);
    fighter.sound_acmd("sound_specialairlw", sound_specialairlw);  
    fighter.sound_acmd("sound_speciallw", sound_speciallw);  
}