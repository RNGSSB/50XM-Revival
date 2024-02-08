from pyprc import *

fighter_param = param("fighter_param_vanilla.prc")
table = fighter_param[hash("fighter_param_table")]

mods = {
    hash("fighter_kind_fox"): {
        hash("dash_speed"): 2.4,
        hash("air_accel_y"): 0.23,
        hash("air_speed_y_stable"): 2.8,
        hash("damage_fly_top_speed_y_stable"): 2.8,
        hash("damage_fly_top_air_accel_y"): 0.23,
        hash("dive_speed_y"): 3.4,
        hash("landing_attack_air_frame_lw"): 9,
        hash("landing_attack_air_frame_f"): 8,
        hash("landing_attack_air_frame_hi"): 9,
    },
    hash("fighter_kind_mario"): {
        hash("landing_attack_air_frame_lw"): 13,
    },
    hash("fighter_kind_sheik"): {
        hash("landing_attack_air_frame_lw"): 10,
    },
    hash("fighter_kind_elight"): {
       hash("dash_speed"): 2.25,
       hash("weight"): 82,
       hash("landing_attack_air_frame_n"): 9,
    },
    hash("fighter_kind_cloud"): {
       hash("landing_attack_air_frame_f"): 13,
       hash("landing_attack_air_frame_b"): 11,
    },
    hash("fighter_kind_captain"): {
       hash("landing_attack_air_frame_f"): 9,
    },
    hash("fighter_kind_gaogaen"): {
       hash("run_speed_max"): 1.18,
       hash("jump_squat_frame"): 4,
       hash("landing_attack_air_frame_n"): 8,
       hash("landing_attack_air_frame_lw"): 13,
    },
    hash("fighter_kind_edge"): {
       hash("jump_squat_frame"): 4,
       hash("landing_attack_air_frame_f"): 9,
       hash("landing_attack_air_frame_b"): 11,
       hash("landing_attack_air_frame_hi"): 16,
       hash("landing_attack_air_frame_lw"): 21, 
       hash("attack_combo_max"): 1,
    },
    hash("fighter_kind_samus"): {
       hash("landing_attack_air_frame_b"): 9,
       hash("landing_attack_air_frame_hi"): 10,
    },
}


# actual param traversal and editing
for fighter in table:
    # the returned value is a hash, not a string
    fighter_name = fighter[hash("fighter_kind")].value
    
    gravity = fighter[hash("air_accel_y")]
    fallspeed = fighter[hash("air_speed_y_stable")]
    fastfallspeed = fighter[hash("dive_speed_y")]
    traction = fighter[hash("ground_brake")]
    jumpspeed = fighter[hash("jump_initial_y")]
    shorthop = fighter[hash("mini_jump_y")]
    runspeed = fighter[hash("run_speed_max")]
    airspeed = fighter[hash("air_speed_x_stable")]
    
    gravity = gravity.value * 1.2
    fallspeed = fallspeed.value * 1.4
    fastfallspeed = fastfallspeed.value * 1.1
    traction = traction.value * 0.6
    jumpspeed = jumpspeed.value * 0.65
    shorthop = shorthop.value * 0.8
    runspeed = runspeed.value * 0.91
    airspeed = airspeed.value * 0.952
    
    fighter[hash("air_accel_y")].value = gravity
    fighter[hash("air_speed_y_stable")].value = fallspeed
    fighter[hash("damage_fly_top_air_accel_y")].value = gravity
    fighter[hash("damage_fly_top_speed_y_stable")].value = fallspeed
    fighter[hash("dive_speed_y")].value = fastfallspeed
    fighter[hash("ground_brake")].value = traction
    fighter[hash("jump_initial_y")].value = jumpspeed
    fighter[hash("mini_jump_y")].value = shorthop
    fighter[hash("run_speed_max")].value = runspeed
    fighter[hash("air_speed_x_stable")].value = airspeed
    fighter[hash("wall_jump_type")].value = True
    
    
    if fighter_name in mods:
        fighter_mods = mods[fighter_name]
        for key in fighter_mods:
            fighter[key].value = fighter_mods[key]

fighter_param.save("fighter_param.prc")

