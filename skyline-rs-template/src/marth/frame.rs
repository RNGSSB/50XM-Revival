use {
    smash::{
        lua2cpp::*,
        app::lua_bind::*,
        lib::lua_const::*
    },
    crate::custom::global_fighter_frame
};

use smash::app::BattleObjectModuleAccessor;
use smash::app::utility::get_kind;
use smash::phx::*;

static mut wallJumpUsed: bool = false;


pub unsafe fn sword_length(boma: &mut BattleObjectModuleAccessor, fighter_kind: i32) {
    let long_sword_scale = Vector3f{x: 1.015, y: 1.115, z: 1.045};
    if [*FIGHTER_KIND_MARTH].contains(&fighter_kind){
        ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("havel"), &long_sword_scale);
        ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("haver"), &long_sword_scale);
    }
}

// What used to be known as a "Once-Per-Fighter-Frame". On-Line functions can be set to run on any status condition.
unsafe extern "C" fn marth_on_main(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = &mut *fighter.module_accessor;
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let fighter_kind = get_kind(module_accessor);
        let situation_kind = StatusModule::situation_kind(module_accessor);
        let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);

        if status_kind == *FIGHTER_STATUS_KIND_WALL_JUMP{
            wallJumpUsed = true;
        }

        if situation_kind != *SITUATION_KIND_AIR {
            wallJumpUsed = false;
        }

        if !wallJumpUsed{
            if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N || status_kind == *FIGHTER_MARTH_STATUS_KIND_SPECIAL_N_END
            || status_kind == *FIGHTER_MARTH_STATUS_KIND_SPECIAL_N_LOOP || status_kind == *FIGHTER_MARTH_STATUS_KIND_SPECIAL_N_END_MAX
            {
                if situation_kind == *SITUATION_KIND_AIR{
                    let touch_right = GroundModule::is_wall_touch_line(module_accessor, *GROUND_TOUCH_FLAG_RIGHT_SIDE as u32);
                    let touch_left = GroundModule::is_wall_touch_line(module_accessor, *GROUND_TOUCH_FLAG_LEFT_SIDE as u32);
                    if touch_left || touch_right {
                        if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_WALL_JUMP_LEFT) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_WALL_JUMP_RIGHT) != 0
                        {
                                StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WALL_JUMP, true);
                            
                        }
                    }
                }
            }
        }
        
        global_fighter_frame(fighter);
        sword_length(module_accessor, fighter_kind);
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_line(smashline::Main, marth_on_main);
}