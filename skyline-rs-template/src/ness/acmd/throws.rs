use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*, *},
        lib::lua_const::*
    },
    smash_script::*
};


unsafe extern "C" fn game_catch(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        GrabModule::set_rebound(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 0, Hash40::new("top"), 4.0, 0.0, 5.2, 4.0, Some(0.0), Some(5.2), Some(8.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
        macros::CATCH(agent, 1, Hash40::new("top"), 2.0, 0.0, 5.2, 2.0, Some(0.0), Some(5.2), Some(10.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_A);
    }
    macros::game_CaptureCutCommon(agent);
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(agent.module_accessor, false);
    }
}


unsafe extern "C" fn game_catchdash(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        GrabModule::set_rebound(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 0, Hash40::new("top"), 3.2, 0.0, 5.2, 4.0, Some(0.0), Some(5.2), Some(9.8), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
        macros::CATCH(agent, 1, Hash40::new("top"), 1.6, 0.0, 5.2, 2.4, Some(0.0), Some(5.2), Some(11.4), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_A);
    }
    macros::game_CaptureCutCommon(agent);
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(agent.module_accessor, false);
    }
}


unsafe extern "C" fn game_catchturn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        GrabModule::set_rebound(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 0, Hash40::new("top"), 4.0, 0.0, 5.2, -4.0, Some(0.0), Some(5.2), Some(-14.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
        macros::CATCH(agent, 1, Hash40::new("top"), 2.0, 0.0, 5.2, -2.0, Some(0.0), Some(5.2), Some(-16.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_A);
    }
    macros::game_CaptureCutCommon(agent);
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(agent.module_accessor, false);
    }
}

pub fn install(fighter: &mut smashline::Agent) {
    fighter.game_acmd("game_catch", game_catch);
    fighter.game_acmd("game_catchdash", game_catchdash);
    fighter.game_acmd("game_catchturn", game_catchturn);
}