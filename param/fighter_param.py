from pyprc import *

fighter_param = param("fighter_param_vanilla.prc")
table = fighter_param[hash("fighter_param_table")]

mods = {
    hash("fighter_kind_gaogaen"): {
        hash("run_speed_max"): 1.45,
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
    
    gravity = gravity.value * 1.2
    fallspeed = fallspeed.value * 1.4
    fastfallspeed = fastfallspeed.value * 1.1
    traction = traction.value * 0.6
    jumpspeed = jumpspeed.value * 0.65
    shorthop = shorthop.value * 0.8
    
    fighter[hash("air_accel_y")].value = gravity
    fighter[hash("air_speed_y_stable")].value = fallspeed
    fighter[hash("damage_fly_top_air_accel_y")].value = gravity
    fighter[hash("damage_fly_top_speed_y_stable")].value = fallspeed
    fighter[hash("dive_speed_y")].value = fastfallspeed
    fighter[hash("ground_brake")].value = traction
    fighter[hash("jump_initial_y")].value = jumpspeed
    fighter[hash("mini_jump_y")].value = shorthop
    fighter[hash("wall_jump_type")].value = True
    
    
    if fighter_name in mods:
        fighter_mods = mods[fighter_name]
        for key in fighter_mods:
            fighter[key].value = fighter_mods[key]

fighter_param.save("fighter_param.prc")

