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

unsafe extern "C" fn game_speciallwstart(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 0.25);
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_GAOGAEN_STATUS_SPECIAL_LW_FLAG_STANCE_START);
    }
    frame(agent.lua_state_agent, 9.0);
    macros::FT_MOTION_RATE(agent, 1.5);
    frame(agent.lua_state_agent, 21.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 28.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_GAOGAEN_STATUS_SPECIAL_LW_FLAG_STANCE_END);
    }
    macros::FT_MOTION_RATE(agent, 0.5);
    frame(agent.lua_state_agent, 32.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(agent.lua_state_agent, 46.0);
    macros::FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn game_specialairlwstart(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 0.25);
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_GAOGAEN_STATUS_SPECIAL_LW_FLAG_STANCE_START);
    }
    frame(agent.lua_state_agent, 9.0);
    macros::FT_MOTION_RATE(agent, 1.5);
    frame(agent.lua_state_agent, 21.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 28.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_GAOGAEN_STATUS_SPECIAL_LW_FLAG_STANCE_END);
    }
    macros::FT_MOTION_RATE(agent, 0.5);
    frame(agent.lua_state_agent, 32.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(agent.lua_state_agent, 46.0);
    macros::FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn game_specialsstart(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_GAOGAEN_STATUS_SPECIAL_S_FLAG_MOVE_START);
        MotionModule::set_frame(agent.module_accessor, 28.0, true);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        GrabModule::set_rebound(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        macros::CATCH(agent, 0, Hash40::new("top"), 4.0, 0.0, 8.0, 2.0, Some(0.0), Some(8.0), Some(8.0), *FIGHTER_STATUS_KIND_SWING_GAOGAEN_CATCHED, *COLLISION_SITUATION_MASK_G);
        macros::CATCH(agent, 1, Hash40::new("top"), 1.5, 0.0, 8.0, 2.0, Some(0.0), Some(8.0), Some(10.5), *FIGHTER_STATUS_KIND_SWING_GAOGAEN_CATCHED, *COLLISION_SITUATION_MASK_G);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        GrabModule::set_rebound(agent.module_accessor, false);
        macros::FT_MOTION_RATE(agent, 1.8);
    }
    frame(agent.lua_state_agent, 39.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_GAOGAEN_STATUS_SPECIAL_S_FLAG_MOVE_END);
    }
    frame(agent.lua_state_agent, 67.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_GAOGAEN_STATUS_SPECIAL_S_WORK_ID_FLAG_AIR_CONTROL);
    }
}

unsafe extern "C" fn game_specialairsstart(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_GAOGAEN_STATUS_SPECIAL_S_FLAG_MOVE_START);
    }
    macros::FT_MOTION_RATE(agent, 0.8);
    frame(agent.lua_state_agent, 11.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 350, 100, 30, 0, 5.0, 0.0, 7.0, 2.0, Some(0.0), Some(7.0), Some(5.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 0.0, 350, 100, 30, 0, 5.0, 0.0, 7.0, 2.0, Some(0.0), Some(7.0), Some(5.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_FEB, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        GrabModule::set_rebound(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        macros::CATCH(agent, 0, Hash40::new("top"), 4.0, 0.0, 8.0, 2.0, Some(0.0), Some(8.0), Some(8.0), *FIGHTER_STATUS_KIND_SWING_GAOGAEN_CATCHED, *COLLISION_SITUATION_MASK_A);
        macros::CATCH(agent, 1, Hash40::new("top"), 2.5, 0.0, 8.0, 2.0, Some(0.0), Some(8.0), Some(9.5), *FIGHTER_STATUS_KIND_SWING_GAOGAEN_CATCHED, *COLLISION_SITUATION_MASK_A);
    }
    frame(agent.lua_state_agent, 35.0);
    if macros::is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        GrabModule::set_rebound(agent.module_accessor, false);
    }
    frame(agent.lua_state_agent, 39.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_GAOGAEN_STATUS_SPECIAL_S_FLAG_MOVE_END);
    }
    frame(agent.lua_state_agent, 44.0);
    macros::FT_MOTION_RATE(agent, 0.8);
    frame(agent.lua_state_agent, 67.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_GAOGAEN_STATUS_SPECIAL_S_WORK_ID_FLAG_AIR_CONTROL);
    }
    frame(agent.lua_state_agent, 74.0);
    macros::FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn effect_specialsstart(agent: &mut L2CAgentBase) {
   
}

unsafe extern "C" fn game_specialairn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 1.6);
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::FT_MOTION_RATE(agent, 1.0);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 17.0, 45, 47, 0, 85, 4.6, 0.0, 10.0, 4.0, Some(0.0), Some(10.0), Some(8.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 30, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 13.0, 45, 50, 0, 80, 4.2, 0.0, 10.0, -4.0, Some(0.0), Some(10.0), Some(-4.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 30, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 13.0, 45, 50, 0, 80, 4.6, 0.0, 8.0, 2.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 30, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_GAOGAEN_STATUS_SPECIAL_N_FLAG_START_ROTATION);
        AttackModule::set_target_category(agent.module_accessor, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_target_category(agent.module_accessor, 1, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_target_category(agent.module_accessor, 2, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_size(agent.module_accessor, 0, 0.1);
        AttackModule::set_size(agent.module_accessor, 1, 0.1);
        AttackModule::set_size(agent.module_accessor, 2, 0.1);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 45, 50, 0, 85, 4.2, 0.0, 10.0, 4.0, Some(0.0), Some(10.0), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 30, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 10.0, 45, 40, 0, 80, 3.8, 0.0, 10.0, -2.0, Some(0.0), Some(10.0), Some(-8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 30, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 10.0, 45, 40, 0, 80, 4.2, 0.0, 8.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 30, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::set_target_category(agent.module_accessor, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_target_category(agent.module_accessor, 1, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_target_category(agent.module_accessor, 2, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_size(agent.module_accessor, 0, 0.1);
        AttackModule::set_size(agent.module_accessor, 1, 0.1);
        AttackModule::set_size(agent.module_accessor, 2, 0.1);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 45, 50, 0, 85, 4.2, 0.0, 10.0, 4.0, Some(0.0), Some(10.0), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 30, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 10.0, 45, 40, 0, 80, 3.8, 0.0, 10.0, -2.0, Some(0.0), Some(10.0), Some(-8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 30, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 10.0, 45, 40, 0, 80, 4.2, 0.0, 8.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 30, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::set_target_category(agent.module_accessor, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_target_category(agent.module_accessor, 1, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_target_category(agent.module_accessor, 2, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_size(agent.module_accessor, 0, 0.1);
        AttackModule::set_size(agent.module_accessor, 1, 0.1);
        AttackModule::set_size(agent.module_accessor, 2, 0.1);
    }
    frame(agent.lua_state_agent, 23.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 11.0, 45, 50, 0, 85, 4.2, 0.0, 10.0, -2.0, Some(0.0), Some(10.0), Some(-8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 9.0, 45, 40, 0, 80, 3.8, 0.0, 10.0, 4.0, Some(0.0), Some(10.0), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 9.0, 45, 40, 0, 80, 4.2, 0.0, 8.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_GAOGAEN_STATUS_SPECIAL_N_FLAG_END_ROTATION);
    }
    frame(agent.lua_state_agent, 28.0);
    MotionModule::set_frame(agent.module_accessor, 56.0, true);
    frame(agent.lua_state_agent, 45.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_GAOGAEN_STATUS_SPECIAL_N_FLAG_REQUEST_GRAVITY_DEFAULT);
    }
    frame(agent.lua_state_agent, 51.0);
    if macros::is_excute(agent) {
        CancelModule::enable_cancel(agent.module_accessor);
    }
}

unsafe extern "C" fn game_specialn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 1.6);
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::FT_MOTION_RATE(agent, 1.0);
        macros::WHOLE_HIT(agent, *HIT_STATUS_INVINCIBLE);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 17.0, 45, 47, 0, 85, 5.8, 0.0, 11.0, 4.0, Some(0.0), Some(11.0), Some(8.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 13.0, 45, 50, 0, 80, 5.4, 0.0, 11.0, -4.0, Some(0.0), Some(11.0), Some(-4.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 13.0, 45, 50, 0, 80, 4.6, 0.0, 8.0, 2.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_GAOGAEN_STATUS_SPECIAL_N_FLAG_START_ROTATION);
        macros::WHOLE_HIT(agent, *HIT_STATUS_NORMAL);
        macros::HIT_NODE(agent, Hash40::new("head"), *HIT_STATUS_INVINCIBLE);
        macros::HIT_NODE(agent, Hash40::new("shoulderr"), *HIT_STATUS_INVINCIBLE);
        macros::HIT_NODE(agent, Hash40::new("shoulderl"), *HIT_STATUS_INVINCIBLE);
        macros::HIT_NODE(agent, Hash40::new("armr"), *HIT_STATUS_INVINCIBLE);
        macros::HIT_NODE(agent, Hash40::new("arml"), *HIT_STATUS_INVINCIBLE);
    }
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
        if macros::is_excute(agent) {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_GAOGAEN_STATUS_SPECIAL_N_FLAG_START_ROTATION);
            macros::WHOLE_HIT(agent, *HIT_STATUS_NORMAL);
            macros::HIT_NODE(agent, Hash40::new("body"), *HIT_STATUS_INVINCIBLE);
            macros::HIT_NODE(agent, Hash40::new("shoulderr"), *HIT_STATUS_INVINCIBLE);
            macros::HIT_NODE(agent, Hash40::new("shoulderl"), *HIT_STATUS_INVINCIBLE);
        }
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::HIT_NODE(agent, Hash40::new("head"), *HIT_STATUS_NORMAL);
        AttackModule::set_target_category(agent.module_accessor, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_target_category(agent.module_accessor, 1, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_target_category(agent.module_accessor, 2, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_size(agent.module_accessor, 0, 0.1);
        AttackModule::set_size(agent.module_accessor, 1, 0.1);
        AttackModule::set_size(agent.module_accessor, 2, 0.1);
    }
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
        if macros::is_excute(agent) {
            macros::HIT_RESET_ALL(agent);
            AttackModule::set_target_category(agent.module_accessor, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
            AttackModule::set_target_category(agent.module_accessor, 1, *COLLISION_CATEGORY_MASK_NO_IF as u32);
            AttackModule::set_target_category(agent.module_accessor, 2, *COLLISION_CATEGORY_MASK_NO_IF as u32);
            AttackModule::set_size(agent.module_accessor, 0, 0.1);
            AttackModule::set_size(agent.module_accessor, 1, 0.1);
            AttackModule::set_size(agent.module_accessor, 2, 0.1);
            macros::HIT_NODE(agent, Hash40::new("shoulderr"), *HIT_STATUS_INVINCIBLE);
            macros::HIT_NODE(agent, Hash40::new("shoulderl"), *HIT_STATUS_INVINCIBLE);
        }
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 45, 50, 0, 85, 4.2, 0.0, 11.0, 4.0, Some(0.0), Some(11.0), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 10.0, 45, 40, 0, 80, 3.8, 0.0, 11.0, -2.0, Some(0.0), Some(11.0), Some(-8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 10.0, 45, 40, 0, 80, 4.2, 0.0, 8.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::set_target_category(agent.module_accessor, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_target_category(agent.module_accessor, 1, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_target_category(agent.module_accessor, 2, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_size(agent.module_accessor, 0, 0.1);
        AttackModule::set_size(agent.module_accessor, 1, 0.1);
        AttackModule::set_size(agent.module_accessor, 2, 0.1);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 45, 50, 0, 85, 4.2, 0.0, 10.0, 4.0, Some(0.0), Some(10.0), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 10.0, 45, 40, 0, 80, 3.8, 0.0, 10.0, -2.0, Some(0.0), Some(10.0), Some(-8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 10.0, 45, 40, 0, 80, 4.2, 0.0, 8.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::set_target_category(agent.module_accessor, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_target_category(agent.module_accessor, 1, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_target_category(agent.module_accessor, 2, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_size(agent.module_accessor, 0, 0.1);
        AttackModule::set_size(agent.module_accessor, 1, 0.1);
        AttackModule::set_size(agent.module_accessor, 2, 0.1);
    }
    frame(agent.lua_state_agent, 23.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 11.0, 45, 50, 0, 85, 4.2, 0.0, 10.0, -2.0, Some(0.0), Some(10.0), Some(-8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 9.0, 45, 40, 0, 80, 3.8, 0.0, 10.0, 4.0, Some(0.0), Some(10.0), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 9.0, 45, 40, 0, 80, 4.2, 0.0, 8.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        HitModule::set_status_all(agent.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_GAOGAEN_STATUS_SPECIAL_N_FLAG_END_ROTATION);
    }
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
        if macros::is_excute(agent) {
            macros::HIT_RESET_ALL(agent);
            AttackModule::clear_all(agent.module_accessor);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_GAOGAEN_STATUS_SPECIAL_N_FLAG_END_ROTATION);
        }
    }
    frame(agent.lua_state_agent, 28.0);
    MotionModule::set_frame(agent.module_accessor, 56.0, true);
    frame(agent.lua_state_agent, 45.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_GAOGAEN_STATUS_SPECIAL_N_FLAG_REQUEST_GRAVITY_DEFAULT);
    }
    frame(agent.lua_state_agent, 51.0);
    if macros::is_excute(agent) {
        CancelModule::enable_cancel(agent.module_accessor);
    }
}

pub fn install(fighter: &mut smashline::Agent) {
    fighter.game_acmd("game_specialn", game_specialn);
    fighter.game_acmd("game_specialairn", game_specialairn);
    fighter.game_acmd("game_specialsstart", game_specialsstart);
    fighter.game_acmd("game_specialairsstart", game_specialairsstart);
    fighter.game_acmd("game_speciallwstart", game_speciallwstart);
    fighter.game_acmd("game_specialairlwstart", game_specialairlwstart);
    fighter.effect_acmd("effect_specialsstart", effect_specialsstart);
}