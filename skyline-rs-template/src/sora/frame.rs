use {
    smash::{
        lua2cpp::*,
        app::lua_bind::*,
        lib::lua_const::*
    },
    crate::custom::global_fighter_frame
};

use smash::app::utility::get_kind;


static mut wallJumpUsed: bool = false;

// What used to be known as a "Once-Per-Fighter-Frame". On-Line functions can be set to run on any status condition.
unsafe extern "C" fn char_on_main(fighter: &mut L2CFighterCommon) {
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
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S  || status_kind == *FIGHTER_TRAIL_STATUS_KIND_SPECIAL_S_ATTACK
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

    if status_kind == *FIGHTER_TRAIL_STATUS_KIND_SPECIAL_S_SEARCH{
        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_TRAIL_STATUS_KIND_SPECIAL_S_END, true);
    }

    if status_kind == *FIGHTER_TRAIL_STATUS_KIND_SPECIAL_S_ATTACK || status_kind == *FIGHTER_TRAIL_STATUS_KIND_SPECIAL_S_END {
        if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
            if ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_JUMP) || (ControlModule::is_enable_flick_jump(module_accessor) && ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_FLICK_JUMP)) {
                if situation_kind == *SITUATION_KIND_AIR{
                    if WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX){
                        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_JUMP_AERIAL, true);
                    }
                }
                else if situation_kind == *SITUATION_KIND_GROUND{
                    StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
                }
            }

            if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE) != 0 {
                if situation_kind == *SITUATION_KIND_AIR{
                    StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, true);
                }
            }
        }
    }
        global_fighter_frame(fighter);
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_line(smashline::Main, char_on_main);
}