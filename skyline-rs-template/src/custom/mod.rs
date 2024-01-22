use smash::lib::{L2CValue, L2CAgent};
use skyline::nro::{self, NroInfo};
use smash::app::BattleObjectModuleAccessor;
use smash::app::lua_bind::*;
use smash::hash40;
use smash::app::utility::get_kind;
use smash::app::utility::*;
use smash::lua2cpp::*;
use smash::lib::lua_const::*;
use smash::phx::*;
use smash::app::GroundCorrectKind;
use skyline::nn::ro::LookupSymbol;
use smashline::*;
use super::*;
use smash::app::FighterKineticEnergyGravity;
use smash_script::lua_args;
use smash::app::sv_animcmd::*;

pub unsafe fn shieldStops(boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, cat1: i32) { //Shield Stop
    if status_kind == *FIGHTER_STATUS_KIND_DASH || status_kind == *FIGHTER_STATUS_KIND_TURN_DASH || status_kind == *FIGHTER_STATUS_KIND_LANDING
    || status_kind == *FIGHTER_STATUS_KIND_LANDING_LIGHT {
        if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE) != 0 || ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_GUARD) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_GUARD_ON, true);
        }
    }
}

pub unsafe fn dashPlatformDrop(boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, stick_value_y: f32, stick_value_x: f32) { //Dash Platform Drop
    if [*FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_TURN_DASH, *FIGHTER_STATUS_KIND_RUN, *FIGHTER_STATUS_KIND_TURN_RUN, *FIGHTER_STATUS_KIND_RUN_BRAKE, *FIGHTER_STATUS_KIND_TURN_RUN_BRAKE].contains(&status_kind) 
    {
            if situation_kind == SITUATION_KIND_GROUND {
                if stick_value_y <= -0.75 {
                    if GroundModule::is_passable_ground(boma) {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_PASS, true);
                    }
                }
            }
    }
}

const PIVOT_STICK_SNAPBACK_WINDOW: f32 = 1.0;
const LIL_BOOSTIE: smash::phx::Vector3f = smash::phx::Vector3f {x: 1.6, y: 0.0, z: 0.0};
unsafe fn pivots(boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, curr_frame: f32, stick_value_x: f32){
    if status_kind == *FIGHTER_STATUS_KIND_TURN_DASH
        && curr_frame <= PIVOT_STICK_SNAPBACK_WINDOW && stick_value_x == 0.0
        && [*FIGHTER_STATUS_KIND_TURN_DASH, *FIGHTER_STATUS_KIND_DASH].contains(&StatusModule::prev_status_kind(boma, 0))
        && ![*FIGHTER_STATUS_KIND_WAIT, *FIGHTER_STATUS_KIND_TURN].contains(&StatusModule::prev_status_kind(boma, 1))
    {
        PostureModule::reverse_lr(boma);
        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_TURN,true);
        KineticModule::clear_speed_all(boma);
        KineticModule::add_speed(boma, &LIL_BOOSTIE);
    }
}

pub unsafe fn fixbackdash(boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, motion_kind: u64, cat1: i32, stick_value_y: f32) { 
    if status_kind == *FIGHTER_STATUS_KIND_TURN_DASH{
        if MotionModule::frame(boma) == (2.0){
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_DASH, true);
        }

    }
}

pub unsafe extern "C" fn global_fighter_frame(fighter : &mut L2CFighterCommon) {
    JostleModule::set_team(fighter.module_accessor, 0);
    let module_accessor = &mut *fighter.module_accessor;
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);
    let cat2 = ControlModule::get_command_flag_cat(fighter.module_accessor, 1);
    let cat3 = ControlModule::get_command_flag_cat(fighter.module_accessor, 2);
    let stick_value_y = ControlModule::get_stick_y(fighter.module_accessor);
    let stick_value_x = ControlModule::get_stick_x(fighter.module_accessor);
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let curr_frame = MotionModule::frame(fighter.module_accessor);

    shieldStops(module_accessor, status_kind, cat1);
    dashPlatformDrop(module_accessor, status_kind, situation_kind, stick_value_y, stick_value_x);
    pivots(module_accessor, status_kind, curr_frame, stick_value_x);
    fixbackdash(module_accessor, status_kind, motion_kind, cat1, stick_value_y);
}