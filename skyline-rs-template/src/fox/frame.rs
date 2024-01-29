use {
    smash::{
        lua2cpp::*,
        app::lua_bind::*,
        lib::lua_const::*
    },
    crate::custom::global_fighter_frame
};

use smash::app::utility::get_kind;

pub unsafe fn stick_y_flick_check(boma: &mut smash::app::BattleObjectModuleAccessor, flick_sensitivity: f32) -> bool {
    let stick_value_y = ControlModule::get_stick_y(boma);
    let cat2 = ControlModule::get_command_flag_cat(boma, 1);
    let stick_value_prev = ControlModule::get_stick_prev_y(boma);
    let flick_check = (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_FALL_JUMP) != 0;
    if flick_sensitivity < 0.0 && stick_value_y < flick_sensitivity && (stick_value_y < stick_value_prev || flick_check) {
        return true;
    }
    else if flick_sensitivity > 0.0 && stick_value_y > flick_sensitivity && (stick_value_y > stick_value_prev || flick_check) {
        return true;
    }
    return false;
}

pub unsafe fn landCancels(boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, fighter_kind: i32) { //Fox & Falco Lasers
    if [*FIGHTER_KIND_FOX].contains(&fighter_kind) {
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N {
            if StatusModule::prev_situation_kind(boma) == *SITUATION_KIND_AIR && situation_kind == *SITUATION_KIND_GROUND {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, true);
            }
        }
    }
    if situation_kind == *SITUATION_KIND_AIR{
        if [*FIGHTER_KIND_FOX].contains(&fighter_kind){
            if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S{
                if status_kind == *FIGHTER_STATUS_KIND_FALL || MotionModule::frame(boma) > 32.0 || CancelModule::is_enable_cancel(boma) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL_SPECIAL, true);
                }
            }
        }
    }
}

pub unsafe fn fastfallShit (boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, fighter_kind: i32) {
    if situation_kind == *SITUATION_KIND_AIR {
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N {
            if [*FIGHTER_KIND_FOX].contains(&fighter_kind) {
                if stick_y_flick_check(boma, -0.3) && KineticModule::get_sum_speed_y(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) <= 0.0 {
                    WorkModule::on_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
                }
            }
        }
    }
}

// What used to be known as a "Once-Per-Fighter-Frame". On-Line functions can be set to run on any status condition.
unsafe extern "C" fn fox_on_main(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = &mut *fighter.module_accessor;
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let fighter_kind = get_kind(module_accessor);
        let situation_kind = StatusModule::situation_kind(module_accessor);
        landCancels(module_accessor, status_kind, situation_kind, fighter_kind);
        fastfallShit(module_accessor, status_kind, situation_kind, fighter_kind);
        global_fighter_frame(fighter);
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_line(smashline::Main, fox_on_main);
}