use {
    smash::{
        lua2cpp::*,
        app::lua_bind::*,
        lib::lua_const::*,
        app::{sv_animcmd::*, lua_bind::*},
        phx::*
    },
    crate::custom::global_fighter_frame
};

use smash::app::utility::get_kind;
use smash::phx::Vector3f;
use smash_script::*;


static mut swordLevel : i32 = 0;
static mut damageTaken : f32 = 0.0;

unsafe extern "C" fn game_speciallw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        if swordLevel < 3{
            swordLevel = swordLevel + 1;
            damageTaken = DamageModule::damage(agent.module_accessor, 0);
        }
    }
    frame(agent.lua_state_agent, 35.0);
    macros::FT_MOTION_RATE(agent, 0.8);
}

unsafe extern "C" fn game_specialairlw(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        KineticModule::add_speed(agent.module_accessor, &Vector3f{x: 0.0, y: 0.4, z: 0.0});
    }
    
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        if swordLevel < 3{
            swordLevel = swordLevel + 1;
            damageTaken = DamageModule::damage(agent.module_accessor, 0);
        }
    }
    frame(agent.lua_state_agent, 35.0);
    macros::FT_MOTION_RATE(agent, 0.8);
}

unsafe extern "C" fn agent_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        swordLevel = 0;
        damageTaken = 0.0;
    }
}

// What used to be known as a "Once-Per-Fighter-Frame". On-Line functions can be set to run on any status condition.
unsafe extern "C" fn char_on_main(fighter: &mut L2CFighterCommon) {
    unsafe {

        let lua_state = fighter.lua_state_agent;
        let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
        let fighter_kind = get_kind(boma);
        let status_kind = StatusModule::status_kind(boma);
        let situation_kind = StatusModule::situation_kind(boma);
        let motion_kind = MotionModule::motion_kind(boma);
        let stick_value_x = ControlModule::get_stick_x(boma);

        if swordLevel == 0{
            AttackModule::set_power_up(boma, 1.0);
        }else if swordLevel == 1{
            AttackModule::set_power_up(boma, 1.2);
        }else if swordLevel == 2{
            AttackModule::set_power_up(boma, 1.4);
        }else if swordLevel == 3{
            AttackModule::set_power_up(boma, 1.6);
        }

        if DamageModule::damage(boma, 0) >= damageTaken + 20.0 {
            swordLevel = 0;
        }

        if [*FIGHTER_STATUS_KIND_CAPTURE_WAIT, *FIGHTER_STATUS_KIND_CAPTURE_JACK_WIRE, 
        *FIGHTER_STATUS_KIND_CAPTURE_MASTER_SWORD, *FIGHTER_STATUS_KIND_DEAD].contains(&status_kind)
        {
            swordLevel = 0;
        }

        global_fighter_frame(fighter);
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.game_acmd("game_speciallw", game_speciallw);
    agent.game_acmd("game_specialairlw", game_specialairlw);
    agent.on_start(agent_init);
    agent.on_line(smashline::Main, char_on_main);
}