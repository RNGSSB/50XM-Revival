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
use smash::app::*;
use smash::app;
use skyline::nn::ro::LookupSymbol;
use smashline::*;
use super::*;
use smash::app::FighterKineticEnergyGravity;
use smash_script::*;
use smash::app::sv_animcmd::*;
use smashline::Pre;
use globals::*;

use utils::{
    *,
    util::*,
    ext::*,
    consts::*,
};

use skyline::install_hooks;

static mut LAGCANCELED: [bool; 9] = [false; 9];
static mut LEDGE_POS: [Vector3f; 9] = [smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0}; 9];
static mut ECB_Y_OFFSETS: [f32; 9] = [0.0; 9];
static mut CURRENTMOMENTUM: [f32; 9] = [0.0; 9];
static mut CANAIRDODGE: [bool; 9] = [true; 9];
static mut CANWALLJUMP: [bool; 9] = [true; 9];

pub const SUB_STATUS: i32 = 0x15; //sub status, i32 value
pub const STICK_X: i32 = 0x1A; //stick x, f32 value
pub const MODULE_ACCESSOR: i32 = 0x5; //module accessor, ptr value
pub const NONE_VECTOR: smash::phx::Vector3f = smash::phx::Vector3f {x: 0.0, y: 0.0, z: 0.0};
pub const FIGHTER_INSTANCE_WORK_ID_INT_PARRIED: i32 = 0x921;

mod jumpsquat;
mod jabs;
mod dashattack;


pub unsafe fn returnSmall(arg1: f32, arg2: f32) -> f32{
    if arg1 < arg2 {
        return arg1;
    }
    else {
        return arg2;
    }
}

pub unsafe fn returnLarge(arg1: f32, arg2: f32) -> f32{
    if arg1 > arg2 {
        return arg1;
    }
    else {
        return arg2;
    }
}

pub unsafe fn get_player_number(boma: &mut smash::app::BattleObjectModuleAccessor) -> usize {
    return WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
}

pub unsafe fn shieldStops(boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, cat1: i32) { //Shield Stop
    if status_kind == *FIGHTER_STATUS_KIND_DASH || status_kind == *FIGHTER_STATUS_KIND_TURN_DASH {
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

    if [*FIGHTER_STATUS_KIND_RUN].contains(&status_kind){
        if stick_value_y <= -0.66 {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SQUAT, true);
        }
    }
}

pub unsafe fn grabRefreshJumps(boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32) { 
    if [*FIGHTER_STATUS_KIND_CAPTURE_PULLED,
    *FIGHTER_STATUS_KIND_CAPTURE_WAIT,
    *FIGHTER_STATUS_KIND_CAPTURE_DAMAGE,
    *FIGHTER_STATUS_KIND_CAPTURE_CUT,
    *FIGHTER_STATUS_KIND_CAPTURE_JUMP,
    *FIGHTER_STATUS_KIND_SHOULDERED_DONKEY,
    *FIGHTER_STATUS_KIND_SWALLOWED,
    *FIGHTER_STATUS_KIND_CLUNG_CAPTAIN,
    *FIGHTER_STATUS_KIND_KOOPA_DIVED,
    *FIGHTER_STATUS_KIND_CLUNG_GANON,
    *FIGHTER_STATUS_KIND_MEWTWO_THROWN,
    *FIGHTER_STATUS_KIND_BITTEN_WARIO_START,
    *FIGHTER_STATUS_KIND_CLUNG_DIDDY,
    *FIGHTER_STATUS_KIND_MIIFIGHTER_COUNTER_THROWN,
    *FIGHTER_STATUS_KIND_CATCHED_REFLET,
    *FIGHTER_STATUS_KIND_CATCHED_RIDLEY,
    *FIGHTER_STATUS_KIND_CAPTURE_PULLED_FISHINGROD,
    *FIGHTER_STATUS_KIND_SWING_GAOGAEN_CATCHED,
    *FIGHTER_STATUS_KIND_CAPTURE_JACK_WIRE,
    *FIGHTER_STATUS_KIND_DEMON_DIVED].contains(&status_kind) 
    && WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) == WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX)
    {
        WorkModule::dec_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
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

#[skyline::hook(replace = smash::app::lua_bind::WorkModule::get_param_float)]
//Landing lag stuff
pub unsafe fn get_param_float_hook(boma: &mut smash::app::BattleObjectModuleAccessor, param_type: u64, param_hash: u64) -> f32 {
    let status_kind = StatusModule::status_kind(boma);
    let stick_value_y = ControlModule::get_stick_y(boma);
    let fighter_kind = get_kind(boma);
    if param_hash == 0 {
     if [hash40("landing_attack_air_frame_n"), hash40("landing_attack_air_frame_hi"), hash40("landing_attack_air_frame_lw"), 
                 hash40("landing_attack_air_frame_f"), hash40("landing_attack_air_frame_b")].contains(&param_type) {
            let origLandingLag = original!()(boma, param_type, param_hash);
            if LAGCANCELED[get_player_number(boma)] {
                return origLandingLag;
            }
            else {
                let newLandingLag = (origLandingLag + 5.0) as i32;
                return newLandingLag as f32;
            }
        }
    }
    original!()(boma, param_type, param_hash)
}

pub unsafe fn lagCanceled(boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32) {
    if [*FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR].contains(&status_kind) {
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) || AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) {
            LAGCANCELED[get_player_number(boma)] = true;
        }
    }
    else {
        LAGCANCELED[get_player_number(boma)] = false;
    }
}

pub unsafe fn shieldDrops(boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, cat2: i32) {
    if status_kind == *FIGHTER_STATUS_KIND_GUARD || status_kind == *FIGHTER_STATUS_KIND_GUARD_ON || status_kind == *FIGHTER_STATUS_KIND_GUARD_OFF { //Shield Drop
        if (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_GUARD_TO_PASS) != 0 || (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_HI) != 0 || (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_LW) != 0  ||
        (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_L) != 0 || (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_R) != 0 {
            if GroundModule::is_passable_ground(boma) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_PASS, true);
            }
        }
    }
    
    if status_kind == *FIGHTER_STATUS_KIND_JUMP || status_kind == *FIGHTER_STATUS_KIND_JUMP_AERIAL ||
    status_kind == *FIGHTER_STATUS_KIND_FALL || status_kind == *FIGHTER_STATUS_KIND_FALL_AERIAL
    {
        if (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_HI) != 0 || (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_LW) != 0  ||
        (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_L) != 0 || (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_R) != 0{
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_DAMAGE_FALL, true);
        }
    }

}

pub unsafe fn meleeECBs(boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, fighter_kind: i32) {
    let mut offset = smash::phx::Vector2f { x: 0.0, y: 0.0};
        let mut max_offset = 0.0;
        let vanilla_ecb =     [*FIGHTER_STATUS_KIND_CAPTURE_PULLED, *FIGHTER_STATUS_KIND_CAPTURE_WAIT, *FIGHTER_STATUS_KIND_CAPTURE_DAMAGE, *FIGHTER_STATUS_KIND_CAPTURE_CUT,
                               *FIGHTER_STATUS_KIND_THROWN].contains(&StatusModule::prev_status_kind(boma, 0)) ||
                               [*FIGHTER_STATUS_KIND_CAPTURE_PULLED, *FIGHTER_STATUS_KIND_CAPTURE_WAIT, *FIGHTER_STATUS_KIND_CAPTURE_DAMAGE, *FIGHTER_STATUS_KIND_CAPTURE_CUT,
                                *FIGHTER_STATUS_KIND_ENTRY, *FIGHTER_STATUS_KIND_THROWN, *FIGHTER_STATUS_KIND_DAMAGE_FLY, *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL, 
                                *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U,
                                *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D, *FIGHTER_STATUS_KIND_DAMAGE_FALL, *FIGHTER_STATUS_KIND_TREAD_DAMAGE_AIR, *FIGHTER_STATUS_KIND_BURY,
                                *FIGHTER_STATUS_KIND_BURY_WAIT].contains(&status_kind);
                                
        let air_trans = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_FRAME_IN_AIR) < 10;
                
            
        if  [*FIGHTER_KIND_KIRBY, *FIGHTER_KIND_PIKACHU, *FIGHTER_KIND_NESS, *FIGHTER_KIND_PURIN, *FIGHTER_KIND_GAMEWATCH, *FIGHTER_KIND_POPO, *FIGHTER_KIND_NANA, 
            *FIGHTER_KIND_PICHU, *FIGHTER_KIND_METAKNIGHT, *FIGHTER_KIND_WARIO, *FIGHTER_KIND_PZENIGAME, *FIGHTER_KIND_PFUSHIGISOU, *FIGHTER_KIND_LUCAS, 
            *FIGHTER_KIND_PIKMIN, *FIGHTER_KIND_TOONLINK, *FIGHTER_KIND_DUCKHUNT, *FIGHTER_KIND_MURABITO, *FIGHTER_KIND_INKLING, *FIGHTER_KIND_SHIZUE].contains(&fighter_kind) {
                max_offset = 2.0;
            }
            
        if  [*FIGHTER_KIND_MARIO, *FIGHTER_KIND_YOSHI, *FIGHTER_KIND_LUIGI, *FIGHTER_KIND_MARIOD, *FIGHTER_KIND_YOUNGLINK, *FIGHTER_KIND_PLIZARDON, *FIGHTER_KIND_DIDDY, 
            *FIGHTER_KIND_DEDEDE, *FIGHTER_KIND_ROCKMAN, *FIGHTER_KIND_GEKKOUGA, *FIGHTER_KIND_PACMAN, *FIGHTER_KIND_KOOPAJR, *FIGHTER_KIND_PACKUN, *FIGHTER_KIND_MIIFIGHTER, 
            *FIGHTER_KIND_MIISWORDSMAN, *FIGHTER_KIND_MIIGUNNER, *FIGHTER_KIND_PACKUN, *FIGHTER_KIND_BUDDY].contains(&fighter_kind) {
                max_offset = 3.5;
            }
            
        if  [*FIGHTER_KIND_FOX, *FIGHTER_KIND_FALCO, *FIGHTER_KIND_DAISY, *FIGHTER_KIND_MEWTWO, *FIGHTER_KIND_PIT, *FIGHTER_KIND_PITB, *FIGHTER_KIND_SONIC, *FIGHTER_KIND_LUCARIO, 
            *FIGHTER_KIND_ROBOT, *FIGHTER_KIND_WOLF, *FIGHTER_KIND_LITTLEMAC, *FIGHTER_KIND_KROOL, *FIGHTER_KIND_GAOGAEN, *FIGHTER_KIND_TANTAN, *FIGHTER_KIND_PICKEL].contains(&fighter_kind) {
                max_offset = 4.0;
            }
            
        if  [*FIGHTER_KIND_DONKEY, *FIGHTER_KIND_LINK, *FIGHTER_KIND_SAMUS, *FIGHTER_KIND_SAMUSD, *FIGHTER_KIND_CAPTAIN, *FIGHTER_KIND_PEACH, *FIGHTER_KIND_KOOPA, 
            *FIGHTER_KIND_SHEIK, *FIGHTER_KIND_ZELDA, *FIGHTER_KIND_MARTH, *FIGHTER_KIND_LUCINA, *FIGHTER_KIND_GANON, *FIGHTER_KIND_ROY, *FIGHTER_KIND_CHROM, 
            *FIGHTER_KIND_SZEROSUIT, *FIGHTER_KIND_SNAKE, *FIGHTER_KIND_IKE, *FIGHTER_KIND_WIIFIT, *FIGHTER_KIND_ROSETTA, *FIGHTER_KIND_PALUTENA, 
            *FIGHTER_KIND_REFLET, *FIGHTER_KIND_SHULK, *FIGHTER_KIND_RYU, *FIGHTER_KIND_KEN, *FIGHTER_KIND_CLOUD, *FIGHTER_KIND_KAMUI, *FIGHTER_KIND_BAYONETTA, 
            *FIGHTER_KIND_RIDLEY, *FIGHTER_KIND_SIMON, *FIGHTER_KIND_RICHTER, *FIGHTER_KIND_JACK, *FIGHTER_KIND_BRAVE, *FIGHTER_KIND_DOLLY, *FIGHTER_KIND_MASTER, *FIGHTER_KIND_EDGE,
            *FIGHTER_KIND_EFLAME, *FIGHTER_KIND_ELIGHT].contains(&fighter_kind) {
                max_offset = 5.0;
            }
            
        if status_kind == *FIGHTER_STATUS_KIND_ENTRY {
                max_offset = 0.0;
            }
            
        if (StatusModule::prev_status_kind(boma, 0) == *FIGHTER_STATUS_KIND_PASS) && MotionModule::frame(boma) < 10.0 {
                max_offset = 0.0;
            }
            
        if situation_kind == *SITUATION_KIND_AIR {// || status_kind == FIGHTER_STATUS_KIND_JUMP || status_kind == FIGHTER_STATUS_KIND_JUMP_AERIAL || status_kind == FIGHTER_STATUS_KIND_FALL || status_kind == FIGHTER_STATUS_KIND_FALL_AERIAL || status_kind == FIGHTER_STATUS_KIND_FALL_SPECIAL){
    
            if ECB_Y_OFFSETS[get_player_number(boma)] < max_offset {
                ECB_Y_OFFSETS[get_player_number(boma)] += 0.05;
            }
            else {
                ECB_Y_OFFSETS[get_player_number(boma)] = max_offset;
            }
                
            //ecb_y_offsets[WorkModule::get_int(boma, FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID)] = max_offset;
                
            offset.x = 0.0;
            offset.y = ECB_Y_OFFSETS[get_player_number(boma)];

            if !(vanilla_ecb || air_trans) {
                GroundModule::set_rhombus_offset(boma, &offset);
            }
        }
            
        else if situation_kind == *SITUATION_KIND_GROUND {
            max_offset = 0.0;
            //ecb_y_offsets[nx::utils::get_player_number(boma)] = max_offset;
                
            offset.x = 0.0;
            //offset.y = ecb_y_offsets[nx::utils::get_player_number(boma)];
            offset.y = 0.0;
            if !vanilla_ecb {
                GroundModule::set_rhombus_offset(boma, &offset);
            }
                
        }
            
        else{
            ECB_Y_OFFSETS[get_player_number(boma)] = 0.0;
            offset.x = 0.0;
            offset.y = ECB_Y_OFFSETS[get_player_number(boma)];
                
            if !vanilla_ecb {
                GroundModule::set_rhombus_offset(boma, &offset);
            }
        }
}

pub unsafe fn init_settings_edges(boma: &mut BattleObjectModuleAccessor, situation: smash::app::SituationKind, arg3: i32, arg4: u32,
    ground_cliff_check_kind: smash::app::GroundCliffCheckKind, arg6: bool,
    arg7: i32, arg8: i32, arg9: i32, arg10: i32) -> u32 {
    /* "fix" forces GroundModule::correct to be called for the statuses we need */
    let mut fix = arg4;
    let category = get_category(boma);
    let fighter_kind = get_kind(boma);
    let status_kind = StatusModule::status_kind(boma);
    let situation_kind = StatusModule::situation_kind(boma);

    if category == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        if [*FIGHTER_STATUS_KIND_WAIT, *FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_TURN, *FIGHTER_STATUS_KIND_TURN_DASH, *FIGHTER_STATUS_KIND_SQUAT, *FIGHTER_STATUS_KIND_SQUAT_WAIT, 
        *FIGHTER_STATUS_KIND_SQUAT_F, *FIGHTER_STATUS_KIND_SQUAT_B, *FIGHTER_STATUS_KIND_SQUAT_RV, *FIGHTER_STATUS_KIND_LANDING, *FIGHTER_STATUS_KIND_LANDING_LIGHT, 
        *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, *FIGHTER_STATUS_KIND_LANDING_DAMAGE_LIGHT, *FIGHTER_STATUS_KIND_GUARD_DAMAGE,
        *FIGHTER_STATUS_KIND_DAMAGE, *FIGHTER_STATUS_KIND_RUN, *FIGHTER_STATUS_KIND_TURN_RUN,
        *FIGHTER_STATUS_KIND_APPEAL, *FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE].contains(&status_kind) {
            fix = *GROUND_CORRECT_KIND_GROUND as u32;
        }
        /*if situation_kind == *SITUATION_KIND_GROUND {
            if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S { //Side b's
                if [*FIGHTER_KIND_INKLING, *FIGHTER_KIND_DIDDY].contains(&fighter_kind) {
                    fix = *GROUND_CORRECT_KIND_GROUND as u32;
                }
                if fighter_kind == *FIGHTER_KIND_JACK {
                    fix = *GROUND_CORRECT_KIND_GROUND as u32;
                }
            }
            if fighter_kind == *FIGHTER_KIND_JACK { // Joker gun edge cancels
                if [*FIGHTER_JACK_STATUS_KIND_SPECIAL_N_LANDING, *FIGHTER_JACK_STATUS_KIND_SPECIAL_N_BARRAGE_END].contains(&status_kind) {
                    fix = *GROUND_CORRECT_KIND_GROUND as u32;
                }
            }
            if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI { //Up specials
                if [*FIGHTER_KIND_PIKACHU, *FIGHTER_KIND_GEKKOUGA, *FIGHTER_KIND_INKLING].contains(&fighter_kind) {
                    fix = *GROUND_CORRECT_KIND_GROUND as u32;
                }
            }
            if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N { //Neutral Specials
                if [*FIGHTER_KIND_JACK, *FIGHTER_KIND_PLIZARDON].contains(&fighter_kind) {
                    fix = *GROUND_CORRECT_KIND_GROUND as u32;
                }
            }
            if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW { //Down Special edgecancels
                if [*FIGHTER_KIND_CAPTAIN, *FIGHTER_KIND_NESS, *FIGHTER_KIND_INKLING].contains(&fighter_kind) {
                    fix = *GROUND_CORRECT_KIND_GROUND as u32;
                }
            }
            if fighter_kind == *FIGHTER_KIND_CAPTAIN {
                if [*FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_LW_WALL_END].contains(&status_kind) {
                    fix = *GROUND_CORRECT_KIND_GROUND as u32;
                }
            }
            if fighter_kind == *FIGHTER_KIND_MIIFIGHTER {
                if [*FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW2_KICK, *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW2_KICK_LANDING].contains(&status_kind) {
                    fix = *GROUND_CORRECT_KIND_GROUND as u32;
                }
            }
        }*/
    }
    return fix;
}

#[skyline::hook(replace=GroundModule::correct)]
unsafe fn correct_hook(boma: &mut BattleObjectModuleAccessor, kind: GroundCorrectKind) -> u64 {
    let status_kind = StatusModule::status_kind(boma);
    let situation_kind = StatusModule::situation_kind(boma);
    let fighter_kind = get_kind(boma);
    let category = get_category(boma);

    if category == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        if [*FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_STATUS_KIND_LANDING, *FIGHTER_STATUS_KIND_TURN_DASH, *FIGHTER_STATUS_KIND_DASH,
        *FIGHTER_STATUS_KIND_TURN_RUN, *FIGHTER_STATUS_KIND_RUN, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, *FIGHTER_STATUS_KIND_WAIT].contains(&status_kind) {
            return original!()(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        }
        /*if situation_kind == *SITUATION_KIND_GROUND {
            if status_kind == *FIGHTER_STATUS_KIND_ATTACK_DASH {
                if [*FIGHTER_KIND_BUDDY, *FIGHTER_KIND_DONKEY, *FIGHTER_KIND_KIRBY].contains(&fighter_kind) {
                    return original!()(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
                }
            }
            if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S { //Side b's
                if [*FIGHTER_KIND_INKLING, *FIGHTER_KIND_LITTLEMAC, *FIGHTER_KIND_BAYONETTA, *FIGHTER_KIND_DIDDY, *FIGHTER_KIND_SHIZUE, *FIGHTER_KIND_MARTH, *FIGHTER_KIND_JACK, *FIGHTER_KIND_GANON].contains(&fighter_kind) {
                    return original!()(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
                }
            }
            if fighter_kind == *FIGHTER_KIND_JACK { // Joker gun edge cancels
                if [*FIGHTER_JACK_STATUS_KIND_SPECIAL_N_ESCAPE, *FIGHTER_JACK_STATUS_KIND_SPECIAL_N_LANDING, *FIGHTER_JACK_STATUS_KIND_SPECIAL_N_BARRAGE_END].contains(&status_kind) {
                    return original!()(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
                }
            }
            if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI { //Up specials
                if [*FIGHTER_KIND_CLOUD, *FIGHTER_KIND_PIKACHU, *FIGHTER_KIND_GEKKOUGA, *FIGHTER_KIND_INKLING, *FIGHTER_KIND_PICHU].contains(&fighter_kind) {
                    return original!()(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
                }
            }
            if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N { //Neutral Specials
                if [*FIGHTER_KIND_JACK, *FIGHTER_KIND_MARTH, *FIGHTER_KIND_PLIZARDON].contains(&fighter_kind) {
                    return original!()(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
                }
            }
            if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW { //Down Special edgecancels
                if [*FIGHTER_KIND_CAPTAIN, *FIGHTER_KIND_NESS, *FIGHTER_KIND_INKLING, *FIGHTER_KIND_GANON].contains(&fighter_kind) {
                    return original!()(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
                }
            }
            if fighter_kind == *FIGHTER_KIND_CAPTAIN {
                if [*FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_LW_WALL_END].contains(&status_kind) {
                    return original!()(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
                }
            }
        }*/
    }
    original!()(boma, kind)
}

#[skyline::hook(replace=StatusModule::init_settings)]
unsafe fn init_settings_hook(boma: &mut BattleObjectModuleAccessor, situation: smash::app::SituationKind, arg3: i32, arg4: u32,
                             ground_cliff_check_kind: smash::app::GroundCliffCheckKind, arg6: bool,
                             arg7: i32, arg8: i32, arg9: i32, arg10: i32) -> u64 {
    let category = get_category(boma);
    let fighter_kind = get_kind(boma);
    let status_kind = StatusModule::status_kind(boma);
    let situation_kind = StatusModule::situation_kind(boma);
    //
    // Call edge_slippoffs init_settings
    let fix = init_settings_edges(boma, situation, arg3, arg4, ground_cliff_check_kind, arg6, arg7, arg8, arg9, arg10);

    original!()(boma, situation, arg3, fix, ground_cliff_check_kind, arg6, arg7, arg8, arg9, arg10)
}

pub unsafe fn jumpCancelMove(boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, fighter_kind: i32, cat1: i32, stick_value_y: f32) { //Jump cancel grab, usmash, etc.
    if status_kind == *FIGHTER_STATUS_KIND_JUMP_SQUAT {
        if ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_GUARD) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ESCAPE_AIR, true);
        }
    }
}

#[skyline::hook(replace = smash::app::lua_bind::GroundModule::entry_cliff)]

pub unsafe fn entry_cliff_hook(boma: &mut smash::app::BattleObjectModuleAccessor) -> u64 {
    let entry_id = get_player_number(boma);
    LEDGE_POS[entry_id] = GroundModule::hang_cliff_pos_3f(boma);
    original!()(boma)
}

#[skyline::hook(replace = smash::app::lua_bind::GroundModule::leave_cliff)]

pub unsafe fn leave_cliff_hook(boma: &mut smash::app::BattleObjectModuleAccessor) -> u64 {
    let entry_id = get_player_number(boma);
    LEDGE_POS[entry_id] = smash::phx::Vector3f { x: 0.0, y: 0.0, z:0.0 };
    original!()(boma)
}

#[skyline::hook(replace = smash::app::lua_bind::GroundModule::can_entry_cliff)]

pub unsafe fn can_entry_cliff_hook(boma: &mut smash::app::BattleObjectModuleAccessor) -> u64 {
    let pos = GroundModule::hang_cliff_pos_3f(boma);
    let entry_id = get_player_number(boma);
    let status_kind = StatusModule::status_kind(boma);
    let fighter_kind = get_kind(boma);
    let motion_kind = MotionModule::motion_kind(boma);
    if fighter_kind == *FIGHTER_KIND_KOOPAJR {
        if KineticModule::get_sum_speed_y(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) > WorkModule::get_param_float(boma, hash40("air_speed_y_stable"), 0) {
            return 0;
        }
    }
    if KineticModule::get_sum_speed_y(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) > 0.0 { //Melee style sweetspots
        if ![*FIGHTER_KIND_PFUSHIGISOU, *FIGHTER_KIND_TANTAN, *FIGHTER_KIND_MASTER].contains(&fighter_kind) && status_kind != *FIGHTER_STATUS_KIND_AIR_LASSO && status_kind != 248 &&
        (fighter_kind != *FIGHTER_KIND_JACK || ![*FIGHTER_JACK_STATUS_KIND_SPECIAL_HI_CUT, *FIGHTER_JACK_STATUS_KIND_SPECIAL_HI_THROW, *FIGHTER_STATUS_KIND_SPECIAL_HI].contains(&status_kind)) &&
        (fighter_kind != *FIGHTER_KIND_SHIZUE || ![*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_SHIZUE_STATUS_KIND_SPECIAL_S_START, *FIGHTER_SHIZUE_STATUS_KIND_SPECIAL_S_THROW].contains(&status_kind)) &&
        (![*FIGHTER_KIND_SIMON, *FIGHTER_KIND_RICHTER].contains(&fighter_kind) || status_kind != *FIGHTER_STATUS_KIND_ATTACK_AIR)  {
            return 0 as u64;
        }
    }
    if (status_kind != *FIGHTER_STATUS_KIND_FALL_AERIAL && status_kind != *FIGHTER_STATUS_KIND_JUMP_AERIAL && status_kind != *FIGHTER_STATUS_KIND_FALL && 
    status_kind != *FIGHTER_STATUS_KIND_FLY && status_kind != *FIGHTER_STATUS_KIND_AIR_LASSO && ![*FIGHTER_KIND_PFUSHIGISOU, *FIGHTER_KIND_MASTER, *FIGHTER_KIND_TANTAN].contains(&fighter_kind) && (fighter_kind != *FIGHTER_KIND_JACK ||  
        ![*FIGHTER_JACK_STATUS_KIND_SPECIAL_HI_CUT, *FIGHTER_JACK_STATUS_KIND_SPECIAL_HI_THROW, *FIGHTER_STATUS_KIND_SPECIAL_HI].contains(&status_kind)) &&
        (![*FIGHTER_KIND_SIMON, *FIGHTER_KIND_RICHTER].contains(&fighter_kind) || status_kind != *FIGHTER_STATUS_KIND_ATTACK_AIR)) || motion_kind == 61345827621 { //Edgehog/trump
        for i in 0..9 {
            i as usize;
            if i == entry_id || LEDGE_POS[i].x == 0.0 {
                continue;
            }

            if pos.x == LEDGE_POS[i].x && pos.y == LEDGE_POS[i].y {
                return 0 as u64;
            }
        }
    }
    original!()(boma)
}

extern "C" {
    #[link_name = "\u{1}_ZN3app11FighterUtil30is_valid_just_shield_reflectorERNS_26BattleObjectModuleAccessorE"]
    fn is_valid_just_shield_reflector(boma: &mut smash::app::BattleObjectModuleAccessor) -> bool;
}

#[skyline::hook(replace=is_valid_just_shield_reflector)]
unsafe fn is_valid_just_shield_reflector_hook(boma: &mut smash::app::BattleObjectModuleAccessor) -> bool {
    return true;
}

#[skyline::hook(offset = 0x6bd6a4, inline)]
unsafe fn exec_command_reset_attack_air_kind_hook(ctx: &mut skyline::hooks::InlineCtx) {
    let control_module = *ctx.registers[21].x.as_ref();
    let boma = *(control_module as *mut *mut BattleObjectModuleAccessor).add(1);
    // For some reason, the game resets your attack_air_kind value every frame
    // even though it resets as soon as you perform an aerial attack
    // We don't want this to reset while in jumpsquat
    // to allow the game to use your initial C-stick input during jumpsquat for your attack_air_kind
    if !(*boma).is_status(*FIGHTER_STATUS_KIND_JUMP_SQUAT) {
        ControlModule::reset_attack_air_kind(boma);
    }
}

pub unsafe fn calcMomentum(boma: &mut smash::app::BattleObjectModuleAccessor) -> f32 {
    let jump_speed_x = WorkModule::get_param_float(boma, hash40("jump_speed_x"), 0);
    let jump_speed_x_mul = WorkModule::get_param_float(boma, hash40("jump_speed_x_mul"), 0);
    let stick_x = ControlModule::get_stick_x(boma);
    let x_vel = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let jump_speed_x_max = WorkModule::get_param_float(boma, hash40("jump_speed_x_max"), 0);
    let calcJumpSpeed = (jump_speed_x * stick_x) + (jump_speed_x_mul * x_vel);
    let mut jumpSpeedClamped = 0.0;
    if x_vel < 0.0 {
        jumpSpeedClamped = returnLarge(calcJumpSpeed, -1.0 * jump_speed_x_max);
    }
    else {
        jumpSpeedClamped = returnSmall(calcJumpSpeed, jump_speed_x_max);
    }
    jumpSpeedClamped
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_sub_attack_air_common)]
pub unsafe fn status_attack_air_hook(fighter: &mut L2CFighterCommon, param_1: L2CValue) {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let mut l2c_agent = L2CAgent::new(fighter.lua_state_agent);
    let is_speed_backward = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) * PostureModule::lr(boma) < 0.0;
    let prev_status_check = [*FIGHTER_STATUS_KIND_FALL, *FIGHTER_STATUS_KIND_JUMP, *FIGHTER_STATUS_KIND_JUMP_SQUAT].contains(&StatusModule::prev_status_kind(boma, 0));    
    let mut new_speed = CURRENTMOMENTUM[get_player_number(boma)];


        /*      Shorthop aerial macro and "bair stick flick" fix     */
    if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_FRAME_IN_AIR) <= 1 && 
        StatusModule::prev_status_kind(boma, 1) == *FIGHTER_STATUS_KIND_JUMP_SQUAT && !is_speed_backward { //if you used the shorthop aerial macro
        new_speed = calcMomentum(boma);
    }

    if prev_status_check {
        l2c_agent.clear_lua_stack();
        l2c_agent.push_lua_stack(&mut L2CValue::new_int(*FIGHTER_KINETIC_ENERGY_ID_CONTROL as u64));
        l2c_agent.push_lua_stack(&mut L2CValue::new_num(new_speed));
        smash::app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);
    }

    original!()(fighter, param_1)
}

pub unsafe fn additionalTransfer(lua_state: u64, l2c_agent: &mut L2CAgent, boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, fighter_kind: i32) {
    if [*FIGHTER_STATUS_KIND_JUMP_SQUAT, *FIGHTER_STATUS_KIND_JUMP, *FIGHTER_STATUS_KIND_FALL].contains(&status_kind) {
        CURRENTMOMENTUM[get_player_number(boma)] = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN); 
    }

            /*      ADDITIONAL MOVES THAT SHOULD CONSERVE MOMENTUM       */
    let mut should_conserve_momentum = false;
    
    if situation_kind == *SITUATION_KIND_AIR && MotionModule::frame(boma) <= 1.0 {

        if [*FIGHTER_KIND_CAPTAIN, *FIGHTER_KIND_MARIO, *FIGHTER_KIND_LUIGI, *FIGHTER_KIND_FOX, *FIGHTER_KIND_FALCO, *FIGHTER_KIND_WOLF, *FIGHTER_KIND_JACK, 
        *FIGHTER_KIND_CLOUD, *FIGHTER_KIND_EDGE, *FIGHTER_KIND_LINK, *FIGHTER_KIND_GEKKOUGA, *FIGHTER_KIND_PIKACHU, *FIGHTER_KIND_SZEROSUIT, *FIGHTER_KIND_DIDDY,
        *FIGHTER_KIND_ROY, *FIGHTER_KIND_MARTH, *FIGHTER_KIND_WARIO, *FIGHTER_KIND_SAMUS, *FIGHTER_KIND_SHEIK, *FIGHTER_KIND_REFLET, *FIGHTER_KIND_ELIGHT,
        *FIGHTER_KIND_LUCARIO, *FIGHTER_KIND_MEWTWO, *FIGHTER_KIND_KOOPA, *FIGHTER_KIND_RIDLEY, *FIGHTER_KIND_INKLING, *FIGHTER_KIND_SNAKE, *FIGHTER_KIND_KROOL]
            .contains(&fighter_kind) && status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N { //put any fighter here whose neutral special should conserve momentum
                should_conserve_momentum = true; 
        }

        if should_conserve_momentum && KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN).abs() > 0.1 {
            l2c_agent.clear_lua_stack();
            l2c_agent.push_lua_stack(&mut L2CValue::new_int(*FIGHTER_KINETIC_ENERGY_ID_CONTROL as u64));
            l2c_agent.push_lua_stack(&mut L2CValue::new_num(CURRENTMOMENTUM[get_player_number(boma)]));
            smash::app::sv_kinetic_energy::set_speed(lua_state);
        }

    }
}


#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_Jump_sub)]
pub unsafe fn status_jump_sub_hook(fighter: &mut L2CFighterCommon, param_2: L2CValue, param_3: L2CValue) -> L2CValue {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let mut l2c_agent = L2CAgent::new(fighter.lua_state_agent);

    l2c_agent.clear_lua_stack();
    l2c_agent.push_lua_stack(&mut L2CValue::new_int(*FIGHTER_KINETIC_ENERGY_ID_CONTROL as u64));
    l2c_agent.push_lua_stack(&mut L2CValue::new_num(calcMomentum(boma)));
    smash::app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);


    original!()(fighter, param_2, param_3)
}

#[skyline::hook(replace = KineticModule::change_kinetic)]
pub unsafe fn change_kinetic_hook(boma: &mut smash::app::BattleObjectModuleAccessor, kinetic_type: i32) -> Option<i32> { //spacie laser momentum conservation
    let mut kinetic_type_new = kinetic_type;
    let status_kind = StatusModule::status_kind(boma);
    let fighter_kind = get_kind(boma);
    let mut should_change_kinetic = false;

    if [*FIGHTER_KIND_FALCO, *FIGHTER_KIND_FOX, *FIGHTER_KIND_WOLF].contains(&fighter_kind) && status_kind == 446 /* laser status */ { 
        should_change_kinetic = true;
    }

    if [*FIGHTER_KINETIC_TYPE_FALL].contains(&kinetic_type_new) && should_change_kinetic {
        kinetic_type_new = -1;
    }     

    original!()(boma, kinetic_type_new)
}

pub unsafe fn regainAirDodge(boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32) {
    if situation_kind != *SITUATION_KIND_AIR {
        CANAIRDODGE[get_player_number(boma)] = true;
        CANWALLJUMP[get_player_number(boma)] = true;
    }
    if status_kind == *FIGHTER_STATUS_KIND_ESCAPE_AIR {
        CANAIRDODGE[get_player_number(boma)] = false;
    }

    if status_kind == *FIGHTER_STATUS_KIND_WALL_JUMP{
        CANWALLJUMP[get_player_number(boma)] = false;
    }
}

#[skyline::hook(replace = smash::app::lua_bind::WorkModule::is_enable_transition_term)]

pub unsafe fn is_enable_transition_term_hook(boma: &mut smash::app::BattleObjectModuleAccessor, flag: i32) -> bool {
    let status_kind = StatusModule::status_kind(boma);
    let fighter_kind = get_kind(boma);
   
    if flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR {
        if !CANAIRDODGE[get_player_number(boma)] {
                return false;
        }
    }

    original!()(boma, flag)
}

//Sub ftStatusUniqProcessGuardFunc_updateShield. Removes shield tilting
#[skyline::hook(replace = L2CFighterCommon_sub_ftStatusUniqProcessGuardFunc_updateShield)]
unsafe fn sub_ftstatusuniqprocessguardfunc_updateshield(fighter: &mut L2CFighterCommon, _param_1: L2CValue) {
    let shield_hp = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD);
    let scale = fighter.FighterStatusGuard__calc_shield_scale(shield_hp.into()).get_f32();
    ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("throw"), &Vector3f{x: scale, y: scale, z: scale});
}

//FighterStatusGuard set_shield_scale. Removes shield tilting, and makes shields no longer decrease in size
#[skyline::hook(replace = L2CFighterCommon_FighterStatusGuard__set_shield_scale)]
unsafe fn fighterstatusguard_set_shield_scale(fighter: &mut L2CFighterCommon, _param_1: L2CValue) -> L2CValue {
    ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("throw"), &Vector3f{x: 1.0, y: 1.0, z: 1.0});
    0.into()
}

//Effect Guard On Common, deals with Shield Effects
#[skyline::hook(replace = L2CFighterAnimcmdEffectCommon_effect_GuardOnCommon)]
unsafe fn effect_guardoncommon(fighter: &mut L2CFighterAnimcmdEffectCommon) -> L2CValue {
    let agent = &mut fighter.agent;
    agent.clear_lua_stack();
    is_excute(agent.lua_state_agent);
    let excute = agent.pop_lua_stack(1).get_bool();
    if excute {
        //Shield Smoke
        agent.clear_lua_stack();
        lua_args!(agent, Hash40::new("sys_shield_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, false);
        EFFECT_FLW_POS(agent.lua_state_agent);
        //Internal Shield, demonstrates shield health
        let color = {agent.clear_lua_stack(); lua_args!(agent, FT_VAR_INT_TEAM_COLOR); get_value_int(agent.lua_state_agent, *FT_VAR_INT_TEAM_COLOR)};
        let shield_hp = WorkModule::get_float(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD);
        let shield_max = WorkModule::get_float(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD_MAX);
        let ratio = (shield_hp/shield_max).clamp(0.1, 1.0);
        agent.clear_lua_stack();
        lua_args!(agent, Hash40::new("sys_shield"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 0.1*ratio, false, 0, color);
        EFFECT_FOLLOW_arg12(agent.lua_state_agent);
        lua_args!(agent, 0.6);
        LAST_EFFECT_SET_ALPHA(agent.lua_state_agent);
        //External Shield, prevents shield poking
        agent.clear_lua_stack();
        lua_args!(agent, Hash40::new("sys_shield"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 0.1, false, 0, color);
        EFFECT_FOLLOW_arg12(agent.lua_state_agent);
        agent.clear_lua_stack();
        lua_args!(agent, 0.2);
        LAST_EFFECT_SET_ALPHA(agent.lua_state_agent);
    }
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_sub_ftStatusUniqProcessGuardDamage_initStatus)]
unsafe fn sub_ftstatusuniqprocessguarddamage_initstatus(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_ftStatusUniqProcessGuardDamage_initStatus_Inner();
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_JUST_SHIELD) {
        fighter.FighterStatusGuard__set_just_shield_scale();
    }
    else {
        ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("throw"), &Vector3f{x: 1.0, y: 1.0, z: 1.0});
    }
    0.into()
}


#[skyline::hook(replace = L2CFighterCommon_status_GuardDamage_common)]
unsafe fn status_guarddamage_common(fighter: &mut L2CFighterCommon, param_1: L2CValue) {
    let fighter_kind = fighter.global_table[FIGHTER_KIND].get_i32();
    let module_accessor = fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut BattleObjectModuleAccessor;
    let shield_lr = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_GUARD_DAMAGE_WORK_FLOAT_SHIELD_LR);
    let just_shield_precede_extension = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("just_shield_precede_extension"));
    let just_shield_se = FighterUtil::get_just_shield_se(fighter_kind);
    let team_color = FighterUtil::get_team_color(module_accessor);
    let shield_effect_color = FighterUtil::get_effect_team_color(EColorKind(team_color as i32), Hash40::new("shield_effect_color"));
    let shield_hp = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD);
    let shield_max = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD_MAX);
    ControlModule::reset_flick_x(fighter.module_accessor);
    ControlModule::reset_flick_sub_x(fighter.module_accessor);
    fighter.global_table[STICK_X].assign(&0xFE.into());
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_JUST_SHIELD) {
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_STATUS_GUARD_ON_WORK_INT_MIN_FRAME);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD);
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_INSTANCE_WORK_ID_INT_DISABLE_GUARD_FRAME);
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_INSTANCE_WORK_ID_INT_DISABLE_ESCAPE_FRAME);
        HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_XLU), 0);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_HIT_XLU);
        ControlModule::set_command_life_extend(fighter.module_accessor, just_shield_precede_extension as u8);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x20cbc92683), 1, FIGHTER_LOG_DATA_INT_JUST_SHIELD);
        FighterUtil::flash_eye_info(module_accessor);
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL) {
            ModelModule::enable_gold_eye(fighter.module_accessor);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_GUARD_DAMAGE_WORK_FLAG_GOLD_EYE);
        }
        EffectModule::req_on_joint(fighter.module_accessor, Hash40::new("sys_just_shield"), Hash40::new("throw"), &NONE_VECTOR, &NONE_VECTOR, 1.0, &NONE_VECTOR, &NONE_VECTOR, false, *EFFECT_SUB_ATTRIBUTE_NONE as u32, *EFFECT_FLIP_NONE, 1);
        ColorBlendModule::set_last_attack_direction(fighter.module_accessor, &Vector3f{x: -shield_lr, y: 0.0, z: 0.0});
        EffectModule::req_common(fighter.module_accessor, Hash40::new("just_shield"), 0.0);
        if fighter.global_table[PREV_STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_GUARD_OFF {
            EffectModule::req_screen(fighter.module_accessor, Hash40::new("just_shield_screen"), false, false, false);
        }
    }
    else {
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD);
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_IS_DONE_GUARD_DAMAGE_NUM) {
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x20cbc92683), 1, FIGHTER_LOG_DATA_INT_GUARD_DAMAGE_NUM);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_IS_DONE_GUARD_DAMAGE_NUM);
        }
        if param_1.get_bool() {
            EffectModule::req_follow(fighter.module_accessor, Hash40::new_raw(0x12c9377e3d), Hash40::new("throw"), &NONE_VECTOR, &NONE_VECTOR, 0.1, false, *EFFECT_SUB_ATTRIBUTE_NONE as u32, 0, -1, *EFFECT_FLIP_NONE, 1, false, true);
            let handle = EffectModule::req_follow(fighter.module_accessor, Hash40::new_raw(0x12be304eab), Hash40::new("throw"), &NONE_VECTOR, &NONE_VECTOR, 0.1, false, *EFFECT_SUB_ATTRIBUTE_NONE as u32, 0, -1, *EFFECT_FLIP_NONE, 1, false, true);
            EffectModule::set_rgb_partial_last(fighter.module_accessor, shield_effect_color.x, shield_effect_color.y, shield_effect_color.z);
            WorkModule::set_int(fighter.module_accessor, handle as i32, *FIGHTER_STATUS_GUARD_ON_WORK_INT_SHIELD_DAMAGE2_EFFECT_HANDLE);
            let handle_1 = EffectModule::req_follow(fighter.module_accessor, Hash40::new_raw(0x113434cb66), Hash40::new("throw"), &NONE_VECTOR, &NONE_VECTOR, 1.0, false, *EFFECT_SUB_ATTRIBUTE_NONE as u32, 0, -1, *EFFECT_FLIP_NONE, 1, false, true);
            EffectModule::set_rgb_partial_last(fighter.module_accessor, shield_effect_color.x, shield_effect_color.y, shield_effect_color.z);
            WorkModule::set_int(fighter.module_accessor, handle_1 as i32, *FIGHTER_STATUS_GUARD_ON_WORK_INT_SHIELD_DAMAGE_EFFECT_HANDLE);
            if handle_1 != 0 {
                let diff = (shield_hp/shield_max).clamp(0.1, 1.0)*0.1;
                EffectModule::set_scale(fighter.module_accessor, handle_1 as u32, &Vector3f{x: diff, y: diff, z: diff});
            }
        }
    }
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.sub_GuardDamageUniq(false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(L2CFighterCommon_bind_address_call_sub_GuardDamageUniq as *const () as _));
}

pub unsafe extern "C" fn global_fighter_frame(fighter : &mut L2CFighterCommon) {
    JostleModule::set_team(fighter.module_accessor, 0);
    let lua_state = fighter.lua_state_agent;
    let mut l2c_agent = L2CAgent::new(lua_state);
    let module_accessor = &mut *fighter.module_accessor;
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let fighter_kind = get_kind(module_accessor);
    let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);
    let cat2 = ControlModule::get_command_flag_cat(fighter.module_accessor, 1);
    let cat3 = ControlModule::get_command_flag_cat(fighter.module_accessor, 2);
    let stick_value_y = ControlModule::get_stick_y(fighter.module_accessor);
    let stick_value_x = ControlModule::get_stick_x(fighter.module_accessor);
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let curr_frame = MotionModule::frame(fighter.module_accessor);

    //Held Buffer
    let control_pad = [
        *CONTROL_PAD_BUTTON_APPEAL_HI, *CONTROL_PAD_BUTTON_APPEAL_LW, *CONTROL_PAD_BUTTON_APPEAL_S_L, *CONTROL_PAD_BUTTON_APPEAL_S_R, *CONTROL_PAD_BUTTON_ATTACK, *CONTROL_PAD_BUTTON_ATTACK_RAW, *CONTROL_PAD_BUTTON_CATCH, *CONTROL_PAD_BUTTON_CSTICK_ON,
        *CONTROL_PAD_BUTTON_FLICK_JUMP, *CONTROL_PAD_BUTTON_GUARD, *CONTROL_PAD_BUTTON_GUARD_HOLD, *CONTROL_PAD_BUTTON_INVALID, *CONTROL_PAD_BUTTON_JUMP, *CONTROL_PAD_BUTTON_JUMP_MINI, *CONTROL_PAD_BUTTON_SMASH, *CONTROL_PAD_BUTTON_SPECIAL, 
        *CONTROL_PAD_BUTTON_SPECIAL_RAW, *CONTROL_PAD_BUTTON_SPECIAL_RAW2, *CONTROL_PAD_BUTTON_STOCK_SHARE, *CONTROL_PAD_BUTTON_TERM, *CONTROL_PAD_CLATTER_CAUSE_NONE, *CONTROL_PAD_CLATTER_FLOWER, *CONTROL_PAD_CLATTER_MAIN, *CONTROL_PAD_CLATTER_NONE,
        *CONTROL_PAD_CLATTER_TERM, *CONTROL_PAD_STICK_REVERSE_ALL, *CONTROL_PAD_STICK_REVERSE_NONE, *CONTROL_PAD_STICK_REVERSE_X, *CONTROL_PAD_STICK_REVERSE_Y
    ];
    for i in control_pad {
        if ControlModule::get_trigger_count(module_accessor, i as u8) > 15 && ControlModule::check_button_on(module_accessor, i)
        && ![*FIGHTER_STATUS_KIND_GUARD, *FIGHTER_STATUS_KIND_GUARD_ON, *FIGHTER_STATUS_KIND_GUARD_DAMAGE, *FIGHTER_STATUS_KIND_GUARD_OFF, *FIGHTER_STATUS_KIND_JUMP_SQUAT].contains(&status_kind) {
            ControlModule::reset_trigger(module_accessor);
            ControlModule::clear_command(module_accessor, true);
        }
    }

    shieldStops(module_accessor, status_kind, cat1);
    shieldDrops(module_accessor, status_kind, cat2);
    dashPlatformDrop(module_accessor, status_kind, situation_kind, stick_value_y, stick_value_x);
    grabRefreshJumps(module_accessor, status_kind, situation_kind);
    pivots(module_accessor, status_kind, curr_frame, stick_value_x);
    fixbackdash(module_accessor, status_kind, motion_kind, cat1, stick_value_y);
    lagCanceled(module_accessor, status_kind);
    meleeECBs(module_accessor, status_kind, situation_kind, fighter_kind);
    jumpCancelMove(module_accessor, status_kind, fighter_kind, cat1, stick_value_y);
    additionalTransfer(lua_state, &mut l2c_agent, module_accessor, status_kind, situation_kind, fighter_kind);
    regainAirDodge(module_accessor, status_kind, situation_kind);
}

fn nro_main(nro: &skyline::nro::NroInfo) {
    match nro.name {
        "common" => {
            skyline::install_hooks!(
                status_jump_sub_hook,
                status_attack_air_hook,
                sub_ftstatusuniqprocessguardfunc_updateshield,
                fighterstatusguard_set_shield_scale,
                effect_guardoncommon,
                sub_ftstatusuniqprocessguarddamage_initstatus,
                status_guarddamage_common
            );
        }
        _ => (),
    }
}

pub fn install() {
    // Removes 10f C-stick lockout for tilt stick and special stick
    skyline::patching::Patch::in_text(0x17527dc).data(0x2A1F03FA);
    skyline::patching::Patch::in_text(0x17527e0).nop();
    skyline::patching::Patch::in_text(0x17527e4).nop();
    skyline::patching::Patch::in_text(0x17527e8).nop();

    // Prevents buffered C-stick aerials from triggering nair
    skyline::patching::Patch::in_text(0x6be644).data(0x52800040);

    // Prevents attack_air_kind from resetting every frame
    // Found in ControlModule::exec_command
    skyline::patching::Patch::in_text(0x6bd6a4).nop();

    skyline::install_hook!(get_param_float_hook);
    skyline::install_hook!(init_settings_hook);
    skyline::install_hook!(correct_hook);
    skyline::install_hook!(entry_cliff_hook);
    skyline::install_hook!(leave_cliff_hook);
    skyline::install_hook!(can_entry_cliff_hook);
    skyline::install_hook!(is_valid_just_shield_reflector_hook);
    skyline::install_hook!(exec_command_reset_attack_air_kind_hook); 
    skyline::install_hook!(change_kinetic_hook);
    skyline::install_hook!(is_enable_transition_term_hook);
    jumpsquat::install();
    jabs::install();
    dashattack::install();
    nro::add_hook(nro_main).unwrap();
}